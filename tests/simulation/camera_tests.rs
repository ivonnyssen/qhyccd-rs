//! Integration tests for simulated cameras
//!
//! These tests verify that simulated cameras work correctly without
//! requiring actual QHYCCD hardware.

use qhyccd_rs::simulation::{ImageGenerator, ImagePattern, SimulatedCameraConfig};
use qhyccd_rs::{BayerMode, CCDChipArea, Camera, Control, FilterWheel, Sdk, StreamMode};

#[test]
fn test_simulated_camera_creation() {
    let config = SimulatedCameraConfig::default();
    let camera = Camera::new_simulated(config);
    assert!(camera.is_simulated());
    assert_eq!(camera.id(), "SIM-001");
}

#[test]
fn test_simulated_camera_open_close() {
    let config = SimulatedCameraConfig::default();
    let camera = Camera::new_simulated(config);

    // Initially not open
    assert!(!camera.is_open().unwrap());

    // Open the camera
    camera.open().unwrap();
    assert!(camera.is_open().unwrap());

    // Close the camera
    camera.close().unwrap();
    assert!(!camera.is_open().unwrap());
}

#[test]
fn test_simulated_camera_info() {
    let config = SimulatedCameraConfig::default();
    let camera = Camera::new_simulated(config);
    camera.open().unwrap();

    let model = camera.get_model().unwrap();
    assert_eq!(model, "QHY-SIMULATED");

    let chip_info = camera.get_ccd_info().unwrap();
    assert_eq!(chip_info.image_width, 3072);
    assert_eq!(chip_info.image_height, 2048);

    let camera_type = camera.get_type().unwrap();
    assert_eq!(camera_type, 4010);

    camera.close().unwrap();
}

#[test]
fn test_simulated_camera_readout_modes() {
    let config = SimulatedCameraConfig::default()
        .with_readout_mode("High Speed", 3072, 2048)
        .with_readout_mode("Low Noise", 1536, 1024);
    let camera = Camera::new_simulated(config);
    camera.open().unwrap();

    let num_modes = camera.get_number_of_readout_modes().unwrap();
    assert_eq!(num_modes, 3); // default "Standard" + 2 custom

    let name = camera.get_readout_mode_name(0).unwrap();
    assert_eq!(name, "Standard");

    let (width, height) = camera.get_readout_mode_resolution(1).unwrap();
    assert_eq!(width, 3072);
    assert_eq!(height, 2048);

    camera.close().unwrap();
}

#[test]
fn test_simulated_camera_parameters() {
    let config = SimulatedCameraConfig::default();
    let camera = Camera::new_simulated(config);
    camera.open().unwrap();

    // Set and get exposure
    camera.set_parameter(Control::Exposure, 1000.0).unwrap();
    let exposure = camera.get_parameter(Control::Exposure).unwrap();
    assert!((exposure - 1000.0).abs() < f64::EPSILON);

    // Set and get gain
    camera.set_parameter(Control::Gain, 50.0).unwrap();
    let gain = camera.get_parameter(Control::Gain).unwrap();
    assert!((gain - 50.0).abs() < f64::EPSILON);

    // Get parameter min/max/step
    let (min, max, step) = camera.get_parameter_min_max_step(Control::Gain).unwrap();
    assert!((min - 0.0).abs() < f64::EPSILON);
    assert!((max - 100.0).abs() < f64::EPSILON);
    assert!((step - 1.0).abs() < f64::EPSILON);

    camera.close().unwrap();
}

#[test]
fn test_simulated_camera_is_control_available() {
    let config = SimulatedCameraConfig::default().with_cooler();
    let camera = Camera::new_simulated(config);
    camera.open().unwrap();

    // Default controls should be available
    assert!(camera.is_control_available(Control::Gain).is_some());
    assert!(camera.is_control_available(Control::Exposure).is_some());

    // Cooler controls should be available
    assert!(camera.is_control_available(Control::Cooler).is_some());
    assert!(camera.is_control_available(Control::CurTemp).is_some());

    // CFW controls should NOT be available (no filter wheel)
    assert!(camera.is_control_available(Control::CfwPort).is_none());

    camera.close().unwrap();
}

#[test]
fn test_simulated_camera_with_filter_wheel() {
    let config = SimulatedCameraConfig::default().with_filter_wheel(5);
    let camera = Camera::new_simulated(config);
    camera.open().unwrap();

    // CFW controls should be available
    assert!(camera.is_control_available(Control::CfwPort).is_some());
    assert!(camera.is_control_available(Control::CfwSlotsNum).is_some());

    // Check filter wheel plugged in
    assert!(camera.is_cfw_plugged_in().unwrap());

    // Get number of slots
    let slots = camera.get_parameter(Control::CfwSlotsNum).unwrap();
    assert!((slots - 5.0).abs() < f64::EPSILON);

    camera.close().unwrap();
}

