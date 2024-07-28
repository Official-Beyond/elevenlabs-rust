use reqwest::{Client, Error as ReqwestError};
use serde::Deserialize;
// use my own crate's config
use crate::config_loader::Config;
use crate::utils::{create_request, UtilsError};

/// Response structure for metadata about a specific voice.
#[derive(Deserialize, Debug)]
pub struct VoiceMetadata {
    // Define the fields based on the API's response format for a voice.
    // Example fields; adjust according to actual API response.
    pub voice_id: String,
    // ... other metadata fields ...
    // If with_settings is true, there might be additional settings fields.
}

/// Represents the voice settings returned by the API.
#[derive(Deserialize, Debug)]
pub struct VoiceSettings {
    pub stability: f32,
    pub similarity_boost: f32,
    pub style: Option<f32>,
    pub use_speaker_boost: Option<bool>,
}

/// Client for interacting with the ElevenLabs Voices API.
pub struct VoicesClient {
    client: Client,
    config: Config,
}

impl VoicesClient {
    /// Creates a new `VoicesClient` using the given configuration.
    ///
    /// # Arguments
    ///
    /// * `config` - A `Config` instance containing the necessary configuration.
    pub fn new(config: Config) -> Self {
        VoicesClient {
            client: Client::new(),
            config,
        }
    }

    /// Fetches metadata about a specific voice from the ElevenLabs API.
    ///
    /// # Arguments
    ///
    /// * `voice_id` - The ID of the voice to fetch metadata for.
    /// * `with_settings` - Whether to include settings information for the voice.
    ///
    /// # Returns
    ///
    /// A `Result` which on success contains `VoiceMetadata`, or `UtilsError` on failure.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Config::new("your_api_key", "https://api.elevenlabs.io");
    /// let voices_client = VoicesClient::new(config);
    /// let metadata = voices_client.get_voice_metadata("voice_id_example", true).await?;
    /// println!("Voice Metadata: {:?}", metadata);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_voice_metadata(&self, voice_id: &str, with_settings: bool) -> Result<VoiceMetadata, UtilsError> {
        let url = format!("{}/v1/voices/{}", &self.config.api_url, voice_id);

        let response = create_request(&self.client, reqwest::Method::GET, &url)
            .header("xi-api-key", &self.config.api_key)
            .query(&[("with_settings", with_settings)])
            .send()
            .await
            .map_err(UtilsError::Http)?;

