//! Parameter and parameter kind types.
use std::collections::HashMap;
use std::{fmt, fmt::Debug};

use serde::{Deserialize, Serialize};

use super::DeviceError;

/// Parameter kind type.
#[derive(Clone, Serialize, Deserialize)]
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

impl Debug for ParamKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ParamKind::*;
        match self {
            Read(s) => write!(f, r#"Read("{}")"#, s),
            Write(s) => write!(f, r#"Write("{}")"#, s),
            ReadWrite(s) => write!(f, r#"ReadWrite("{}")"#, s),
        }
    }
}

/// Parameter type.
#[derive(Clone)]
pub enum Param {
    Read(f64),
    Write(f64),
    ReadWrite(f64),
}

/// Shortcut for `HashMap<String, Param>`.
pub type Params = HashMap<String, Param>;

impl Debug for Param {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Param::*;
        match self {
            Read(v) => write!(f, "Read({})", v),
            Write(v) => write!(f, "Write({})", v),
            ReadWrite(v) => write!(f, "ReadWrite({})", v),
        }
    }
}

impl Param {
    /// Read parameter value.
    ///
    /// Fails if the paramter kind is write only.
    pub fn read(&self) -> Result<f64, DeviceError> {
        use Param::*;
        match self {
            Read(v) | ReadWrite(v) => Ok(*v),
            Write(_) => Err(DeviceError::WriteOnly),
        }
    }

    /// Read parameter value, ignoring the parameter kind.
    pub fn read_internal(&self) -> f64 {
        use Param::*;
        match self {
            Read(v) | Write(v) | ReadWrite(v) => *v,
        }
    }

    /// Write parameter value.
    ///
    /// Fails if the paramter kind is read only.
    pub fn write(&mut self, val: f64) -> Result<(), DeviceError> {
        use Param::*;
        match self {
            Write(v) | ReadWrite(v) => Ok(*v = val),
            Read(_) => Err(DeviceError::ReadOnly),
        }
    }

    /// Write parameter value, ignoring the parameter kind.
    pub fn write_internal(&mut self, val: f64) {
        use Param::*;
        match self {
            Read(v) | Write(v) | ReadWrite(v) => *v = val,
        }
    }
}