#[test]
fn test_simulated_camera_single_frame_mode() {
    let config = SimulatedCameraConfig::default();
    let camera = Camera::new_simulated(config);
    camera.open().unwrap();

    camera.set_stream_mode(StreamMode::SingleFrameMode).unwrap();
    camera.init().unwrap();
    camera.set_parameter(Control::Exposure, 1000.0).unwrap(); // 1ms

    let buffer_size = camera.get_image_size().unwrap();
    assert!(buffer_size > 0);

    camera.start_single_frame_exposure().unwrap();
    let image = camera.get_single_frame(buffer_size).unwrap();

    assert_eq!(image.width, 3072);
    assert_eq!(image.height, 2048);
    assert_eq!(image.bits_per_pixel, 16);
    assert!(!image.data.is_empty());

    camera.close().unwrap();
}

#[test]
fn test_simulated_camera_live_mode() {
    let config = SimulatedCameraConfig::default();
    let camera = Camera::new_simulated(config);
    camera.open().unwrap();

    camera.set_stream_mode(StreamMode::LiveMode).unwrap();
    camera.init().unwrap();
    camera.begin_live().unwrap();

    let buffer_size = camera.get_image_size().unwrap();
    let image = camera.get_live_frame(buffer_size).unwrap();

    assert_eq!(image.width, 3072);
    assert_eq!(image.height, 2048);
    assert!(!image.data.is_empty());

    camera.end_live().unwrap();
    camera.close().unwrap();
}

#[test]
fn test_simulated_camera_binning() {
    let config = SimulatedCameraConfig::default();
    let camera = Camera::new_simulated(config);
    camera.open().unwrap();
    camera.set_stream_mode(StreamMode::LiveMode).unwrap();
    camera.init().unwrap();

    // Set 2x2 binning
    camera.set_bin_mode(2, 2).unwrap();
    camera.begin_live().unwrap();

    let buffer_size = camera.get_image_size().unwrap();
    let image = camera.get_live_frame(buffer_size).unwrap();

    // With 2x2 binning, dimensions should be halved
    assert_eq!(image.width, 1536);
    assert_eq!(image.height, 1024);

    camera.end_live().unwrap();
    camera.close().unwrap();
}

#[test]
fn test_simulated_camera_bit_mode() {
    let config = SimulatedCameraConfig::default();
    let camera = Camera::new_simulated(config);
    camera.open().unwrap();
    camera.set_stream_mode(StreamMode::LiveMode).unwrap();
    camera.init().unwrap();

    // Set 8-bit mode
    camera.set_bit_mode(8).unwrap();
    camera.begin_live().unwrap();

    let buffer_size = camera.get_image_size().unwrap();
    let image = camera.get_live_frame(buffer_size).unwrap();

    assert_eq!(image.bits_per_pixel, 8);
    // 8-bit mode uses 1 byte per pixel
    assert_eq!(image.data.len(), (3072 * 2048) as usize);

    camera.end_live().unwrap();
    camera.close().unwrap();
}

#[test]
fn test_simulated_sdk_new_simulated() {
    let sdk = Sdk::new_simulated();
    assert_eq!(sdk.cameras().count(), 0);
    assert_eq!(sdk.filter_wheels().count(), 0);
}

#[test]
fn test_simulated_sdk_add_camera() {
    let mut sdk = Sdk::new_simulated();

    let config = SimulatedCameraConfig::default().with_id("TEST-001");
    sdk.add_simulated_camera(config);

    assert_eq!(sdk.cameras().count(), 1);

    let camera = sdk.cameras().next().unwrap();
    assert_eq!(camera.id(), "TEST-001");
    assert!(camera.is_simulated());
}

#[test]
fn test_simulated_sdk_add_camera_with_filter_wheel() {
    let mut sdk = Sdk::new_simulated();

    let config = SimulatedCameraConfig::default()
        .with_id("CAM-WITH-FW")
        .with_filter_wheel(7);
    sdk.add_simulated_camera(config);

    assert_eq!(sdk.cameras().count(), 1);
    assert_eq!(sdk.filter_wheels().count(), 1);

    let fw = sdk.filter_wheels().next().unwrap();
    assert_eq!(fw.id(), "CAM-WITH-FW");
}

