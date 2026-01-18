use std::sync::{Arc, RwLock};

use eyre::{eyre, Result, WrapErr};

#[cfg(feature = "simulation")]
use crate::simulation::SimulatedCameraState;
use crate::QHYError::CameraNotOpenError;

#[derive(Debug, PartialEq, Copy, Clone)]
pub(crate) struct QHYCCDHandle {
    pub ptr: *const std::ffi::c_void,
}

//Safety: QHYCCDHandle is only used in Camera and Camera is Send and Sync
unsafe impl Send for QHYCCDHandle {}
unsafe impl Sync for QHYCCDHandle {}

/// Internal backend for camera operations
#[derive(Debug)]
pub(crate) enum CameraBackend {
    /// Real hardware camera using FFI calls
    Real {
        handle: Arc<RwLock<Option<QHYCCDHandle>>>,
    },
    /// Simulated camera for testing
    #[cfg(feature = "simulation")]
    Simulated {
        state: Arc<RwLock<SimulatedCameraState>>,
    },
}

impl Clone for CameraBackend {
    fn clone(&self) -> Self {
        match self {
            CameraBackend::Real { handle } => CameraBackend::Real {
                handle: Arc::clone(handle),
            },
            #[cfg(feature = "simulation")]
            CameraBackend::Simulated { state } => CameraBackend::Simulated {
                state: Arc::clone(state),
            },
        }
    }
}

impl PartialEq for CameraBackend {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (CameraBackend::Real { .. }, CameraBackend::Real { .. }) => true,
            #[cfg(feature = "simulation")]
            (CameraBackend::Simulated { .. }, CameraBackend::Simulated { .. }) => true,
            #[allow(unreachable_patterns)]
            _ => false,
        }
    }
}

macro_rules! read_lock {
    ($var:expr, $wrap:expr) => {{
        use eyre::WrapErr as _;
        $var.read()
            .map_err(|err| {
                tracing::error!(error = ?err);
                eyre!("Could not acquire read lock on camera handle")
            })
            .and_then(|lock| match *lock {
                Some(handle) => Ok(handle.ptr),
                None => {
                    tracing::error!(error = ?CameraNotOpenError);
                    Err(eyre!(CameraNotOpenError))
                }
            })
            .wrap_err($wrap)
    }};
}

pub(crate) use read_lock;
