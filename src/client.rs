//! Main client module for the Lycento SDK.
//!
//! This module provides the `LycentoClient` struct and all related types
//! for interacting with the Lycento licensing API.

use crate::device::{get_device_id, get_device_info, DeviceInfo, Platform};
use crate::errors::{
    ActivationError, DeactivationError, LycentoError, NetworkError, ValidationError,
};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Configuration for the Lycento client.
#[derive(Debug, Clone)]
pub struct LycentoConfig {
    /// Base URL of the Lycento API.
    pub base_url: String,
    /// Optional API key for authentication.
    pub api_key: Option<String>,
    /// Request timeout in milliseconds.
    pub timeout: Option<u64>,
}

impl LycentoConfig {
    /// Create a new configuration.
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            api_key: None,
            timeout: None,
        }
    }

    /// Set the API key.
    pub fn with_api_key(mut self, api_key: impl Into<String>) -> Self {
        self.api_key = Some(api_key.into());
        self
    }

    /// Set the timeout in milliseconds.
    pub fn with_timeout(mut self, timeout: u64) -> Self {
        self.timeout = Some(timeout);
        self
    }
}

/// Options for license activation.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivateOptions {
    /// The license key to activate.
    pub license_key: String,
    /// Optional custom device ID.
    pub device_id: Option<String>,
    /// Optional custom device name.
    pub device_name: Option<String>,
    /// Optional custom platform.
    pub device_platform: Option<String>,
    /// Optional IP address.
    pub ip_address: Option<String>,
}

impl ActivateOptions {
    /// Create new activation options.
    pub fn new(license_key: impl Into<String>) -> Self {
        Self {
            license_key: license_key.into(),
            device_id: None,
            device_name: None,
            device_platform: None,
            ip_address: None,
        }
    }

    /// Set a custom device ID.
    pub fn with_device_id(mut self, device_id: impl Into<String>) -> Self {
        self.device_id = Some(device_id.into());
        self
    }

    /// Set a custom device name.
    pub fn with_device_name(mut self, device_name: impl Into<String>) -> Self {
        self.device_name = Some(device_name.into());
        self
    }

    /// Set a custom platform.
    pub fn with_platform(mut self, platform: Platform) -> Self {
        self.device_platform = Some(platform.as_str().to_string());
        self
    }

    /// Set IP address.
    pub fn with_ip_address(mut self, ip_address: impl Into<String>) -> Self {
        self.ip_address = Some(ip_address.into());
        self
    }
}

/// Options for license validation.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidateOptions {
    /// The license key to validate.
    pub license_key: String,
    /// Optional device ID to check.
    pub device_id: Option<String>,
}

impl ValidateOptions {
    /// Create new validation options.
    pub fn new(license_key: impl Into<String>) -> Self {
        Self {
            license_key: license_key.into(),
            device_id: None,
        }
    }

    /// Set device ID for validation.
    pub fn with_device_id(mut self, device_id: impl Into<String>) -> Self {
        self.device_id = Some(device_id.into());
        self
    }
}

/// Options for license deactivation.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeactivateOptions {
    /// The license key to deactivate.
    pub license_key: String,
    /// The device ID to deactivate.
    pub device_id: String,
}

impl DeactivateOptions {
    /// Create new deactivation options.
    pub fn new(license_key: impl Into<String>, device_id: impl Into<String>) -> Self {
        Self {
            license_key: license_key.into(),
            device_id: device_id.into(),
        }
    }
}

/// Response from license activation.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivateResponse {
    /// Whether activation was successful.
    pub success: bool,
    /// License information.
    pub license: LicenseInfo,
    /// Activation details.
    pub activation: ActivationDetails,
}

/// Response from license validation.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidateResponse {
    /// Whether the license is valid.
    pub valid: bool,
    /// License information.
    pub license: LicenseInfo,
    /// Activation details (if device-specific validation).
    pub activation: Option<ActivationDetails>,
}

/// Response from license deactivation.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeactivateResponse {
    /// Whether deactivation was successful.
    pub success: bool,
    /// Response message.
    pub message: String,
    /// Deactivation details.
    pub activation: DeactivationDetails,
}

/// License information structure.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LicenseInfo {
    /// License key.
    pub key: String,
    /// License status (active, expired, revoked, etc.).
    pub status: String,
    /// License type (perpetual, subscription, etc.).
    #[serde(rename = "type")]
    pub license_type: String,
    /// Expiration date (null for perpetual).
    pub expires_at: Option<String>,
    /// Maximum allowed devices.
    pub max_devices: u32,
    /// Number of active devices (for info endpoint).
    #[serde(default)]
    pub active_devices: Option<u32>,
}