#[test]
fn test_simulated_filter_wheel() {
    let config = SimulatedCameraConfig::default()
        .with_id("FW-TEST")
        .with_filter_wheel(5);
    let camera = Camera::new_simulated(config);
    let fw = FilterWheel::new(camera);

    fw.open().unwrap();
    assert!(fw.is_open().unwrap());
    assert!(fw.is_cfw_plugged_in().unwrap());

    let num_filters = fw.get_number_of_filters().unwrap();
    assert_eq!(num_filters, 5);

    // Get initial position
    let pos = fw.get_fw_position().unwrap();
    assert_eq!(pos, 0);

    // Set position
    fw.set_fw_position(3).unwrap();
    let pos = fw.get_fw_position().unwrap();
    assert_eq!(pos, 3);

    fw.close().unwrap();
}

#[test]
fn test_simulated_color_camera() {
    let config = SimulatedCameraConfig::default().with_color(BayerMode::RGGB);
    let camera = Camera::new_simulated(config);
    camera.open().unwrap();

    // Check color mode is available
    let bayer = camera.is_control_available(Control::CamColor);
    assert!(bayer.is_some());
    assert_eq!(bayer.unwrap(), BayerMode::RGGB as u32);

    camera.close().unwrap();
}

#[test]
fn test_image_generator_gradient() {
    let gen = ImageGenerator::default();
    let data = gen.generate_16bit(100, 100, 1);
    assert_eq!(data.len(), 20000); // 100 * 100 * 2 bytes

    // Verify gradient - right side should be brighter than left
    let left_pixel = u16::from_le_bytes([data[0], data[1]]);
    let right_pixel = u16::from_le_bytes([data[198], data[199]]);
    assert!(right_pixel > left_pixel);
}

#[test]
fn test_image_generator_starfield() {
    let gen = ImageGenerator::new(ImagePattern::StarField);
    let data = gen.generate_16bit(200, 200, 1);
    assert_eq!(data.len(), 80000);

    // Starfield should have some variation (stars)
    let mut min_val = u16::MAX;
    let mut max_val = u16::MIN;
    for i in (0..data.len()).step_by(2) {
        let val = u16::from_le_bytes([data[i], data[i + 1]]);
        min_val = min_val.min(val);
        max_val = max_val.max(val);
    }
    // Should have significant contrast between background and stars
    assert!(max_val - min_val > 10000);
}

#[test]
fn test_exposure_timing() {
    let config = SimulatedCameraConfig::default();
    let camera = Camera::new_simulated(config);
    camera.open().unwrap();
    camera.set_stream_mode(StreamMode::SingleFrameMode).unwrap();
    camera.init().unwrap();

    // Set a short exposure
    camera.set_parameter(Control::Exposure, 50000.0).unwrap(); // 50ms

    camera.start_single_frame_exposure().unwrap();

    // Check remaining exposure time immediately
    let remaining = camera.get_remaining_exposure_us().unwrap();
    assert!(remaining > 0);
    assert!(remaining <= 50000);

    // Wait a bit and check again
    std::thread::sleep(std::time::Duration::from_millis(30));
    let remaining_after = camera.get_remaining_exposure_us().unwrap();
    assert!(remaining_after < remaining);

    camera.close().unwrap();
}

#[test]
fn test_set_readout_mode() {
    let config = SimulatedCameraConfig::default()
        .with_readout_mode("High Speed", 3072, 2048)
        .with_readout_mode("Low Noise", 1536, 1024);
    let camera = Camera::new_simulated(config);
    camera.open().unwrap();

    // Set to mode 1 (High Speed)
    camera.set_readout_mode(1).unwrap();

    // Verify we can get mode resolution for mode 1
    let (width, height) = camera.get_readout_mode_resolution(1).unwrap();
    assert_eq!(width, 3072);
    assert_eq!(height, 2048);

    // Set to mode 2 (Low Noise)
    camera.set_readout_mode(2).unwrap();

    // Test invalid mode - should fail
    let result = camera.set_readout_mode(99);
    assert!(result.is_err());

    camera.close().unwrap();
}

#[test]
fn test_get_firmware_version() {
    let config =
        SimulatedCameraConfig::default().with_firmware_version("Firmware version: 2025_3_15");
    let camera = Camera::new_simulated(config);
    camera.open().unwrap();

    let version = camera.get_firmware_version().unwrap();
    assert_eq!(version, "Firmware version: 2025_3_15");

    camera.close().unwrap();
}

#[test]
fn test_set_roi() {
    let config = SimulatedCameraConfig::default();
    let camera = Camera::new_simulated(config);
    camera.open().unwrap();
    camera.set_stream_mode(StreamMode::LiveMode).unwrap();
    camera.init().unwrap();

    // Set a custom ROI
    let roi = CCDChipArea {
        start_x: 100,
        start_y: 100,
        width: 1000,
        height: 800,
    };
    camera.set_roi(roi).unwrap();

    // Begin live mode and capture a frame
    camera.begin_live().unwrap();

    let buffer_size = camera.get_image_size().unwrap();
    let image = camera.get_live_frame(buffer_size).unwrap();

    // Image dimensions should match ROI
    assert_eq!(image.width, 1000);
    assert_eq!(image.height, 800);

    camera.end_live().unwrap();
    camera.close().unwrap();
}

