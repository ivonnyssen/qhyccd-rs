//! Unit tests requiring FFI mocking
//!
//! These tests use mockall to mock the QHYCCD FFI layer. They must remain
//! in the src/ directory because mockall's `#[automock]` attribute generates
//! mock contexts at compile time that must be in the same crate.

mod camera_tests;
mod filter_wheel_tests;
mod sdk_tests;