/// Activation details.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivationDetails {
    /// Activation ID.
    pub id: u32,
    /// Device ID.
    pub device_id: String,
    /// Device name.
    pub device_name: String,
    /// Device platform.
    pub device_platform: String,
    /// When the license was activated.
    pub activated_at: String,
    /// Last validation time (for validation response).
    #[serde(default)]
    pub last_validated_at: Option<String>,
}

/// Deactivation details.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeactivationDetails {
    /// Activation ID.
    pub id: u32,
    /// Device ID.
    pub device_id: String,
    /// When the license was deactivated.
    pub deactivated_at: String,
}

/// License information with all activations.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LicenseInfoResponse {
    /// License details.
    pub license: LicenseInfo,
    /// All activations (active and inactive).
    pub activations: Vec<ActivationRecord>,
}

/// An activation record.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivationRecord {
    /// Activation ID.
    pub id: u32,
    /// Device ID.
    pub device_id: String,
    /// Device name.
    pub device_name: String,
    /// Device platform.
    pub device_platform: String,
    /// When activated.
    pub activated_at: String,
    /// When deactivated (null if still active).
    pub deactivated_at: Option<String>,
    /// Whether currently active.
    pub is_active: bool,
}

/// The main Lycento client for license operations.
#[derive(Debug)]
pub struct LycentoClient {
    client: Client,
    base_url: String,
    api_key: Option<String>,
}

impl LycentoClient {
    /// Create a new Lycento client with the given configuration.
    pub fn new(config: LycentoConfig) -> Result<Self, LycentoError> {
        let timeout = config.timeout.unwrap_or(10000);

        let client = Client::builder()
            .timeout(Duration::from_millis(timeout))
            .build()
            .map_err(|e| LycentoError::new(e.to_string()))?;

        // Remove trailing slash from base URL
        let base_url = config.base_url.trim_end_matches('/').to_string();

        Ok(Self {
            client,
            base_url,
            api_key: config.api_key,
        })
    }

    /// Create a new Lycento client with a builder-like interface.
    pub fn create(base_url: impl Into<String>) -> Result<Self, LycentoError> {
        Self::new(LycentoConfig::new(base_url))
    }

    /// Activate a license on the current device.
    pub async fn activate(&self, options: ActivateOptions) -> Result<ActivateResponse, ActivationError> {
        let device_info = get_device_info();

        let payload = serde_json::json!({
            "license_key": options.license_key,
            "device_id": options.device_id.unwrap_or(device_info.device_id),
            "device_name": options.device_name.unwrap_or(device_info.device_name),
            "device_platform": options.device_platform.unwrap_or_else(|| device_info.platform.as_str().to_string()),
            "ip_address": options.ip_address,
        });

        let response = self
            .post("/api/v1/licenses/activate", payload)
            .await
            .map_err(ActivationError::from)?;

        self.handle_activation_response(response, "activation").await
    }

    /// Activate with a license key string (uses default device).
    pub async fn activate_license(&self, license_key: &str) -> Result<ActivateResponse, ActivationError> {
        self.activate(ActivateOptions::new(license_key)).await
    }

    /// Validate a license.
    pub async fn validate(&self, options: ValidateOptions) -> Result<ValidateResponse, ValidationError> {
        let device_id = options.device_id.unwrap_or_else(get_device_id);

        let payload = serde_json::json!({
            "license_key": options.license_key,
            "device_id": device_id,
        });

        let response = self
            .post("/api/v1/licenses/validate", payload)
            .await
            .map_err(ValidationError::from)?;

        self.handle_validation_response(response).await
    }

    /// Validate a license key (uses default device).
    pub async fn validate_license(&self, license_key: &str) -> Result<ValidateResponse, ValidationError> {
        self.validate(ValidateOptions::new(license_key)).await
    }

    /// Quick check if a license is valid.
    pub async fn is_valid(&self, license_key: &str) -> bool {
        self.validate_license(license_key)
            .await
            .map(|r| r.valid)
            .unwrap_or(false)
    }

    /// Deactivate a license on a specific device.
    pub async fn deactivate(&self, options: DeactivateOptions) -> Result<DeactivateResponse, DeactivationError> {
        let payload = serde_json::json!({
            "license_key": options.license_key,
            "device_id": options.device_id,
        });

        let response = self
            .post("/api/v1/licenses/deactivate", payload)
            .await
            .map_err(DeactivationError::from)?;

        self.handle_deactivation_response(response).await
    }

    /// Deactivate the current device.
    pub async fn deactivate_current(&self, license_key: &str) -> Result<DeactivateResponse, DeactivationError> {
        let device_id = get_device_id();
        self.deactivate(DeactivateOptions::new(license_key, device_id)).await
    }