#[test]
fn test_get_effective_area() {
    let config = SimulatedCameraConfig::default();
    let camera = Camera::new_simulated(config);
    camera.open().unwrap();

    let effective_area = camera.get_effective_area().unwrap();

    // Default config effective area
    assert_eq!(effective_area.start_x, 0);
    assert_eq!(effective_area.start_y, 0);
    assert_eq!(effective_area.width, 3072);
    assert_eq!(effective_area.height, 2048);

    camera.close().unwrap();
}

#[test]
fn test_get_overscan_area() {
    let config = SimulatedCameraConfig::default();
    let camera = Camera::new_simulated(config);
    camera.open().unwrap();

    let overscan_area = camera.get_overscan_area().unwrap();

    // Default config overscan area matches effective area
    assert_eq!(overscan_area.start_x, 0);
    assert_eq!(overscan_area.start_y, 0);
    assert_eq!(overscan_area.width, 3072);
    assert_eq!(overscan_area.height, 2048);

    camera.close().unwrap();
}

#[test]
fn test_stop_exposure() {
    let config = SimulatedCameraConfig::default();
    let camera = Camera::new_simulated(config);
    camera.open().unwrap();
    camera.set_stream_mode(StreamMode::SingleFrameMode).unwrap();
    camera.init().unwrap();

    // Set a long exposure
    camera
        .set_parameter(Control::Exposure, 10_000_000.0)
        .unwrap(); // 10 seconds

    camera.start_single_frame_exposure().unwrap();

    // Exposure should be in progress
    let remaining = camera.get_remaining_exposure_us().unwrap();
    assert!(remaining > 0);

    // Stop the exposure
    camera.stop_exposure().unwrap();

    // Remaining time should be 0 after stopping
    let remaining_after = camera.get_remaining_exposure_us().unwrap();
    assert_eq!(remaining_after, 0);

    camera.close().unwrap();
}

#[test]
fn test_abort_exposure_and_readout() {
    let config = SimulatedCameraConfig::default();
    let camera = Camera::new_simulated(config);
    camera.open().unwrap();
    camera.set_stream_mode(StreamMode::SingleFrameMode).unwrap();
    camera.init().unwrap();

    // Set a long exposure
    camera
        .set_parameter(Control::Exposure, 10_000_000.0)
        .unwrap(); // 10 seconds

    camera.start_single_frame_exposure().unwrap();

    // Exposure should be in progress
    let remaining = camera.get_remaining_exposure_us().unwrap();
    assert!(remaining > 0);

    // Abort the exposure and readout
    camera.abort_exposure_and_readout().unwrap();

    // Remaining time should be 0 after aborting
    let remaining_after = camera.get_remaining_exposure_us().unwrap();
    assert_eq!(remaining_after, 0);

    camera.close().unwrap();
}

#[test]
fn test_set_debayer() {
    let config = SimulatedCameraConfig::default().with_color(BayerMode::RGGB);
    let camera = Camera::new_simulated(config);
    camera.open().unwrap();
    camera.set_stream_mode(StreamMode::LiveMode).unwrap();
    camera.init().unwrap();

    // Enable debayer
    camera.set_debayer(true).unwrap();
    camera.begin_live().unwrap();

    let buffer_size = camera.get_image_size().unwrap();
    let image_debayer_on = camera.get_live_frame(buffer_size).unwrap();

    // With debayer enabled, should get 3-channel RGB image
    // Check buffer size is 3x larger than mono
    let mono_pixels = 3072 * 2048;
    let expected_rgb_size = mono_pixels * 2 * 3; // 16-bit * 3 channels
    assert_eq!(image_debayer_on.data.len(), expected_rgb_size as usize);

    camera.end_live().unwrap();

    // Disable debayer
    camera.set_debayer(false).unwrap();
    camera.begin_live().unwrap();

    let buffer_size = camera.get_image_size().unwrap();
    let image_debayer_off = camera.get_live_frame(buffer_size).unwrap();

    // With debayer disabled, should get 1-channel image
    let expected_mono_size = mono_pixels * 2; // 16-bit * 1 channel
    assert_eq!(image_debayer_off.data.len(), expected_mono_size as usize);

    camera.end_live().unwrap();
    camera.close().unwrap();
}

