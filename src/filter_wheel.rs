use eyre::{eyre, Result};
use tracing::error;

use crate::{Camera, Control, QHYError::*};

#[derive(Debug, PartialEq, Clone)]
/// Filter wheels are directly connected to the QHY camera
pub struct FilterWheel {
    camera: Camera,
}

/// Filter wheels are directly connected to the QHY camera and can be controlled through the camera
#[allow(unused_unsafe)]
impl FilterWheel {
    /// Creates a new instance of the filter wheel. The Sdk automatically finds all filter wheels and provides them in it's `filter_wheels()` iterator. Creating
    /// a filter wheek manually should only be needed for rare cases.
    /// # Example
    /// ```no_run
    /// use qhyccd_rs::{Sdk, Camera, FilterWheel};
    /// let fw = FilterWheel::new(Camera::new("filter wheel id from sdk".to_string()));
    /// println!("FilterWheel: {:?}", fw);
    /// ```
    pub fn new(camera: Camera) -> Self {
        Self { camera }
    }

    /// Returns the id of the filter wheel
    /// # Example
    /// ```no_run
    /// use qhyccd_rs::{Sdk,FilterWheel};
    /// let sdk = Sdk::new().expect("SDK::new failed");
    /// let fw = sdk.filter_wheels().last().expect("no filter wheel found");
    /// println!("Filter wheel id: {}", fw.id());
    /// ```
    pub fn id(&self) -> &str {
        self.camera.id()
    }

    /// Opens the filter wheel
    /// # Example
    /// ```no_run
    /// use qhyccd_rs::{Sdk,FilterWheel};
    /// let sdk = Sdk::new().expect("SDK::new failed");
    /// let fw = sdk.filter_wheels().last().expect("no filter wheel found");
    /// fw.open().expect("open failed");
    /// ```
    pub fn open(&self) -> Result<()> {
        self.camera.open()
    }

    /// Returns `true` if the filter wheel is open
    /// # Example
    /// ```no_run
    /// use qhyccd_rs::{Sdk,FilterWheel};
    /// let sdk = Sdk::new().expect("SDK::new failed");
    /// let fw = sdk.filter_wheels().last().expect("no filter wheel found");
    /// fw.open().expect("open failed");
    /// let is_open = fw.is_open();
    /// println!("Is filter wheel open: {:?}", is_open);
    /// ```
    pub fn is_open(&self) -> Result<bool> {
        self.camera.is_open()
    }

    /// Returns `true` if the filter wheel is plugged into the camera
    /// # Example
    /// ```no_run
    /// use qhyccd_rs::{Sdk,FilterWheel};
    /// let sdk = Sdk::new().expect("SDK::new failed");
    /// let fw = sdk.filter_wheels().last().expect("no filter wheel found");
    /// fw.open().expect("open failed");
    /// let is_cfw_plugged_in = fw.is_cfw_plugged_in().expect("is_cfw_plugged_in failed");
    /// println!("Is filter wheel plugged in: {}", is_cfw_plugged_in);
    /// ```
    pub fn is_cfw_plugged_in(&self) -> Result<bool> {
        self.camera.is_cfw_plugged_in()
    }

    /// Closes the filter wheel
    /// # Example
    /// ```no_run
    /// use qhyccd_rs::{Sdk,FilterWheel};
    /// let sdk = Sdk::new().expect("SDK::new failed");
    /// let fw = sdk.filter_wheels().last().expect("no filter wheel found");
    /// fw.open().expect("open failed");
    /// fw.close().expect("close failed");
    /// ```
    pub fn close(&self) -> Result<()> {
        self.camera.close()
    }

    /// Returns the number of filters in the filter wheel
    /// # Example
    /// ```no_run
    /// use qhyccd_rs::{Sdk,FilterWheel};
    /// let sdk = Sdk::new().expect("SDK::new failed");
    /// let fw = sdk.filter_wheels().last().expect("no filter wheel found");
    /// fw.open().expect("open failed");
    /// let number_of_filters = fw.get_number_of_filters().expect("get_number_of_filters failed");
    /// println!("Number of filters: {}", number_of_filters);
    /// ```
    pub fn get_number_of_filters(&self) -> Result<u32> {
        match self.camera.is_control_available(Control::CfwSlotsNum) {
            Some(_) => self.camera.get_parameter(Control::CfwSlotsNum).map_or_else(
                |e| {
                    error!(?e, "could not get number of filters from camera");
                    Err(e)
                },
                |num| Ok(num as u32),
            ),
            None => {
                tracing::debug!("I'm a filter wheel without filters. :(");
                Err(eyre!(GetNumberOfFiltersError))
            }
        }
    }

    /// Returns the current filter wheel position
    /// # Example
    /// ```no_run
    /// use qhyccd_rs::{Sdk,FilterWheel};
    /// let sdk = Sdk::new().expect("SDK::new failed");
    /// let fw = sdk.filter_wheels().last().expect("no filter wheel found");
    /// fw.open().expect("open failed");
    /// let current_position = fw.get_fw_position().expect("get_fw_position failed");
    /// println!("Current position: {}", current_position);
    /// ```
    pub fn get_fw_position(&self) -> Result<u32> {
        match self.camera.is_control_available(Control::CfwPort) {
            Some(_) => match self.camera.get_parameter(Control::CfwPort) {
                //the parameter uses ASCII values to represent the position
                Ok(position) => Ok((position - 48_f64) as u32), //removing ASCII offset
                Err(error) => {
                    tracing::error!(error = ?error);
                    Err(eyre!(error))
                }
            },
            None => {
                tracing::debug!("No filter wheel plugged in.");
                Err(eyre!(GetCfwPositionError))
            }
        }
    }

    /// Sets the current filter wheel position
    /// # Example
    /// ```no_run
    /// use qhyccd_rs::{Sdk,FilterWheel};
    /// let sdk = Sdk::new().expect("SDK::new failed");
    /// let fw = sdk.filter_wheels().last().expect("no filter wheel found");
    /// fw.open().expect("open failed");
    /// fw.set_fw_position(1).expect("set_fw_position failed");
    /// ```
    pub fn set_fw_position(&self, position: u32) -> Result<()> {
        match self.camera.is_control_available(Control::CfwPort) {
            //the parameter uses ASCII values to represent the position
            Some(_) => self
                .camera
                .set_parameter(Control::CfwPort, (position + 48_u32) as f64) //adding ASCII offset
                .map_err(|_| {
                    let error = SetCfwPositionError;
                    tracing::error!(error = ?error);
                    eyre!(error)
                }),
            None => {
                tracing::debug!("No filter wheel plugged in.");
                Err(eyre!(SetCfwPositionError))
            }
        }
    }
}
