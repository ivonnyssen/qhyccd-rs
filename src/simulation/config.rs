//! Configuration for simulated cameras

use crate::{BayerMode, CCDChipArea, CCDChipInfo, Control};
use std::collections::HashMap;

/// Configuration for a simulated camera
///
/// # Example
/// ```no_run
/// use qhyccd_rs::simulation::SimulatedCameraConfig;
///
/// let config = SimulatedCameraConfig::default()
///     .with_filter_wheel(5)
///     .with_cooler();
/// ```
#[derive(Debug, Clone)]
pub struct SimulatedCameraConfig {
    /// Camera identifier (e.g., "SIM-001")
    pub id: String,
    /// Model name (e.g., "QHY178M-SIM")
    pub model: String,
    /// CCD/CMOS chip information
    pub chip_info: CCDChipInfo,
    /// Effective imaging area
    pub effective_area: CCDChipArea,
    /// Overscan area (if any)
    pub overscan_area: CCDChipArea,
    /// Supported controls with their (min, max, step) values
    pub supported_controls: HashMap<Control, (f64, f64, f64)>,
    /// Number of filter wheel slots (0 = no filter wheel)
    pub filter_wheel_slots: u32,
    /// Whether the camera has a cooler
    pub has_cooler: bool,
    /// Bayer mode for color cameras (None = mono)
    pub bayer_mode: Option<BayerMode>,
    /// Available readout modes (name, (width, height))
    pub readout_modes: Vec<(String, (u32, u32))>,
    /// Camera type code
    pub camera_type: u32,
    /// Firmware version string
    pub firmware_version: String,
}

impl Default for SimulatedCameraConfig {
    /// Creates a default configuration similar to a QHY178M
    fn default() -> Self {
        let mut supported_controls = HashMap::new();

        // Basic controls
        supported_controls.insert(Control::Gain, (0.0, 100.0, 1.0));
        supported_controls.insert(Control::Offset, (0.0, 255.0, 1.0));
        supported_controls.insert(Control::Exposure, (1.0, 3600_000_000.0, 1.0)); // 1us to 1hr
        supported_controls.insert(Control::Speed, (0.0, 2.0, 1.0));
        supported_controls.insert(Control::UsbTraffic, (0.0, 255.0, 1.0));
        supported_controls.insert(Control::TransferBit, (8.0, 16.0, 8.0));

        // Binning modes
        supported_controls.insert(Control::CamBin1x1mode, (1.0, 1.0, 1.0));
        supported_controls.insert(Control::CamBin2x2mode, (1.0, 1.0, 1.0));

        // Frame modes
        supported_controls.insert(Control::CamSingleFrameMode, (1.0, 1.0, 1.0));
        supported_controls.insert(Control::CamLiveVideoMode, (1.0, 1.0, 1.0));

        // Bit modes
        supported_controls.insert(Control::Cam8bits, (1.0, 1.0, 1.0));
        supported_controls.insert(Control::Cam16bits, (1.0, 1.0, 1.0));

        Self {
            id: "SIM-001".to_string(),
            model: "QHY-SIMULATED".to_string(),
            chip_info: CCDChipInfo {
                chip_width: 7.4,  // mm
                chip_height: 5.0, // mm
                image_width: 3072,
                image_height: 2048,
                pixel_width: 2.4,  // um
                pixel_height: 2.4, // um
                bits_per_pixel: 16,
            },
            effective_area: CCDChipArea {
                start_x: 0,
                start_y: 0,
                width: 3072,
                height: 2048,
            },
            overscan_area: CCDChipArea {
                start_x: 0,
                start_y: 0,
                width: 3072,
                height: 2048,
            },
            supported_controls,
            filter_wheel_slots: 0,
            has_cooler: false,
            bayer_mode: None,
            readout_modes: vec![("Standard".to_string(), (3072, 2048))],
            camera_type: 4010,
            firmware_version: "Firmware version: 2024_1_1".to_string(),
        }
    }
}

impl SimulatedCameraConfig {
    /// Creates a new configuration with a custom ID
    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = id.into();
        self
    }

    /// Sets the camera model name
    pub fn with_model(mut self, model: impl Into<String>) -> Self {
        self.model = model.into();
        self
    }

    /// Adds filter wheel support with the specified number of slots
    pub fn with_filter_wheel(mut self, slots: u32) -> Self {
        self.filter_wheel_slots = slots;
        if slots > 0 {
            self.supported_controls
                .insert(Control::CfwPort, (0.0, (slots - 1) as f64, 1.0));
            self.supported_controls
                .insert(Control::CfwSlotsNum, (slots as f64, slots as f64, 0.0));
        }
        self
    }

    /// Makes this a color camera with the specified Bayer pattern
    pub fn with_color(mut self, bayer_mode: BayerMode) -> Self {
        self.bayer_mode = Some(bayer_mode);
        self.supported_controls
            .insert(Control::CamColor, (bayer_mode as u32 as f64, bayer_mode as u32 as f64, 0.0));
        self.supported_controls.insert(Control::Wbr, (0.0, 255.0, 1.0));
        self.supported_controls.insert(Control::Wbb, (0.0, 255.0, 1.0));
        self.supported_controls.insert(Control::Wbg, (0.0, 255.0, 1.0));
        self
    }

    /// Adds cooler support
    pub fn with_cooler(mut self) -> Self {
        self.has_cooler = true;
        self.supported_controls
            .insert(Control::Cooler, (-40.0, 30.0, 0.1));
        self.supported_controls
            .insert(Control::CurTemp, (-40.0, 50.0, 0.1));
        self.supported_controls
            .insert(Control::CurPWM, (0.0, 255.0, 1.0));
        self.supported_controls
            .insert(Control::ManualPWM, (0.0, 255.0, 1.0));
        self
    }

    /// Sets custom chip information
    pub fn with_chip_info(mut self, chip_info: CCDChipInfo) -> Self {
        self.effective_area = CCDChipArea {
            start_x: 0,
            start_y: 0,
            width: chip_info.image_width,
            height: chip_info.image_height,
        };
        self.overscan_area = self.effective_area;
        self.chip_info = chip_info;
        self
    }

    /// Adds a readout mode
    pub fn with_readout_mode(mut self, name: impl Into<String>, width: u32, height: u32) -> Self {
        self.readout_modes.push((name.into(), (width, height)));
        self
    }

    /// Sets the firmware version string
    pub fn with_firmware_version(mut self, version: impl Into<String>) -> Self {
        self.firmware_version = version.into();
        self
    }

    /// Adds support for a control with the specified min, max, step values
    pub fn with_control(mut self, control: Control, min: f64, max: f64, step: f64) -> Self {
        self.supported_controls.insert(control, (min, max, step));
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert!(config.supported_controls.contains_key(&Control::CfwSlotsNum));
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
}
