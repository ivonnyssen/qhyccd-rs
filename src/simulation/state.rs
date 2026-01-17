//! Runtime state for simulated cameras

use crate::{CCDChipArea, Control, StreamMode};
use std::collections::HashMap;
use std::time::Instant;

use super::SimulatedCameraConfig;

/// Runtime state for a simulated camera
#[derive(Debug)]
pub struct SimulatedCameraState {
    /// Camera configuration (immutable reference data)
    pub config: SimulatedCameraConfig,
    /// Whether the camera is currently open
    pub is_open: bool,
    /// Whether the camera has been initialized
    pub is_initialized: bool,
    /// Current stream mode
    pub stream_mode: Option<StreamMode>,
    /// Current parameter values
    pub parameters: HashMap<Control, f64>,
    /// Current ROI settings
    pub roi: CCDChipArea,
    /// Current binning (x, y)
    pub binning: (u32, u32),
    /// Current bit depth for transfers
    pub bit_depth: u32,
    /// Current readout mode index
    pub readout_mode: u32,
    /// Whether live mode is active
    pub live_mode_active: bool,
    /// Exposure start time (for single frame mode)
    pub exposure_start: Option<Instant>,
    /// Exposure duration in microseconds (for single frame mode)
    pub exposure_duration_us: u64,
    /// Current filter wheel position (0-indexed)
    pub filter_wheel_position: u32,
    /// Current target temperature for cooler
    pub target_temperature: f64,
    /// Current actual temperature (simulated)
    pub current_temperature: f64,
    /// Current cooler PWM
    pub cooler_pwm: f64,
    /// Debayer enabled
    pub debayer_enabled: bool,
}

impl SimulatedCameraState {
    /// Creates a new state from a configuration
    pub fn new(config: SimulatedCameraConfig) -> Self {
        let roi = config.effective_area;
        let bit_depth = config.chip_info.bits_per_pixel;

        // Initialize parameters with default values (middle of range)
        let mut parameters = HashMap::new();
        for (control, (min, max, _step)) in &config.supported_controls {
            let default = match control {
                Control::Gain => 0.0,
                Control::Offset => 10.0,
                Control::Exposure => 1000.0, // 1ms default
                Control::Speed => 0.0,
                Control::UsbTraffic => 50.0,
                Control::TransferBit => 16.0,
                Control::CfwPort => 0.0,
                Control::CfwSlotsNum => config.filter_wheel_slots as f64,
                Control::CurTemp => 20.0, // Room temperature
                Control::CurPWM => 0.0,
                Control::Cooler => 20.0,
                Control::ManualPWM => 0.0,
                _ => (*min + *max) / 2.0,
            };
            parameters.insert(*control, default);
        }

        Self {
            config,
            is_open: false,
            is_initialized: false,
            stream_mode: None,
            parameters,
            roi,
            binning: (1, 1),
            bit_depth,
            readout_mode: 0,
            live_mode_active: false,
            exposure_start: None,
            exposure_duration_us: 1000,
            filter_wheel_position: 0,
            target_temperature: 20.0,
            current_temperature: 20.0,
            cooler_pwm: 0.0,
            debayer_enabled: false,
        }
    }

    /// Gets the current image dimensions accounting for ROI and binning
    pub fn get_current_image_dimensions(&self) -> (u32, u32) {
        let width = self.roi.width / self.binning.0;
        let height = self.roi.height / self.binning.1;
        (width, height)
    }

    /// Gets the number of bytes per pixel based on current bit depth
    pub fn get_bytes_per_pixel(&self) -> u32 {
        if self.bit_depth <= 8 {
            1
        } else {
            2
        }
    }

    /// Gets the number of channels (1 for mono, 3 for color with debayer)
    pub fn get_channels(&self) -> u32 {
        if self.config.bayer_mode.is_some() && self.debayer_enabled {
            3
        } else {
            1
        }
    }

    /// Calculates the required buffer size for the current settings
    pub fn calculate_buffer_size(&self) -> usize {
        let (width, height) = self.get_current_image_dimensions();
        let bytes_per_pixel = self.get_bytes_per_pixel();
        let channels = self.get_channels();
        (width * height * bytes_per_pixel * channels) as usize
    }

    /// Returns the remaining exposure time in microseconds
    pub fn get_remaining_exposure_us(&self) -> u32 {
        match self.exposure_start {
            Some(start) => {
                let elapsed_us = start.elapsed().as_micros() as u64;
                if elapsed_us >= self.exposure_duration_us {
                    0
                } else {
                    (self.exposure_duration_us - elapsed_us) as u32
                }
            }
            None => 0,
        }
    }

    /// Checks if the current exposure is complete
    pub fn is_exposure_complete(&self) -> bool {
        match self.exposure_start {
            Some(start) => {
                let elapsed_us = start.elapsed().as_micros() as u64;
                elapsed_us >= self.exposure_duration_us
            }
            None => true,
        }
    }

    /// Starts an exposure
    pub fn start_exposure(&mut self) {
        self.exposure_start = Some(Instant::now());
        // Get exposure time from parameters
        if let Some(&exposure_us) = self.parameters.get(&Control::Exposure) {
            self.exposure_duration_us = exposure_us as u64;
        }
    }

    /// Cancels the current exposure
    pub fn cancel_exposure(&mut self) {
        self.exposure_start = None;
    }

    /// Updates the simulated temperature (call periodically for realistic behavior)
    #[allow(dead_code)]
    pub fn update_temperature(&mut self) {
        if self.config.has_cooler && self.cooler_pwm > 0.0 {
            // Simple simulation: temperature approaches target based on PWM
            let cooling_rate = self.cooler_pwm / 255.0 * 0.1; // Max 0.1C per update
            if self.current_temperature > self.target_temperature {
                self.current_temperature =
                    (self.current_temperature - cooling_rate).max(self.target_temperature);
            }
        } else {
            // Warm up towards ambient (20C)
            if self.current_temperature < 20.0 {
                self.current_temperature = (self.current_temperature + 0.05).min(20.0);
            }
        }
        // Update the parameter
        self.parameters
            .insert(Control::CurTemp, self.current_temperature);
    }
}