    /// Get license information including all activations.
    pub async fn get_info(&self, license_key: &str) -> Result<LicenseInfoResponse, LycentoError> {
        let response = self
            .client
            .get(&format!("{}/api/v1/licenses/info", self.base_url))
            .query(&[("license_key", license_key)])
            .send()
            .await?;

        let status = response.status();
        let json: serde_json::Value = response.json().await?;

        if status.is_success() {
            serde_json::from_value(json).map_err(|e| LycentoError::Custom(e.to_string()))
        } else {
            Err(self.handle_error_response(status, &json))
        }
    }

    /// Get the number of active devices for a license.
    pub async fn get_active_device_count(&self, license_key: &str) -> Result<u32, LycentoError> {
        let info = self.get_info(license_key).await?;
        Ok(info.license.active_devices.unwrap_or(0))
    }

    /// Check if a license can activate another device.
    pub async fn can_activate(&self, license_key: &str) -> Result<bool, LycentoError> {
        let info = self.get_info(license_key).await?;
        let active = info.license.active_devices.unwrap_or(0);
        Ok(active < info.license.max_devices)
    }

    // Private helper methods

    async fn post(&self, endpoint: &str, payload: serde_json::Value) -> Result<serde_json::Value, LycentoError> {
        let url = format!("{}{}", self.base_url, endpoint);

        let mut request = self.client.post(&url).json(&payload);

        if let Some(ref api_key) = self.api_key {
            request = request.header("Authorization", format!("Bearer {}", api_key));
        }

        let response = request
            .send()
            .await
            .map_err(|e| self.handle_network_error(e))?;

        self.handle_response(response).await
    }

    async fn handle_response(&self, response: reqwest::Response) -> Result<serde_json::Value, LycentoError> {
        let status = response.status();
        let json: serde_json::Value = response.json().await.map_err(LycentoError::from)?;

        if status.is_success() {
            Ok(json)
        } else {
            Err(self.handle_error_response(status, &json))
        }
    }

    async fn handle_activation_response(
        &self,
        json: serde_json::Value,
        _context: &str,
    ) -> Result<ActivateResponse, ActivationError> {
        // Check for error in response
        if let Some(error) = json.get("error").and_then(|e| e.as_str()) {
            return Err(ActivationError::new(error));
        }

        if let Some(success) = json.get("success").and_then(|s| s.as_bool()) {
            if !success {
                let error = json
                    .get("error")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Activation failed");
                return Err(ActivationError::new(error));
            }
        }

        serde_json::from_value(json).map_err(|e| ActivationError::new(e.to_string()))
    }

    async fn handle_validation_response(&self, json: serde_json::Value) -> Result<ValidateResponse, ValidationError> {
        // Check for error in response
        if let Some(error) = json.get("error").and_then(|e| e.as_str()) {
            return Err(ValidationError::new(error));
        }

        serde_json::from_value(json).map_err(|e| ValidationError::new(e.to_string()))
    }

    async fn handle_deactivation_response(&self, json: serde_json::Value) -> Result<DeactivateResponse, DeactivationError> {
        // Check for error in response
        if let Some(error) = json.get("error").and_then(|e| e.as_str()) {
            return Err(DeactivationError::new(error));
        }

        if let Some(success) = json.get("success").and_then(|s| s.as_bool()) {
            if !success {
                let error = json
                    .get("error")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Deactivation failed");
                return Err(DeactivationError::new(error));
            }
        }

        serde_json::from_value(json).map_err(|e| DeactivationError::new(e.to_string()))
    }

    fn handle_network_error(&self, error: reqwest::Error) -> LycentoError {
        if error.is_connect() || error.is_timeout() || error.is_request() {
            LycentoError::new("Network error - please check your connection")
        } else {
            LycentoError::new(error.to_string())
        }
    }

    fn handle_error_response(&self, status: reqwest::StatusCode, json: &serde_json::Value) -> LycentoError {
        let error_message = json
            .get("error")
            .or_else(|| json.get("message"))
            .and_then(|m| m.as_str())
            .unwrap_or("Unknown error");

        match status.as_u16() {
            404 => LycentoError::new("License not found"),
            422 => LycentoError::new(error_message),
            429 => LycentoError::new("Rate limit exceeded - please try again later"),
            _ => LycentoError::new(format!("Server error: {} - {}", status, error_message)),
        }
    }
}

/// Create a new Lycento client with the given configuration.
pub fn create_client(config: LycentoConfig) -> Result<LycentoClient, LycentoError> {
    LycentoClient::new(config)
}

/// Quick validation helper - validates with auto-detected device.
pub async fn validate_license(
    license_key: &str,
    base_url: &str,
    api_key: Option<&str>,
) -> Result<bool, LycentoError> {
    let mut config = LycentoConfig::new(base_url);
    if let Some(key) = api_key {
        config = config.with_api_key(key);
    }

    let client = LycentoClient::new(config)?;
    Ok(client.is_valid(license_key).await)
}
