//! Parameter and parameter kind types.
use std::collections::HashMap;
use std::{fmt, fmt::Display};

use serde::{Deserialize, Serialize};

use super::DeviceError;

/// Parameter kind type.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum ParamKind {
    Read(String),
    Write(String),
    ReadWrite(String),
}

impl ParamKind {
    /// Make a new parameter pair with the key of this parameter kind.
    pub fn make(&self) -> (String, Param) {
        match self {
            ParamKind::Read(k) => (k.clone(), Param::Read(0_f64)),
            ParamKind::Write(k) => (k.clone(), Param::Write(0_f64)),
            ParamKind::ReadWrite(k) => (k.clone(), Param::ReadWrite(0_f64)),
        }
    }
}

impl Display for ParamKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParamKind::Read(key) => write!(f, "{} (R)", key),
            ParamKind::Write(key) => write!(f, "{} (W)", key),
            ParamKind::ReadWrite(key) => write!(f, "{} (RW)", key),
        }
    }
}

/// Parameter type.
#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
pub enum Param {
    Read(f64),
    Write(f64),
    ReadWrite(f64),
}

/// Shortcut for `HashMap<String, Param>`.
pub type Params = HashMap<String, Param>;

impl Param {
    pub(crate) fn value(&self) -> f64 {
        use Param::*;
        match self {
            Read(v) => *v,
            Write(v) => *v,
            ReadWrite(v) => *v,
        }
    }

    /// Read parameter value.
    ///
    /// Fails if the paramter kind is write only.
    pub fn read(&self) -> Result<f64, DeviceError> {
        use Param::*;
        match self {
            Read(v) | ReadWrite(v) => Ok(*v),
            Write(_) => Err(DeviceError::ParamWriteOnly),
        }
    }

    /// Write parameter value.
    ///
    /// Fails if the paramter kind is read only.
    pub fn write(&mut self, val: f64) -> Result<(), DeviceError> {
        use Param::*;
        match self {
            Write(v) | ReadWrite(v) => Ok(*v = val),
            Read(_) => Err(DeviceError::ParamReadOnly),
        }
    }
}

impl Display for Param {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Param::Read(v) => write!(f, "{} (R)", v),
            Param::Write(v) => write!(f, "{} (W)", v),
            Param::ReadWrite(v) => write!(f, "{} (RW)", v),
        }
    }
}
