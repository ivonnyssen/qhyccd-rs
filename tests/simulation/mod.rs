//! Integration tests for simulated cameras
//!
//! These tests verify that simulated cameras work correctly without
//! requiring actual QHYCCD hardware.

mod camera_tests;
mod config_tests;
mod image_generator_tests;
// Note: state_tests remain in src/simulation/test_state.rs because
// SimulatedCameraState is pub(crate) and can't be tested from here
