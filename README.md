# lycento-sdk

Official Lycento Rust SDK for software licensing.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
lycento-sdk = "1.0"
```

## Quick Start

```rust
use lycento_sdk::{LycentoClient, LycentoConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create client
    let config = LycentoConfig::new("https://lycento.test")
        .with_api_key("your-api-key")
        .with_timeout(10000);

    let client = LycentoClient::new(config)?;

    // Validate a license
    let result = client.validate_license("YOUR-LICENSE-KEY").await?;
    println!("Valid: {}", result.valid);

    // Activate a license
    let activation = client.activate_license("YOUR-LICENSE-KEY").await?;
    println!("Activated: {}", activation.success);

    // Deactivate a license
    client.deactivate_license("YOUR-LICENSE-KEY", &activation.device_id).await?;

    Ok(())
}
```

## API Reference

### `LycentoConfig`

Configure the client instance.

```rust
let config = LycentoConfig::new("https://lycento.test")
    .with_api_key("your-api-key")
    .with_timeout(10000); // milliseconds
```

### `LycentoClient`

Main client for license operations.

```rust
let client = LycentoClient::new(config)?;
```

### `validate_license`

Validate a license key.

```rust
let result = client.validate_license("LICENSE-KEY").await?;
// ValidateResponse { valid: bool, license: Option<LicenseInfo>, ... }
```

### `activate_license`

Activate a license on the current device.

```rust
let options = ActivateOptions {
    device_name: Some("My Device".to_string()),
    metadata: Some(serde_json::json!({"version": "1.0.0"})),
};
let result = client.activate_license_with_options("LICENSE-KEY", options).await?;
```

### `deactivate_license`

Deactivate a device from a license.

```rust
client.deactivate_license("LICENSE-KEY", &device_id).await?;
```

### `get_license_info`

Get license information.

```rust
let info = client.get_license_info("LICENSE-KEY").await?;
```

## Device Identification

The SDK automatically generates a unique device ID based on system information.

```rust
use lycento_sdk::{get_device_id, get_device_info, get_platform};

let device_id = get_device_id();
let device_info = get_device_info();
let platform = get_platform();
```

## Error Handling

```rust
use lycento_sdk::{LycentoError, ActivationError, ValidationError};

match client.validate_license("LICENSE-KEY").await {
    Ok(result) => println!("Valid: {}", result.valid),
    Err(LycentoError::Validation(e)) => eprintln!("Validation error: {}", e),
    Err(LycentoError::Activation(e)) => eprintln!("Activation error: {}", e),
    Err(LycentoError::Network(e)) => eprintln!("Network error: {}", e),
    Err(e) => eprintln!("Error: {}", e),
}
```

## Tauri Integration

This SDK is designed to work seamlessly with Tauri applications.

```rust
// In your Tauri app
use lycento_sdk::LycentoClient;

#[tauri::command]
async fn validate_license(license_key: String) -> Result<bool, String> {
    let client = LycentoClient::new(LycentoConfig::new("https://lycento.test")
        .with_api_key("your-api-key"))
        .map_err(|e| e.to_string())?;

    let result = client.validate_license(&license_key)
        .await
        .map_err(|e| e.to_string())?;

    Ok(result.valid)
}
```

## Features

- `default` - Uses native TLS
- `rustls-tls` - Uses rustls for TLS

```toml
[dependencies]
lycento-sdk = { version = "1.0", default-features = false, features = ["rustls-tls"] }
```

## Platform Support

- Windows
- macOS
- Linux

## Documentation

- [API Documentation](https://docs.rs/lycento-sdk)
- [Examples](./examples/)

## License

MIT © Lycento
