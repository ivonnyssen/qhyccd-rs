//! # QHYCCD SDK bindings for Rust
//!
//! This crate provides a safe interface to the QHYCCD SDK for controlling QHYCCD cameras, filter wheels and focusers.
//! The libqhyccd-sys crate provides the raw FFI bindings. It uses tracing for logging and eyre for error handling.
//!
//! # Example
//! ```no_run
//! use qhyccd_rs::Sdk;
//! let sdk = Sdk::new().expect("SDK::new failed");
//! let sdk_version = sdk.version().expect("get_sdk_version failed");
//! println!("SDK version: {:?}", sdk_version);
//! ```
//!
//! # Simulation Feature
//!
//! The `simulation` feature enables development and testing without physical hardware. When enabled,
//! [`Sdk::new()`] automatically provides a simulated camera environment that behaves like real hardware.
//!
//! ## Enabling Simulation
//!
//! Add the feature to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! qhyccd-rs = { version = "0.1.7", features = ["simulation"] }
//! ```
//!
//! ## Transparent Usage
//!
//! With simulation enabled, your code works identically for both real and simulated cameras:
//!
//! ```no_run
//! use qhyccd_rs::Sdk;
//!
//! // Same code works with or without the simulation feature
//! let sdk = Sdk::new().expect("Failed to initialize SDK");
//! let cameras = sdk.cameras();
//! println!("Found {} camera(s)", cameras.count());
//! ```
//!
//! ## Default Simulated Camera
//!
//! When compiled with the `simulation` feature, [`Sdk::new()`] automatically provides:
//!
//! - **Camera**: QHY178M-Simulated (`SIM-QHY178M`)
//!   - 3072x2048 resolution, 16-bit depth
//!   - Cooler support for temperature control
//!   - Full control API (gain, offset, exposure, etc.)
//!
//! - **Filter Wheel**: 7-position CFW
//!   - Accessible via [`Sdk::filter_wheels()`]
//!   - Complete control API support
//!
//! ## Custom Simulated Cameras
//!
//! For advanced use cases, use [`Sdk::new_simulated()`] and [`Sdk::add_simulated_camera()`]:
//!
//! ```
//! # #[cfg(feature = "simulation")]
//! # {
//! use qhyccd_rs::{Sdk, simulation::SimulatedCameraConfig};
//!
//! let mut sdk = Sdk::new_simulated();
//! let config = SimulatedCameraConfig::default()
//!     .with_id("CUSTOM-CAM")
//!     .with_filter_wheel(5);
//! sdk.add_simulated_camera(config);
//! # }
//! ```
#![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]

#[macro_use]
extern crate educe;

// Module declarations
mod backend;
mod camera;
mod control;
mod error;
mod filter_wheel;
mod sdk;
mod types;

#[cfg(test)]
pub mod mocks;

#[cfg(feature = "simulation")]
pub mod simulation;

// Public re-exports
pub use camera::Camera;
pub use control::Control;
pub use error::QHYError;
pub use filter_wheel::FilterWheel;
pub use sdk::Sdk;
pub use types::{
    BayerMode, CCDChipArea, CCDChipInfo, ImageData, ReadoutMode, SDKVersion, StreamMode,
};

// Unit tests requiring FFI mocking are in src/tests/
// Simulation integration tests are in tests/simulation/
#[cfg(test)]
mod tests;