#[test]
fn test_camera_not_open_errors() {
    let config = SimulatedCameraConfig::default();
    let camera = Camera::new_simulated(config);

    // Camera is not open - these should all fail
    assert!(camera.get_model().is_err());
    assert!(camera.get_ccd_info().is_err());
    assert!(camera.get_firmware_version().is_err());
    assert!(camera.get_effective_area().is_err());
    assert!(camera.get_overscan_area().is_err());
    assert!(camera.set_parameter(Control::Gain, 50.0).is_err());
    assert!(camera.get_parameter(Control::Gain).is_err());
    assert!(camera.stop_exposure().is_err());
    assert!(camera.abort_exposure_and_readout().is_err());
}

#[test]
fn test_parameter_not_available_error() {
    let config = SimulatedCameraConfig::default(); // No cooler
    let camera = Camera::new_simulated(config);
    camera.open().unwrap();

    // Cooler control should not be available
    assert!(camera.is_control_available(Control::Cooler).is_none());

    // Getting an unavailable control should fail
    assert!(camera.get_parameter(Control::Cooler).is_err());

    // get_parameter_min_max_step should fail for unavailable control
    assert!(camera.get_parameter_min_max_step(Control::Cooler).is_err());

    // set_if_available should fail for unavailable control
    assert!(camera.set_if_available(Control::Cooler, -10.0).is_err());

    camera.close().unwrap();
}

#[test]
fn test_set_if_available() {
    let config = SimulatedCameraConfig::default().with_cooler();
    let camera = Camera::new_simulated(config);
    camera.open().unwrap();

    // set_if_available should work for available control
    camera.set_if_available(Control::Gain, 50.0).unwrap();
    let gain = camera.get_parameter(Control::Gain).unwrap();
    assert!((gain - 50.0).abs() < f64::EPSILON);

    // set_if_available should work for cooler (available due to with_cooler)
    camera.set_if_available(Control::Cooler, -10.0).unwrap();

    camera.close().unwrap();
}

#[test]
fn test_double_open() {
    let config = SimulatedCameraConfig::default();
    let camera = Camera::new_simulated(config);

    // First open should succeed
    camera.open().unwrap();
    assert!(camera.is_open().unwrap());

    // Second open should also succeed (idempotent)
    camera.open().unwrap();
    assert!(camera.is_open().unwrap());

    camera.close().unwrap();
}

#[test]
fn test_double_close() {
    let config = SimulatedCameraConfig::default();
    let camera = Camera::new_simulated(config);

    camera.open().unwrap();
    assert!(camera.is_open().unwrap());

    // First close should succeed
    camera.close().unwrap();
    assert!(!camera.is_open().unwrap());

    // Second close should also succeed (idempotent)
    camera.close().unwrap();
    assert!(!camera.is_open().unwrap());
}

#[test]
fn test_get_ccd_info_not_open() {
    let config = SimulatedCameraConfig::default();
    let camera = Camera::new_simulated(config);

    // Camera is not open, should fail
    assert!(camera.get_ccd_info().is_err());

    // After opening, should succeed
    camera.open().unwrap();
    let chip_info = camera.get_ccd_info().unwrap();
    assert_eq!(chip_info.image_width, 3072);

    camera.close().unwrap();
}

#[test]
fn test_no_filter_wheel() {
    let config = SimulatedCameraConfig::default(); // No filter wheel
    let camera = Camera::new_simulated(config);
    camera.open().unwrap();

    // Should report no filter wheel plugged in
    assert!(!camera.is_cfw_plugged_in().unwrap());

    // CFW control should not be available
    assert!(camera.is_control_available(Control::CfwPort).is_none());

    camera.close().unwrap();
}

// ============================================================================
// Trait Implementation Tests
// ============================================================================

#[test]
fn test_camera_clone() {
    let config = SimulatedCameraConfig::default().with_id("CLONE-TEST");
    let camera1 = Camera::new_simulated(config);

    // Clone the camera
    let camera2 = camera1.clone();

    // Both should have the same ID
    assert_eq!(camera1.id(), camera2.id());
    assert_eq!(camera1.id(), "CLONE-TEST");

    // Both should be simulated
    assert!(camera1.is_simulated());
    assert!(camera2.is_simulated());

    // Opening one should not affect the other's state query
    camera1.open().unwrap();
    assert!(camera1.is_open().unwrap());

    camera1.close().unwrap();
}

#[test]
fn test_camera_partial_eq() {
    let config1 = SimulatedCameraConfig::default().with_id("CAM-A");
    let config2 = SimulatedCameraConfig::default().with_id("CAM-B");
    let config3 = SimulatedCameraConfig::default().with_id("CAM-A");

    let camera_a = Camera::new_simulated(config1);
    let camera_b = Camera::new_simulated(config2);
    let camera_a2 = Camera::new_simulated(config3);

    // Cameras with the same ID should be equal
    assert_eq!(camera_a, camera_a2);

    // Cameras with different IDs should not be equal
    assert_ne!(camera_a, camera_b);
}

