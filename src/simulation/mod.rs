//! Simulation support for QHYCCD cameras
//!
//! This module provides simulated camera and filter wheel support, allowing
//! library users to develop and test applications without physical QHYCCD hardware.
//!
//! # Example
//!
//! ```no_run
//! use qhyccd_rs::simulation::{SimulatedCameraConfig, ImagePattern, ImageGenerator};
//!
//! // Create a custom camera configuration
//! let config = SimulatedCameraConfig::default()
//!     .with_id("TEST-001")
//!     .with_filter_wheel(5)
//!     .with_cooler();
//!
//! // Create an image generator for testing
//! let generator = ImageGenerator::new(ImagePattern::StarField)
//!     .with_noise_level(0.02);
//! ```

mod config;
mod image_generator;
mod state;

// Note: config and image_generator tests are now in tests/simulation/
// state_tests remain here because SimulatedCameraState is pub(crate)
#[cfg(test)]
mod test_state;

pub use config::SimulatedCameraConfig;
pub use image_generator::{ImageGenerator, ImagePattern};
pub(crate) use state::SimulatedCameraState;
