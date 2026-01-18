//! Integration tests for QHYCCD simulation support
//!
//! These tests verify that simulated cameras work correctly without
//! requiring actual QHYCCD hardware.

#![cfg(feature = "simulation")]

mod common;
mod simulation;