#[test]
fn test_camera_debug() {
    let config = SimulatedCameraConfig::default().with_id("DEBUG-TEST");
    let camera = Camera::new_simulated(config);

    let debug_str = format!("{:?}", camera);
    assert!(debug_str.contains("Camera"));
    assert!(debug_str.contains("DEBUG-TEST"));
}

// ============================================================================
// Additional Error Case Tests
// ============================================================================

#[test]
fn test_init_not_open_error() {
    let config = SimulatedCameraConfig::default();
    let camera = Camera::new_simulated(config);

    // init() without opening should fail
    assert!(camera.init().is_err());
}

#[test]
fn test_set_stream_mode_not_open_error() {
    let config = SimulatedCameraConfig::default();
    let camera = Camera::new_simulated(config);

    // set_stream_mode() without opening should fail
    assert!(camera.set_stream_mode(StreamMode::LiveMode).is_err());
}

#[test]
fn test_set_bin_mode_not_open_error() {
    let config = SimulatedCameraConfig::default();
    let camera = Camera::new_simulated(config);

    // set_bin_mode() without opening should fail
    assert!(camera.set_bin_mode(2, 2).is_err());
}

#[test]
fn test_set_bit_mode_not_open_error() {
    let config = SimulatedCameraConfig::default();
    let camera = Camera::new_simulated(config);

    // set_bit_mode() without opening should fail
    assert!(camera.set_bit_mode(8).is_err());
}

#[test]
fn test_begin_live_not_open_error() {
    let config = SimulatedCameraConfig::default();
    let camera = Camera::new_simulated(config);

    // begin_live() without opening should fail
    assert!(camera.begin_live().is_err());
}

#[test]
fn test_end_live_not_open_error() {
    let config = SimulatedCameraConfig::default();
    let camera = Camera::new_simulated(config);

    // end_live() without opening should fail
    assert!(camera.end_live().is_err());
}

#[test]
fn test_get_image_size_not_open_error() {
    let config = SimulatedCameraConfig::default();
    let camera = Camera::new_simulated(config);

    // get_image_size() without opening should fail
    assert!(camera.get_image_size().is_err());
}

#[test]
fn test_get_live_frame_not_open_error() {
    let config = SimulatedCameraConfig::default();
    let camera = Camera::new_simulated(config);

    // get_live_frame() without opening should fail
    assert!(camera.get_live_frame(1000).is_err());
}

#[test]
fn test_get_single_frame_not_open_error() {
    let config = SimulatedCameraConfig::default();
    let camera = Camera::new_simulated(config);

    // get_single_frame() without opening should fail
    assert!(camera.get_single_frame(1000).is_err());
}

#[test]
fn test_start_single_frame_exposure_not_open_error() {
    let config = SimulatedCameraConfig::default();
    let camera = Camera::new_simulated(config);

    // start_single_frame_exposure() without opening should fail
    assert!(camera.start_single_frame_exposure().is_err());
}

#[test]
fn test_get_remaining_exposure_not_open_error() {
    let config = SimulatedCameraConfig::default();
    let camera = Camera::new_simulated(config);

    // get_remaining_exposure_us() without opening should fail
    assert!(camera.get_remaining_exposure_us().is_err());
}

#[test]
fn test_set_roi_not_open_error() {
    let config = SimulatedCameraConfig::default();
    let camera = Camera::new_simulated(config);

    let roi = CCDChipArea {
        start_x: 0,
        start_y: 0,
        width: 100,
        height: 100,
    };

    // set_roi() without opening should fail
    assert!(camera.set_roi(roi).is_err());
}

#[test]
fn test_set_debayer_not_open_error() {
    let config = SimulatedCameraConfig::default().with_color(BayerMode::RGGB);
    let camera = Camera::new_simulated(config);

    // set_debayer() without opening should fail
    assert!(camera.set_debayer(true).is_err());
}

#[test]
fn test_is_cfw_plugged_in_not_open_error() {
    let config = SimulatedCameraConfig::default().with_filter_wheel(5);
    let camera = Camera::new_simulated(config);

    // is_cfw_plugged_in() without opening should fail
    assert!(camera.is_cfw_plugged_in().is_err());
}

#[test]
fn test_get_type_not_open_error() {
    let config = SimulatedCameraConfig::default();
    let camera = Camera::new_simulated(config);

    // get_type() without opening should fail
    assert!(camera.get_type().is_err());
}

#[test]
fn test_get_number_of_readout_modes_not_open_error() {
    let config = SimulatedCameraConfig::default();
    let camera = Camera::new_simulated(config);

    // get_number_of_readout_modes() without opening should fail
    assert!(camera.get_number_of_readout_modes().is_err());
}

