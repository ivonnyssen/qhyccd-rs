# qhyccd-rs

[![Crates.io](https://img.shields.io/crates/v/qhyccd-rs.svg)](https://crates.io/crates/qhyccd-rs)
[![Documentation](https://docs.rs/qhyccd-rs/badge.svg)](https://docs.rs/qhyccd-rs/)
[![Codecov](https://codecov.io/github/ivonnyssen/qhyccd-rs/coverage.svg?branch=main)](https://codecov.io/gh/ivonnyssen/qhyccd-rs)
[![Dependency status](https://deps.rs/repo/github/ivonnyssen/qhyccd-rs/status.svg)](https://deps.rs/repo/github/ivonnyssen/qhyccd-rs)

libqhyccd bindings for Rust.

Current bindings are not complete, but will grow as functionality is needed for the ASCOM alpaca drivers or other projects. It is very early in the dev cycle and I am still learning a ton about Rust, so things might be in flux for a while.

```toml
[dependencies]
qhyccd-rs = "0.1.8"
```

## Rust version requirements

qhyccd-rs works with stable Rust. The minimum required Rust version is 1.68.

## Version of libqhyccd

Currently the library works with  [QHYCCD SDK 25.09.29](https://www.qhyccd.com/html/prepub/log_en.html#!log_en.md#25.09.29). The library supports Linux, Windows, and macOS (experimental).

## Platform Support

- **Linux**: Fully supported (x86_64 and aarch64)
- **Windows**: Fully supported
- **macOS**: Experimental support (both Intel and Apple Silicon)

## License

This project is licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   <https://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or
   <https://opensource.org/licenses/MIT>)

at your option.

The repository contains files from the QHYCCD SDK, these are not covered by these licenses and only provided, so builds in CI pass reasonably.

### Contribution

All contributions are welcome.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in qhyccd-rs by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

## Quick Start

### Linux

#### Install libusb-1.0-dev

##### Debian / Ubuntu

```sh
sudo apt-get install libusb-1.0-0-dev
```

##### Fedora

```sh
sudo dnf install libusb1-devel
```

#### Install QHYCCD SDK

##### ARM

```sh
wget https://www.qhyccd.com/file/repository/publish/SDK/25.09.29/sdk_Arm64_25.09.29.tgz
tar xzvf sdk_Arm64_25.09.29.tgz 
cd sdk_Arm64_25.09.29/
sudo sh install.sh 
```

##### Linux_64

```sh
wget https://www.qhyccd.com/file/repository/publish/SDK/25.09.29/sdk_linux64_25.09.29.tgz
tar xzvf sdk_linux64_25.09.29.tgz
cd sdk_linux64_25.09.29/
sudo sh install.sh 
```

### Windows

Download and install the QHYCCD SDK from the [official website](https://www.qhyccd.com/html/prepub/log_en.html#!log_en.md#25.09.29).

### macOS (Experimental)

Download and install the QHYCCD SDK from the [official website](https://www.qhyccd.com/html/prepub/log_en.html#!log_en.md#25.09.29).

## Usage Examples

[src/bin/LiveFrameMode.rs](https://github.com/ivonnyssen/qhyccd-rs/blob/main/src/bin/LiveFrameMode.rs)

[src/bin/SingleFrameMode.rs](https://github.com/ivonnyssen/qhyccd-rs/blob/main/src/bin/SingleFrameMode.rs)

## Simulation Feature

The `simulation` feature enables testing and development without physical hardware. When enabled, `Sdk::new()` automatically provides a simulated camera environment with realistic behavior.

### Enable Simulation

Add the feature to your `Cargo.toml`:

```toml
[dependencies]
qhyccd-rs = { version = "0.1.8", features = ["simulation"] }
```

Or use it for development dependencies:

```toml
[dev-dependencies]
qhyccd-rs = { version = "0.1.8", features = ["simulation"] }
```

### Transparent Usage

With the simulation feature enabled, your code works identically whether using real or simulated hardware:

```rust
use qhyccd_rs::Sdk;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Same code for both real and simulated cameras
    let sdk = Sdk::new()?;

    let cameras = sdk.cameras();
    println!("Found {} camera(s)", cameras.count());

    if let Some(camera) = cameras.first() {
        println!("Camera: {}", camera.id()?);

        // Check for filter wheel
        let filter_wheels = sdk.filter_wheels();
        if let Some(fw) = filter_wheels.first() {
            fw.open()?;
            println!("Filter wheel with {} positions", fw.get_number_of_filters()?);
            fw.close()?;
        }
    }

    Ok(())
}
```

### Default Simulated Camera

When using the simulation feature, `Sdk::new()` automatically provides:

- **Camera**: QHY178M-Simulated (`SIM-QHY178M`)
  - 3072x2048 resolution
  - 16-bit depth
  - Cooler support for temperature control
  - Multiple readout modes
  - Gain, offset, and exposure controls

- **Filter Wheel**: 7-position CFW
  - Accessible via `sdk.filter_wheels()`
  - Full control API support

### Custom Simulated Cameras

For advanced use cases requiring custom camera configurations, use `Sdk::new_simulated()` and `add_simulated_camera()`:

```rust
use qhyccd_rs::{Sdk, simulation::SimulatedCameraConfig};

let mut sdk = Sdk::new_simulated();

// Add custom camera with specific configuration
let config = SimulatedCameraConfig::default()
    .with_id("CUSTOM-CAM-001")
    .with_model("Custom Camera Model")
    .with_filter_wheel(5)  // 5-position wheel
    .with_cooler();

sdk.add_simulated_camera(config);
```

### Building with Simulation

```bash
# Build with simulation
cargo build --features simulation

# Run tests with simulation
cargo test --features simulation

# Run examples with simulation
cargo run --features simulation --bin SingleFrameMode
```
