use reqwest::{Client, header};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::config::Config;
// use crate::utils::{UtilsError};

/// Settings for customizing the voice output.
#[derive(Serialize, Deserialize, Debug)]
pub struct VoiceSettings {
    pub stability: i32,
    pub similarity_boost: i32,
    pub style: Option<f32>,
    pub use_speaker_boost: Option<bool>,
    // Include additional settings as per the API.
}

/// A list of pronunciation dictionary locators to be applied to the text.
/// They will be applied in order. Up to 3 locators per request.
#[derive(Serialize, Deserialize, Debug)]
pub struct PronunciationDictionaryLocator {
    pub pronunciation_dictionary_id: String,
    pub version_id: String,
}

/// The request payload for the TTS API.
#[derive(Serialize, Deserialize, Debug)]
pub struct TtsRequest {
    pub text: String,
    pub model_id: Option<String>,
    pub voice_settings: Option<VoiceSettings>,
    pub pronunciation_dictionary_locators: Option<Vec<PronunciationDictionaryLocator>>,
}

/// Client for interacting with the ElevenLabs Text-to-Speech API.
pub struct TextToSpeechClient {
    client: Client,
    config: Config,
}

impl TextToSpeechClient {
    /// Creates a new `TextToSpeechClient` using the given configuration.
    ///
    /// # Arguments
    ///
    /// * `config` - A `Config` instance containing the necessary configuration.
    pub fn new(config: Config) -> Self {
        TextToSpeechClient {
            client: Client::new(),
            config,
        }
    }

    /// Converts text to speech using the specified voice and settings.
    ///
    /// # Arguments
    ///
    /// * `voice_id` - The ID of the voice model to use for synthesis.
    /// * `request` - The `TtsRequest` containing the text and other parameters for synthesis.
    ///
    /// # Returns
    ///
    /// A `Result` which, on success, contains the synthesized speech as a byte array,
    /// or `UtilsError` on failure.
    ///
    /// # Examples
    ///
    /// ```
    /// let tts_client = TextToSpeechClient::new(config);
    /// let request = TtsRequest {
    ///     text: "Hello, world!".to_string(),
    ///     model_id: Some("default_model_id".to_string()),
    ///     voice_settings: Some(VoiceSettings {
    ///         stability: Some(0.5),
    ///         similarity_boost: Some(0.5),
    ///         // Other settings...
    ///     }),
    ///     // Other fields...
    /// };
    /// let response = tts_client.synthesize("voice_id", &request).await.unwrap();
    /// ```
    pub async fn synthesize(&self, voice_id: &str, request: &TtsRequest) -> Result<Vec<u8>, UtilsError> {
        let url = format!("{}/v1/text-to-speech/{}", &self.config.api_url, voice_id);

        let mut req_headers = header::HeaderMap::new();
        req_headers.insert("Accept", header::HeaderValue::from_static("audio/mpeg"));
        req_headers.insert("Content-Type", header::HeaderValue::from_static("application/json"));
        req_headers.insert("xi-api-key", header::HeaderValue::from_str(&self.config.api_key)?);

        let response = self.client.post(url)
            .headers(req_headers)
            .json(&request)
            .send()
            .await
            .map_err(UtilsError::Http)?;

        match response.error_for_status_ref() {
            Ok(_) => {
                let bytes = response.bytes().await.map_err(UtilsError::Http)?;
                Ok(bytes.to_vec())
            },
            Err(e) => {
                let error_msg = format!("🚨 Failed to synthesize text: {}", e);
                Err(UtilsError::Custom(error_msg))
            }
        }
    }
}
