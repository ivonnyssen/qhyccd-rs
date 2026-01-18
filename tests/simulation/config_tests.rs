//! Tests for the SimulatedCameraConfig module

use qhyccd_rs::simulation::SimulatedCameraConfig;
use qhyccd_rs::{BayerMode, CCDChipInfo, Control};

#[test]
fn test_default_config() {
    let config = SimulatedCameraConfig::default();
    assert_eq!(config.id, "SIM-001");
    assert_eq!(config.filter_wheel_slots, 0);
    assert!(!config.has_cooler);
    assert!(config.bayer_mode.is_none());
}

#[test]
fn test_with_filter_wheel() {
    let config = SimulatedCameraConfig::default().with_filter_wheel(5);
    assert_eq!(config.filter_wheel_slots, 5);
    assert!(config.supported_controls.contains_key(&Control::CfwPort));
    assert!(config
        .supported_controls
        .contains_key(&Control::CfwSlotsNum));
}

#[test]
fn test_with_cooler() {
    let config = SimulatedCameraConfig::default().with_cooler();
    assert!(config.has_cooler);
    assert!(config.supported_controls.contains_key(&Control::Cooler));
    assert!(config.supported_controls.contains_key(&Control::CurTemp));
}

#[test]
fn test_with_color() {
    let config = SimulatedCameraConfig::default().with_color(BayerMode::RGGB);
    assert_eq!(config.bayer_mode, Some(BayerMode::RGGB));
    assert!(config.supported_controls.contains_key(&Control::CamColor));
}

#[test]
fn test_builder_chaining() {
    let config = SimulatedCameraConfig::default()
        .with_id("TEST-001")
        .with_model("Test Camera")
        .with_filter_wheel(7)
        .with_cooler()
        .with_color(BayerMode::GBRG);

    assert_eq!(config.id, "TEST-001");
    assert_eq!(config.model, "Test Camera");
    assert_eq!(config.filter_wheel_slots, 7);
    assert!(config.has_cooler);
    assert_eq!(config.bayer_mode, Some(BayerMode::GBRG));
}

#[test]
fn test_with_model() {
    let config = SimulatedCameraConfig::default().with_model("QHY600M-PRO");
    assert_eq!(config.model, "QHY600M-PRO");
}

#[test]
fn test_with_chip_info() {
    let chip_info = CCDChipInfo {
        chip_width: 23.6,
        chip_height: 15.8,
        image_width: 6224,
        image_height: 4168,
        pixel_width: 3.76,
        pixel_height: 3.76,
        bits_per_pixel: 16,
    };
    let config = SimulatedCameraConfig::default().with_chip_info(chip_info);

    assert_eq!(config.chip_info.image_width, 6224);
    assert_eq!(config.chip_info.image_height, 4168);
    assert!((config.chip_info.pixel_width - 3.76).abs() < f64::EPSILON);
    // Effective area should be updated to match chip info
    assert_eq!(config.effective_area.width, 6224);
    assert_eq!(config.effective_area.height, 4168);
    assert_eq!(config.overscan_area.width, 6224);
    assert_eq!(config.overscan_area.height, 4168);
}

#[test]
fn test_with_firmware_version() {
    let config =
        SimulatedCameraConfig::default().with_firmware_version("Firmware version: 2025_3_15");
    assert_eq!(config.firmware_version, "Firmware version: 2025_3_15");
}

#[test]
fn test_with_control() {
    let config =
        SimulatedCameraConfig::default().with_control(Control::Brightness, 0.0, 100.0, 0.1);

    assert!(config.supported_controls.contains_key(&Control::Brightness));
    let (min, max, step) = config.supported_controls.get(&Control::Brightness).unwrap();
    assert!((min - 0.0).abs() < f64::EPSILON);
    assert!((max - 100.0).abs() < f64::EPSILON);
    assert!((step - 0.1).abs() < f64::EPSILON);
}

#[test]
fn test_with_filter_wheel_zero_slots() {
    // Edge case: filter wheel with 0 slots should not add CFW controls
    let config = SimulatedCameraConfig::default().with_filter_wheel(0);
    assert_eq!(config.filter_wheel_slots, 0);
    assert!(!config.supported_controls.contains_key(&Control::CfwPort));
}

#[test]
fn test_with_readout_mode() {
    let config = SimulatedCameraConfig::default()
        .with_readout_mode("High Speed", 3072, 2048)
        .with_readout_mode("Low Noise", 1536, 1024);

    assert_eq!(config.readout_modes.len(), 3); // default + 2 custom
    assert_eq!(config.readout_modes[1].0, "High Speed");
    assert_eq!(config.readout_modes[1].1, (3072, 2048));
    assert_eq!(config.readout_modes[2].0, "Low Noise");
    assert_eq!(config.readout_modes[2].1, (1536, 1024));
}
