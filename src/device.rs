//! Device information module for the Lycento SDK.
//!
//! This module provides functionality to gather device information for license activation,
//! including device ID generation, platform detection, and device name resolution.

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use sys_info;

/// Supported platforms for license activation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Platform {
    #[serde(rename = "windows")]
    Windows,
    #[serde(rename = "macos")]
    Macos,
    #[serde(rename = "linux")]
    Linux,
    #[serde(rename = "android")]
    Android,
    #[serde(rename = "ios")]
    Ios,
    #[serde(rename = "unknown")]
    Unknown,
}

impl Platform {
    /// Convert platform to string representation.
    pub fn as_str(&self) -> &'static str {
        match self {
            Platform::Windows => "windows",
            Platform::Macos => "macos",
            Platform::Linux => "linux",
            Platform::Android => "android",
            Platform::Ios => "ios",
            Platform::Unknown => "unknown",
        }
    }
}

impl Default for Platform {
    fn default() -> Self {
        Self::Unknown
    }
}

/// Device information structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceInfo {
    /// Unique device identifier.
    pub device_id: String,
    /// Human-readable device name.
    pub device_name: String,
    /// Current platform.
    pub platform: Platform,
    /// Platform version/release.
    pub platform_version: String,
    /// System architecture.
    pub architecture: String,
}

impl Default for DeviceInfo {
    fn default() -> Self {
        Self {
            device_id: get_device_id(),
            device_name: get_device_name(),
            platform: get_platform(),
            platform_version: get_platform_version(),
            architecture: get_architecture(),
        }
    }
}

/// Cached device ID for performance.
static CACHED_DEVICE_ID: Lazy<String> = Lazy::new(generate_device_id);

/// Get the cached device ID.
pub fn get_device_id() -> String {
    CACHED_DEVICE_ID.clone()
}

/// Generate a deterministic device ID from machine characteristics.
///
/// Uses multiple system identifiers to create a unique but consistent
/// device identifier that persists across restarts.
pub fn generate_device_id() -> String {
    let mut hasher = Sha256::new();

    // Include multiple system identifiers for uniqueness
    let hostname = sys_info::hostname().unwrap_or_default();
    let os_type = sys_info::os_type().unwrap_or_default();
    let os_release = sys_info::os_release().unwrap_or_default();

    // Combine all identifiers
    let combined = format!("{}-{}-{}", hostname, os_type, os_release);
    hasher.update(combined.as_bytes());

    // Take first 32 characters of hex hash
    let result = hasher.finalize();
    hex::encode(result)[..32].to_string()
}

/// Get the current device information.
///
/// This function gathers platform, architecture, and other system information
/// to create a complete device profile for license activation.
pub fn get_device_info() -> DeviceInfo {
    DeviceInfo {
        device_id: get_device_id(),
        device_name: get_device_name(),
        platform: get_platform(),
        platform_version: get_platform_version(),
        architecture: get_architecture(),
    }
}

/// Detect the current platform.
pub fn get_platform() -> Platform {
    let os_type = sys_info::os_type().unwrap_or_default().to_lowercase();

    if os_type.contains("windows") {
        Platform::Windows
    } else if os_type.contains("darwin") || os_type.contains("macos") {
        Platform::Macos
    } else if os_type.contains("linux") {
        // Check for Android via os_release
        let os_release = sys_info::os_release().unwrap_or_default().to_lowercase();
        if os_release.contains("android") {
            Platform::Android
        } else {
            Platform::Linux
        }
    } else if os_type.contains("ios") {
        Platform::Ios
    } else if os_type.contains("android") {
        Platform::Android
    } else {
        Platform::Unknown
    }
}

/// Get a human-readable device name.
pub fn get_device_name() -> String {
    // Try hostname first
    if let Ok(hostname) = sys_info::hostname() {
        if !hostname.is_empty() {
            // On Windows, prepend username if available
            #[cfg(target_os = "windows")]
            {
                if let Ok(info) = sys_info::info() {
                    let username = info.username.unwrap_or_default();
                    if !username.is_empty() && username != "Unknown" {
                        return format!("{}-{}", username, hostname);
                    }
                }
            }
            return hostname;
        }
    }

    // Fallback to a generated name based on device ID
    let device_id = get_device_id();
    format!("device-{}", &device_id[..8])
}

/// Get the platform version/release.
pub fn get_platform_version() -> String {
    sys_info::os_release().unwrap_or_else(|_| "unknown".to_string())
}

/// Get the system architecture.
pub fn get_architecture() -> String {
    // sys_info doesn't have arch, so we detect it from the OS type
    #[cfg(target_arch = "x86_64")]
    return "x86_64".to_string();
    #[cfg(target_arch = "aarch64")]
    return "aarch64".to_string();
    #[cfg(target_arch = "x86")]
    return "x86".to_string();
    #[cfg(target_arch = "arm")]
    return "arm".to_string();
    #[cfg(not(any(
        target_arch = "x86_64",
        target_arch = "aarch64",
        target_arch = "x86",
        target_arch = "arm"
    )))]
    return "unknown".to_string();
}

/// Hash a string using SHA256.
pub fn hash_string(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    hex::encode(hasher.finalize())
}

/// Generate a simple hash for non-cryptographic purposes.
pub fn simple_hash(input: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    input.hash(&mut hasher);
    hasher.finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_platform_detection() {
        let platform = get_platform();
        assert!(matches!(
            platform,
            Platform::Windows | Platform::Macos | Platform::Linux | Platform::Unknown
        ));
    }

    #[test]
    fn test_device_id_consistency() {
        let id1 = get_device_id();
        let id2 = get_device_id();
        assert_eq!(id1, id2);
    }

    #[test]
    fn test_device_info() {
        let info = get_device_info();
        assert!(!info.device_id.is_empty());
        assert!(!info.device_name.is_empty());
    }

    #[test]
    fn test_hash_string() {
        let hash = hash_string("test-input");
        assert_eq!(hash.len(), 64);
    }
}
