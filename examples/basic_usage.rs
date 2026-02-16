//! Basic usage example for the Lycento SDK.
//!
//! Run with: cargo run --example basic-usage
//!
//! Or with custom configuration:
//! LYCENTO_BASE_URL=https://api.lycento.com LYCENTO_API_KEY=your-key cargo run --example basic-usage

use lycento_sdk::{
    ActivateOptions, DeactivateOptions, DeviceInfo, LycentoClient, LycentoConfig, Platform,
    ValidateOptions,
};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load .env file
    dotenvy::dotenv().ok();

    // Initialize logging
    env_logger::init();

    // Get configuration from environment or use defaults
    let base_url = env::var("LYCENTO_BASE_URL").unwrap_or_else(|_| "http://localhost:8000".to_string());
    let api_key = env::var("LYCENTO_API_KEY").ok();

    println!("=== Lycento SDK Basic Usage Example ===\n");

    // Create client configuration
    let mut config = LycentoConfig::new(&base_url).with_timeout(10000);
    if let Some(key) = api_key {
        config = config.with_api_key(key);
    }

    // Create the client
    let client = LycentoClient::new(config)?;
    println!("Client created successfully\n");

    // Get device info
    let device_info = DeviceInfo::default();
    println!("Device Information:");
    println!("  Device ID: {}", device_info.device_id);
    println!("  Device Name: {}", device_info.device_name);
    println!("  Platform: {}", device_info.platform.as_str());
    println!("  Architecture: {}", device_info.architecture);
    println!();

    // Example license key (replace with a real one for testing)
    let license_key = env::var("LYCENTO_LICENSE_KEY").unwrap_or_else(|_| "TEST-ABC123-DEF456".to_string());

    // 1. Activate a license
    println!("1. Activating license: {}", license_key);
    match client
        .activate(
            ActivateOptions::new(&license_key)
                .with_platform(device_info.platform.clone()),
        )
        .await
    {
        Ok(response) => {
            println!("   ✓ Activation successful!");
            println!("     License Status: {}", response.license.status);
            println!("     License Type: {}", response.license.license_type);
            println!("     Max Devices: {}", response.license.max_devices);
            println!(
                "     Expires: {}",
                response.license.expires_at.unwrap_or_else(|| "Never".to_string())
            );
            println!("     Activation ID: {}", response.activation.id);
        }
        Err(e) => {
            println!("   ✗ Activation failed: {}", e);
        }
    }
    println!();

    // 2. Validate the license
    println!("2. Validating license: {}", license_key);
    match client.validate_license(&license_key).await {
        Ok(response) => {
            println!("   ✓ Validation result: {}", response.valid);
            println!("     License Status: {}", response.license.status);
            if let Some(activation) = response.activation {
                println!("     Device: {}", activation.device_name);
                println!(
                    "     Last Validated: {}",
                    activation.last_validated_at.unwrap_or_else(|| "N/A".to_string())
                );
            }
        }
        Err(e) => {
            println!("   ✗ Validation failed: {}", e);
        }
    }
    println!();

    // 3. Quick validation check
    println!("3. Quick validation check (is_valid):");
    let is_valid = client.is_valid(&license_key).await;
    println!("   ✓ License is valid: {}", is_valid);
    println!();

    // 4. Get license info
    println!("4. Getting license information:");
    match client.get_info(&license_key).await {
        Ok(info) => {
            println!("   ✓ License Info retrieved");
            println!("     Status: {}", info.license.status);
            println!(
                "     Active Devices: {}/{}",
                info.license.active_devices.unwrap_or(0),
                info.license.max_devices
            );
            println!("     Total Activations: {}", info.activations.len());
            for (i, activation) in info.activations.iter().enumerate().take(3) {
                println!(
                    "     - Device {}: {} ({})",
                    i + 1,
                    activation.device_name,
                    if activation.is_active { "active" } else { "inactive" }
                );
            }
        }
        Err(e) => {
            println!("   ✗ Failed to get info: {}", e);
        }
    }
    println!();

    // 5. Check if can activate more devices
    println!("5. Checking device slot availability:");
    let can_activate = client.can_activate(&license_key).await?;
    println!("   ✓ Can activate more devices: {}", can_activate);
    println!();

    // 6. Deactivate (optional - commented out by default)
    // println!("6. Deactivating license...");
    // match client.deactivate_current(&license_key).await {
    //     Ok(response) => {
    //         println!("   ✓ Deactivation successful: {}", response.message);
    //     }
    //     Err(e) => {
    //         println!("   ✗ Deactivation failed: {}", e);
    //     }
    // }

    println!("=== Example completed ===");
    Ok(())
}
