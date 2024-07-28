use reqwest::{Client, Response};
use serde::{Deserialize, Serialize};

use crate::api::utils::{create_request, log_error, UtilsError};
use crate::config::Config;

/// Represents detailed information about the user's subscription, including limits
/// and permissions associated with the current subscription tier.
#[derive(Serialize, Deserialize, Debug)]
pub struct SubscriptionInfo {
    pub tier: String,
    pub character_count: i32,
    pub character_limit: i32,
    pub can_extend_character_limit: bool,
    pub allowed_to_extend_character_limit: bool,
    pub next_character_count_reset_unix: i64,
    pub voice_limit: i32,
    pub max_voice_add_edits: i32,
    pub voice_add_edit_counter: i32,
    pub professional_voice_limit: i32,
    pub can_extend_voice_limit: bool,
    pub can_use_instant_voice_cloning: bool,
    pub can_use_professional_voice_cloning: bool,
    pub currency: String,
    pub status: String,
    pub billing_period: String,
    pub next_invoice: NextInvoiceDetails,
    pub has_open_invoices: bool,
}

/// Contains details regarding the next invoice within the user's subscription.
#[derive(Serialize, Deserialize, Debug)]
pub struct NextInvoiceDetails {
    pub amount_due_cents: i32,
    pub next_payment_attempt_unix: i64,
}

/// Represents information about a user, including subscription details and
/// user-specific properties like new user status and onboarding completion.#[derive(Serialize, Deserialize, Debug)]
pub struct UserInfo {
    pub subscription: SubscriptionInfo,
    pub is_new_user: bool,
    pub xi_api_key: String,
    pub can_use_delayed_payment_methods: bool,
    pub is_onboarding_completed: bool,
    pub first_name: Option<String>,
}

/// Provides functionality for interacting with the ElevenLabs User API endpoints.
/// Allows retrieval of user information and subscription details.
pub struct UserClient {
    client: Client,
    config: Config,
}

impl UserClient {
    /// Creates a new `UserClient` with the provided configuration.
    pub fn new(config: Config) -> Self {
        UserClient {
            client: Client::new(),
            config,
        }
    }
    
    /// Fetches detailed information about the user from the ElevenLabs API.
    ///
    /// # Returns
    ///
    /// A `Result` type that, on success, contains `UserInfo` representing the user's details,
    /// or `UtilsError` on failure.
    ///
    /// # Examples
    ///
    /// ```
    /// let user_client = UserClient::new(config);
    /// let user_info = user_client.get_user_info().await.unwrap();
    /// println!("User's first name: {}", user_info.first_name);
    /// ```
    pub async fn get_user_info(&self) -> Result<UserInfo, UtilsError> {
        let url = format!("{}/v1/user", &self.config.api_url);
        let response = self.send_request(url).await?;

        response.json::<UserInfo>().await.map_err(UtilsError::Http)
    }

    /// Retrieves the user's subscription information from the ElevenLabs API.
    ///
    /// # Returns
    ///
    /// A `Result` type that, on success, contains `SubscriptionInfo` detailing the subscription,
    /// or `UtilsError` on failure.
    ///
    /// # Examples
    ///
    /// ```
    /// let user_client = UserClient::new(config);
    /// let subscription_info = user_client.get_user_subscription_info().await.unwrap();
    /// println!("Subscription tier: {}", subscription_info.tier);
    /// ```
    pub async fn get_user_subscription_info(&self) -> Result<SubscriptionInfo, UtilsError> {
        let url = format!("{}/v1/user/subscription", &self.config.api_url);
        let response = self.send_request(url).await?;

        response.json::<SubscriptionInfo>().await.map_err(UtilsError::Http)
    }

    /// Sends a GET request to the provided URL and returns the HTTP response.
    ///
    /// This internal method sets up the request with the necessary headers and error handling.
    ///
    /// # Arguments
    ///
    /// * `url` - A `String` specifying the full URL to which the request will be sent.
    ///
    /// # Returns
    ///
    /// A `Result` type that, on success, contains the `Response` object, or `UtilsError` on failure.
    async fn send_request(&self, url: String) -> Result<Response, UtilsError> {
        let response = create_request(&self.client, reqwest::Method::GET, &url)
            .header("xi-api-key", &self.config.api_key)
            .send()
            .await
            .map_err(UtilsError::Http)?;

        if response.status().is_success() {
            Ok(response)
        } else {
            let error_msg = format!("🚨 Failed to send request: HTTP {}", response.status());
            log_error(&error_msg);
            Err(UtilsError::Custom(error_msg))
        }
    }
}