#[test]
fn test_get_readout_mode_name_not_open_error() {
    let config = SimulatedCameraConfig::default();
    let camera = Camera::new_simulated(config);

    // get_readout_mode_name() without opening should fail
    assert!(camera.get_readout_mode_name(0).is_err());
}

#[test]
fn test_get_readout_mode_resolution_not_open_error() {
    let config = SimulatedCameraConfig::default();
    let camera = Camera::new_simulated(config);

    // get_readout_mode_resolution() without opening should fail
    assert!(camera.get_readout_mode_resolution(0).is_err());
}

#[test]
fn test_set_readout_mode_not_open_error() {
    let config = SimulatedCameraConfig::default();
    let camera = Camera::new_simulated(config);

    // set_readout_mode() without opening should fail
    assert!(camera.set_readout_mode(0).is_err());
}

#[test]
fn test_is_control_available_not_open_error() {
    let config = SimulatedCameraConfig::default();
    let camera = Camera::new_simulated(config);

    // is_control_available() without opening returns None (not an error, but no result)
    // This is expected behavior - returns None when camera not open
    let result = camera.is_control_available(Control::Gain);
    assert!(result.is_none());
}

#[test]
fn test_get_parameter_min_max_step_not_open_error() {
    let config = SimulatedCameraConfig::default();
    let camera = Camera::new_simulated(config);

    // get_parameter_min_max_step() without opening should fail
    assert!(camera.get_parameter_min_max_step(Control::Gain).is_err());
}

#[test]
fn test_invalid_readout_mode_name() {
    let config = SimulatedCameraConfig::default();
    let camera = Camera::new_simulated(config);
    camera.open().unwrap();

    // Requesting a non-existent readout mode should fail
    let result = camera.get_readout_mode_name(99);
    assert!(result.is_err());

    camera.close().unwrap();
}

#[test]
fn test_invalid_readout_mode_resolution() {
    let config = SimulatedCameraConfig::default();
    let camera = Camera::new_simulated(config);
    camera.open().unwrap();

    // Requesting a non-existent readout mode resolution should fail
    let result = camera.get_readout_mode_resolution(99);
    assert!(result.is_err());

    camera.close().unwrap();
}

#[test]
fn test_filter_wheel_not_open_errors() {
    let config = SimulatedCameraConfig::default()
        .with_id("FW-ERROR-TEST")
        .with_filter_wheel(5);
    let camera = Camera::new_simulated(config);
    let fw = FilterWheel::new(camera);

    // Filter wheel operations without opening should fail
    assert!(fw.get_number_of_filters().is_err());
    assert!(fw.get_fw_position().is_err());
    assert!(fw.set_fw_position(1).is_err());
    assert!(fw.is_cfw_plugged_in().is_err());
}

#[test]
fn test_multiple_simulated_cameras() {
    let mut sdk = Sdk::new_simulated();

    // Add multiple cameras
    sdk.add_simulated_camera(SimulatedCameraConfig::default().with_id("CAM-001"));
    sdk.add_simulated_camera(SimulatedCameraConfig::default().with_id("CAM-002"));
    sdk.add_simulated_camera(
        SimulatedCameraConfig::default()
            .with_id("CAM-003")
            .with_filter_wheel(5),
    );

    assert_eq!(sdk.cameras().count(), 3);
    assert_eq!(sdk.filter_wheels().count(), 1); // Only CAM-003 has a filter wheel

    // Verify camera IDs
    let ids: Vec<_> = sdk.cameras().map(|c| c.id().to_string()).collect();
    assert!(ids.contains(&"CAM-001".to_string()));
    assert!(ids.contains(&"CAM-002".to_string()));
    assert!(ids.contains(&"CAM-003".to_string()));
}

#[test]
fn test_get_current_readout_mode() {
    let config = SimulatedCameraConfig::default()
        .with_readout_mode("High Speed", 3072, 2048)
        .with_readout_mode("Low Noise", 1536, 1024);
    let camera = Camera::new_simulated(config);
    camera.open().unwrap();

    // Get initial readout mode (should be 0)
    let mode = camera.get_readout_mode().unwrap();
    assert_eq!(mode, 0);

    // Set to mode 1
    camera.set_readout_mode(1).unwrap();

    // Get current mode (should be 1)
    let mode = camera.get_readout_mode().unwrap();
    assert_eq!(mode, 1);

    // Set to mode 2
    camera.set_readout_mode(2).unwrap();
    let mode = camera.get_readout_mode().unwrap();
    assert_eq!(mode, 2);

    camera.close().unwrap();
}

#[test]
fn test_get_readout_mode_not_open_error() {
    let config = SimulatedCameraConfig::default();
    let camera = Camera::new_simulated(config);

    // get_readout_mode() without opening should fail
    assert!(camera.get_readout_mode().is_err());
}

