//! Tests for the SimulatedCameraState module

use super::config::SimulatedCameraConfig;
use super::state::SimulatedCameraState;
use crate::{BayerMode, Control};

#[test]
fn test_new_state() {
    let config = SimulatedCameraConfig::default();
    let state = SimulatedCameraState::new(config);

    assert!(!state.is_open);
    assert!(!state.is_initialized);
    assert_eq!(state.binning, (1, 1));
}

#[test]
fn test_image_dimensions() {
    let config = SimulatedCameraConfig::default();
    let mut state = SimulatedCameraState::new(config);

    let (w, h) = state.get_current_image_dimensions();
    assert_eq!(w, 3072);
    assert_eq!(h, 2048);

    state.binning = (2, 2);
    let (w, h) = state.get_current_image_dimensions();
    assert_eq!(w, 1536);
    assert_eq!(h, 1024);
}

#[test]
fn test_buffer_size() {
    let config = SimulatedCameraConfig::default();
    let state = SimulatedCameraState::new(config);

    // 3072 * 2048 * 2 bytes (16-bit) * 1 channel = 12,582,912
    assert_eq!(state.calculate_buffer_size(), 12_582_912);
}

#[test]
fn test_exposure_timing() {
    let config = SimulatedCameraConfig::default();
    let mut state = SimulatedCameraState::new(config);

    state.exposure_duration_us = 1000; // 1ms
    state.start_exposure();

    // Should not be complete immediately
    assert!(!state.is_exposure_complete());
    assert!(state.get_remaining_exposure_us() > 0);

    // Wait and check again
    std::thread::sleep(std::time::Duration::from_millis(2));
    assert!(state.is_exposure_complete());
    assert_eq!(state.get_remaining_exposure_us(), 0);
}

#[test]
fn test_cancel_exposure() {
    let config = SimulatedCameraConfig::default();
    let mut state = SimulatedCameraState::new(config);

    state.exposure_duration_us = 1_000_000; // 1 second
    state.start_exposure();

    // Exposure should be in progress
    assert!(!state.is_exposure_complete());
    assert!(state.exposure_start.is_some());

    // Cancel the exposure
    state.cancel_exposure();

    // After canceling, exposure_start should be None
    assert!(state.exposure_start.is_none());
    // is_exposure_complete returns true when exposure_start is None
    assert!(state.is_exposure_complete());
    // Remaining time should be 0
    assert_eq!(state.get_remaining_exposure_us(), 0);
}

#[test]
fn test_update_temperature_cooling() {
    let config = SimulatedCameraConfig::default().with_cooler();
    let mut state = SimulatedCameraState::new(config);

    // Set up cooling: current temp is 20C, target is 0C, PWM is max
    state.current_temperature = 20.0;
    state.target_temperature = 0.0;
    state.cooler_pwm = 255.0;

    let initial_temp = state.current_temperature;

    // Update temperature several times
    for _ in 0..10 {
        state.update_temperature();
    }

    // Temperature should have decreased
    assert!(state.current_temperature < initial_temp);
    // CurTemp parameter should be updated
    assert!(
        (state.parameters.get(&Control::CurTemp).unwrap() - state.current_temperature).abs()
            < f64::EPSILON
    );
}

#[test]
fn test_update_temperature_warming() {
    let config = SimulatedCameraConfig::default().with_cooler();
    let mut state = SimulatedCameraState::new(config);

    // Camera is cold and cooler is off
    state.current_temperature = 0.0;
    state.cooler_pwm = 0.0;

    let initial_temp = state.current_temperature;

    // Update temperature several times
    for _ in 0..10 {
        state.update_temperature();
    }

    // Temperature should have increased toward ambient (20C)
    assert!(state.current_temperature > initial_temp);
    assert!(state.current_temperature <= 20.0);
}

#[test]
fn test_get_channels_mono() {
    let config = SimulatedCameraConfig::default(); // Mono camera
    let state = SimulatedCameraState::new(config);

    // Mono camera should have 1 channel
    assert_eq!(state.get_channels(), 1);
}

#[test]
fn test_get_channels_color_no_debayer() {
    let config = SimulatedCameraConfig::default().with_color(BayerMode::RGGB);
    let state = SimulatedCameraState::new(config);

    // Color camera with debayer disabled should return 1 channel
    assert_eq!(state.get_channels(), 1);
}

#[test]
fn test_get_channels_color_debayer() {
    let config = SimulatedCameraConfig::default().with_color(BayerMode::RGGB);
    let mut state = SimulatedCameraState::new(config);

    // Enable debayer
    state.debayer_enabled = true;

    // Color camera with debayer enabled should return 3 channels
    assert_eq!(state.get_channels(), 3);
}

#[test]
fn test_bytes_per_pixel_8bit() {
    let config = SimulatedCameraConfig::default();
    let mut state = SimulatedCameraState::new(config);

    state.bit_depth = 8;
    assert_eq!(state.get_bytes_per_pixel(), 1);
}

#[test]
fn test_bytes_per_pixel_16bit() {
    let config = SimulatedCameraConfig::default();
    let mut state = SimulatedCameraState::new(config);

    state.bit_depth = 16;
    assert_eq!(state.get_bytes_per_pixel(), 2);

    // Also test intermediate values (12-bit, etc.)
    state.bit_depth = 12;
    assert_eq!(state.get_bytes_per_pixel(), 2);
}

#[test]
fn test_buffer_size_with_binning_and_channels() {
    let config = SimulatedCameraConfig::default().with_color(BayerMode::RGGB);
    let mut state = SimulatedCameraState::new(config);

    // Set 2x2 binning, 8-bit mode, and enable debayer (3 channels)
    state.binning = (2, 2);
    state.bit_depth = 8;
    state.debayer_enabled = true;

    // (3072/2) * (2048/2) * 1 byte * 3 channels = 1536 * 1024 * 3 = 4,718,592
    assert_eq!(state.calculate_buffer_size(), 4_718_592);
}

#[test]
fn test_remaining_exposure_no_exposure_started() {
    let config = SimulatedCameraConfig::default();
    let state = SimulatedCameraState::new(config);

    // No exposure started, should return 0
    assert_eq!(state.get_remaining_exposure_us(), 0);
    // Should be considered complete
    assert!(state.is_exposure_complete());
}

#[test]
fn test_start_exposure_uses_parameter() {
    let config = SimulatedCameraConfig::default();
    let mut state = SimulatedCameraState::new(config);

    // Set exposure parameter
    state.parameters.insert(Control::Exposure, 5_000_000.0); // 5 seconds

    state.start_exposure();

    // exposure_duration_us should be set from parameter
    assert_eq!(state.exposure_duration_us, 5_000_000);
}
