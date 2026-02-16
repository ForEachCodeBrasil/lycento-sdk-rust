# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] - 2025-02-16

### Added

- Initial release
- License activation and deactivation
- License validation
- Device information management
- Automatic device ID generation
- Full async/await support with tokio
- Error handling with specific error types
- Comprehensive documentation

### Features

- `LycentoClient` - Main client for license operations
- `LycentoConfig` - Client configuration builder
- `validate_license()` - Validate license keys
- `activate_license()` - Activate license on device
- `deactivate_license()` - Deactivate device from license
- `get_license_info()` - Get license information
- `get_device_id()` - Get unique device identifier
- `get_device_info()` - Get device information object

### Supported Platforms

- Windows
- macOS
- Linux

### Dependencies

- reqwest (HTTP client)
- tokio (async runtime)
- serde/serde_json (serialization)
- thiserror/anyhow (error handling)
- sha2/hex (device ID hashing)
- chrono (date/time)
