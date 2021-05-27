//! Device and device logic parameter types.
mod devices;
mod params;

pub use devices::{DeserializeError, Device, DeviceError, DeviceKind, DeviceKinds};
pub use params::{Param, ParamKind};
