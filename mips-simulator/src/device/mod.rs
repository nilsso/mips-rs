//! Device and device logic parameter types.
#[derive(Clone, Debug)]
pub enum DeviceError {
    Unset,
    UnknownParam(String),
    ReadOnly,
    WriteOnly,
}

mod params;
mod devices;

pub use params::{Param, ParamKind};
pub use devices::{Device, DeviceKind};