#[test]
fn test_custom_chip_info() {
    use qhyccd_rs::CCDChipInfo;

    let chip_info = CCDChipInfo {
        chip_width: 14.0,
        chip_height: 10.0,
        image_width: 4096,
        image_height: 3072,
        pixel_width: 3.76,
        pixel_height: 3.76,
        bits_per_pixel: 16,
    };
    let config = SimulatedCameraConfig::default().with_chip_info(chip_info);
    let camera = Camera::new_simulated(config);
    camera.open().unwrap();

    let info = camera.get_ccd_info().unwrap();
    assert_eq!(info.image_width, 4096);
    assert_eq!(info.image_height, 3072);
    assert!((info.pixel_width - 3.76).abs() < 0.01);
    assert!((info.pixel_height - 3.76).abs() < 0.01);

    camera.close().unwrap();
}

#[test]
fn test_cooler_controls() {
    let config = SimulatedCameraConfig::default().with_cooler();
    let camera = Camera::new_simulated(config);
    camera.open().unwrap();

    // Check cooler control is available
    assert!(camera.is_control_available(Control::Cooler).is_some());
    assert!(camera.is_control_available(Control::CurTemp).is_some());

    // Get current temperature
    let temp = camera.get_parameter(Control::CurTemp).unwrap();
    assert!(temp > -50.0 && temp < 50.0); // Reasonable range

    // Set cooler target temperature
    camera.set_parameter(Control::Cooler, -10.0).unwrap();
    let cooler_target = camera.get_parameter(Control::Cooler).unwrap();
    assert!((cooler_target - (-10.0)).abs() < 0.001);

    camera.close().unwrap();
}

#[test]
fn test_usb_traffic_control() {
    let config = SimulatedCameraConfig::default();
    let camera = Camera::new_simulated(config);
    camera.open().unwrap();

    // Check USB traffic control is available
    assert!(camera.is_control_available(Control::UsbTraffic).is_some());

    // Get and set USB traffic
    let (min, max, step) = camera
        .get_parameter_min_max_step(Control::UsbTraffic)
        .unwrap();
    assert!(min <= max);
    assert!(step > 0.0);

    camera.set_parameter(Control::UsbTraffic, 50.0).unwrap();
    let traffic = camera.get_parameter(Control::UsbTraffic).unwrap();
    assert!((traffic - 50.0).abs() < f64::EPSILON);

    camera.close().unwrap();
}

#[test]
fn test_offset_control() {
    let config = SimulatedCameraConfig::default();
    let camera = Camera::new_simulated(config);
    camera.open().unwrap();

    // Check offset control is available
    assert!(camera.is_control_available(Control::Offset).is_some());

    // Get and set offset
    camera.set_parameter(Control::Offset, 10.0).unwrap();
    let offset = camera.get_parameter(Control::Offset).unwrap();
    assert!((offset - 10.0).abs() < f64::EPSILON);

    camera.close().unwrap();
}

#[test]
fn test_image_with_custom_generator() {
    let gen = ImageGenerator::new(ImagePattern::TestPattern)
        .with_noise_level(5.0)
        .with_base_level(1000);

    let config = SimulatedCameraConfig::default();
    let camera = Camera::new_simulated(config);
    camera.open().unwrap();
    camera.set_stream_mode(StreamMode::LiveMode).unwrap();
    camera.init().unwrap();
    camera.begin_live().unwrap();

    let buffer_size = camera.get_image_size().unwrap();
    let image = camera.get_live_frame(buffer_size).unwrap();

    // Image should have expected dimensions
    assert_eq!(image.width, 3072);
    assert_eq!(image.height, 2048);
    assert!(!image.data.is_empty());

    camera.end_live().unwrap();
    camera.close().unwrap();

    // Also test the generator directly produces test pattern
    let data = gen.generate_16bit(100, 100, 1);
    assert_eq!(data.len(), 20000); // 100 * 100 * 2 bytes
}

#[test]
fn test_filter_wheel_close_then_reopen() {
    let config = SimulatedCameraConfig::default()
        .with_id("FW-REOPEN")
        .with_filter_wheel(5);
    let camera = Camera::new_simulated(config);
    let fw = FilterWheel::new(camera);

    // Open
    fw.open().unwrap();
    assert!(fw.is_open().unwrap());

    // Set position
    fw.set_fw_position(2).unwrap();
    assert_eq!(fw.get_fw_position().unwrap(), 2);

    // Close
    fw.close().unwrap();
    assert!(!fw.is_open().unwrap());

    // Reopen
    fw.open().unwrap();
    assert!(fw.is_open().unwrap());

    // Position should still be accessible
    let pos = fw.get_fw_position().unwrap();
    assert!(pos < 10); // Valid position range

    fw.close().unwrap();
}