        if response.status().is_success() {
            response.json::<VoiceMetadata>().await.map_err(UtilsError::Http)
        } else {
            let error_msg = format!("🚨 Failed to get voice metadata: HTTP {}", response.status());
            Err(UtilsError::Custom(error_msg))
        }
    }

    /// Deletes a voice by its ID.
    ///
    /// # Arguments
    ///
    /// * `voice_id` - The ID of the voice to delete.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or error.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Config::new("your_api_key", "https://api.elevenlabs.io");
    /// let voices_client = VoicesClient::new(config);
    /// voices_client.delete_voice("voice_id_example").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete_voice(&self, voice_id: &str) -> Result<(), UtilsError> {
        let url = format!("{}/v1/voices/{}", &self.config.api_url, voice_id);

        let response = create_request(&self.client, reqwest::Method::DELETE, &url)
            .header("xi-api-key", &self.config.api_key)
            .send()
            .await
            .map_err(UtilsError::Http)?;

        if response.status().is_success() {
            Ok(())
        } else {
            let error_msg = format!("🚨 Failed to delete voice: HTTP {}", response.status());
            Err(UtilsError::Custom(error_msg))
        }
    }
    
    /// Adds a new voice to the collection of voices in VoiceLab.
    ///
    /// # Arguments
    ///
    /// * `name` - The name that identifies the voice.
    /// * `files` - One or more audio files to clone the voice from.
    /// * `description` - A description for the voice.
    /// * `labels` - Serialized labels dictionary for the voice.
    ///
    /// # Returns
    ///
    /// A `Result` that, on success, contains the voice ID of the added voice(s),
    /// or `UtilsError` on failure.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Config::new("your_api_key", "https://api.elevenlabs.io");
    /// let voices_client = VoicesClient::new(config);
    /// let files = vec!["path_to_audio_file_1", "path_to_audio_file_2"];
    /// let voice_id = voices_client.add_voice("New Voice Name", files, Some("Description"), None).await?;
    /// println!("Added voice ID: {}", voice_id);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn add_voice(
        &self,
        name: &str,
        files: Vec<&str>, // Assuming paths to files
        description: Option<&str>,
        labels: Option<&str>,
    ) -> Result<String, UtilsError> {
        let url = format!("{}/v1/voices/add", &self.config.api_url);

        let mut form = multipart::Form::new().text("name", name.to_string());

        for file_path in files {
            let file = multipart::Part::file(file_path)?;
            form = form.part("files", file);
        }

        if let Some(description) = description {
            form = form.text("description", description.to_string());
        }

        if let Some(labels) = labels {
            form = form.text("labels", labels.to_string());
        }

        let response = self.client.post(url)
            .header("xi-api-key", &self.config.api_key)
            .multipart(form)
            .send()
            .await
            .map_err(UtilsError::Http)?;

        if response.status().is_success() {
            let voice_id = response.text().await.map_err(UtilsError::Http)?;
            Ok(voice_id)
        } else {
            let error_msg = format!("Failed to add voice: HTTP {}", response.status());
            Err(UtilsError::Custom(error_msg))
        }
    }

    /// Edits an existing voice.
    ///
    /// # Arguments
    ///
    /// * `voice_id` - The ID of the voice to edit.
    /// * `name` - The new name of the voice.
    /// * `files` - Audio files to add to the voice.
    /// * `description` - New description for the voice.
    /// * `labels` - Serialized labels dictionary for the voice.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or error.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Config::new("your_api_key", "https://api.elevenlabs.io");
    /// let voices_client = VoicesClient::new(config);
    /// voices_client.edit_voice("voice_id_example", "New Voice Name", vec!["path_to_audio_file"], Some("New Description"), None).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn edit_voice(
        &self,
        voice_id: &str,
        name: &str,
        files: Vec<&str>, // Assuming paths to files
        description: Option<&str>,
        labels: Option<&str>,
    ) -> Result<(), UtilsError> {
        let url = format!("{}/v1/voices/{}/edit", &self.config.api_url, voice_id);

        let mut form = multipart::Form::new().text("name", name.to_string());

        for file_path in files {
            let file = multipart::Part::file(file_path)?;
            form = form.part("files", file);
        }

        if let Some(description) = description {
            form = form.text("description", description.to_string());
        }

        if let Some(labels) = labels {
            form = form.text("labels", labels.to_string());
        }

        let response = self.client.post(url)
            .header("xi-api-key", &self.config.api_key)
            .multipart(form)
            .send()
            .await
            .map_err(UtilsError::Http)?;

        if response.status().is_success() {
            Ok(())
        } else {
            let error_msg = format!("Failed to edit voice: HTTP {}", response.status());
            Err(UtilsError::Custom(error_msg))
        }
    }

        /// Edits the settings for a specific voice.
    ///
    /// # Arguments
    ///
    /// * `voice_id` - The ID of the voice whose settings are to be edited.
    /// * `settings` - The `VoiceSettings` struct containing the new settings.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or error.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Config::new("your_api_key", "https://api.elevenlabs.io");
    /// let voices_client = VoicesClient::new(config);
    /// let settings = VoiceSettings {
    ///     stability: 0.5,
    ///     similarity_boost: 0.5,
    ///     style: None,
    ///     use_speaker_boost: Some(true),
    /// };
    /// voices_client.edit_voice_settings("voice_id_example", settings).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn edit_voice_settings(
        &self,
        voice_id: &str,
        settings: VoiceSettings,
    ) -> Result<(), UtilsError> {
        let url = format!("{}/v1/voices/{}/settings/edit", &self.config.api_url, voice_id);

        let response = self.client.post(url)
            .header("Content-Type", "application/json")
            .header("xi-api-key", &self.config.api_key)
            .json(&settings)
            .send()
            .await
            .map_err(UtilsError::Http)?;

        if response.status().is_success() {
            Ok(())
        } else {
            let error_msg = format!("Failed to edit voice settings: HTTP {}", response.status());
            Err(UtilsError::Custom(error_msg))
        }
    }
}
