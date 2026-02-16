//! # Lycento SDK for Rust
//!
//! A comprehensive Rust SDK for the Lycento licensing platform, designed for
//! integration with Tauri desktop applications.
//!
//! ## Features
//!
//! - License activation and deactivation
//! - License validation
//! - Device information management
//! - Automatic device ID generation
//! - Full async/await support
//!
//! ## Quick Start
//!
//! ```rust
//! use lycento_sdk::{LycentoClient, LycentoConfig};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let config = LycentoConfig::new("https://api.lycento.com")
//!         .with_api_key("your-api-key")
//!         .with_timeout(10000);
//!
//!     let client = LycentoClient::new(config)?;
//!
//!     // Activate a license
//!     let response = client.activate_license("LICENSE-KEY-HERE").await?;
//!     println!("Activated: {}", response.success);
//!
//!     // Validate a license
//!     let is_valid = client.validate_license("LICENSE-KEY-HERE").await?;
//!     println!("Valid: {}", is_valid.valid);
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Tauri Integration
//!
//! This SDK is designed to work seamlessly with Tauri applications. You can
//! expose the client functionality to your frontend using Tauri commands.
//!
//! ## Modules
//!
//! - [`client`] - Main client for license operations
//! - [`device`] - Device information and identification
//! - [`errors`] - Error types

// Re-export public API
pub use crate::client::{
    create_client, validate_license, ActivateOptions, ActivateResponse, DeactivateOptions,
    DeactivateResponse, LicenseInfo, LicenseInfoResponse, LycentoClient, LycentoConfig,
    ValidateOptions, ValidateResponse,
};

pub use crate::device::{
    get_device_id, get_device_info, get_device_name, get_platform, get_platform_version,
    hash_string, simple_hash, DeviceInfo, Platform,
};

pub use crate::client::{ActivationDetails, ActivationRecord};

pub use crate::errors::{
    ActivationError, DeactivationError, LycentoError, NetworkError, ValidationError,
};

mod client;
mod device;
mod errors;

// Re-export version info
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
