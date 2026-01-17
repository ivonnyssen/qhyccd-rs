//! Shared test utilities for integration tests

#![allow(dead_code)]

use qhyccd_rs::simulation::SimulatedCameraConfig;
use qhyccd_rs::{BayerMode, Camera, FilterWheel, Sdk};

/// Creates a standard simulated camera for testing
pub fn create_test_camera() -> Camera {
    let config = SimulatedCameraConfig::default().with_id("TEST-CAM-001");
    Camera::new_simulated(config)
}

/// Creates a simulated camera with cooler support
pub fn create_camera_with_cooler() -> Camera {
    let config = SimulatedCameraConfig::default()
        .with_id("TEST-CAM-COOL")
        .with_cooler();
    Camera::new_simulated(config)
}

/// Creates a simulated camera with filter wheel
pub fn create_camera_with_filter_wheel(slots: u32) -> Camera {
    let config = SimulatedCameraConfig::default()
        .with_id("TEST-CAM-FW")
        .with_filter_wheel(slots);
    Camera::new_simulated(config)
}

/// Creates a simulated color camera
pub fn create_color_camera(bayer_mode: BayerMode) -> Camera {
    let config = SimulatedCameraConfig::default()
        .with_id("TEST-CAM-COLOR")
        .with_color(bayer_mode);
    Camera::new_simulated(config)
}

/// Creates a filter wheel from a simulated camera with filter wheel support
pub fn create_test_filter_wheel(slots: u32) -> FilterWheel {
    let camera = create_camera_with_filter_wheel(slots);
    FilterWheel::new(camera)
}

/// Creates a simulated SDK for testing
pub fn create_test_sdk() -> Sdk {
    Sdk::new_simulated()
}

/// Creates a simulated camera with multiple readout modes
pub fn create_camera_with_readout_modes() -> Camera {
    let config = SimulatedCameraConfig::default()
        .with_id("TEST-CAM-MODES")
        .with_readout_mode("High Speed", 3072, 2048)
        .with_readout_mode("Low Noise", 1536, 1024);
    Camera::new_simulated(config)
}
