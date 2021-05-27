//! Device and logic parameter types.
use std::collections::HashMap;
use std::io::Error as IoError;
use std::path::PathBuf;
use std::{fmt, fmt::Display};

use ron::de::Error as RonError;
use serde::Deserialize;

use super::params::{Param, ParamKind, Params};

/// Device kind type.
#[derive(Clone, Debug, Deserialize)]
pub struct DeviceKind {
    pub name: String,
    pub hash: i64,
    pub params: Vec<ParamKind>,
}

/// Shortcut type for HashMap<String, DeviceKind>.
// #[allow(dead_code)]
pub type DeviceKinds = HashMap<String, DeviceKind>;

/// Device kinds deserialization error kinds.
pub enum DeserializeError {
    RonError(RonError),
    IoError(IoError),
}

macro_rules! impl_from_error {
    ($T:ty, $($E:tt),*) => {
        $( impl From<$E> for $T { fn from(e: $E) -> Self { <$T>::$E(e) } })*
    }
}

impl_from_error!(DeserializeError, IoError, RonError);

impl DeviceKind {
    /// Make a new device of this kind.
    pub fn make(&self) -> Device {
        let params = self.params.iter().map(|pk| pk.make()).collect();
        Device { kind: self, params }
    }

    /// Try to deserialize `DeviceKinds` from a Ron string.
    pub fn from_ron_str(s: &str) -> Result<DeviceKinds, DeserializeError> {
        let kinds = ron::de::from_str(s)?;
        Ok(kinds)
    }

    /// Try to deserialize `DeviceKinds` from a Ron file.
    pub fn from_ron<P: Into<PathBuf>>(p: P) -> Result<DeviceKinds, DeserializeError> {
        let reader = std::fs::File::open(p.into())?;
        let kinds = ron::de::from_reader(reader)?;
        Ok(kinds)
    }
}

#[derive(Clone, Debug)]
pub enum DeviceError {
    Unset,
    UnknownParam(String),
    ReadOnly,
    WriteOnly,
}

/// Device type.
#[derive(Clone, Debug)]
pub struct Device<'dk> {
    pub kind: &'dk DeviceKind,
    pub params: Params,
}

impl<'dk> Display for Device<'dk> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_fmt(format_args!("{} {{\n", self.kind.name))?;
        for (k, v) in self.params.iter() {
            f.write_fmt(format_args!(
                "    {}: {} ({})\n",
                k,
                v.read_internal(),
                match v {
                    Param::Read(_) => "R",
                    Param::Write(_) => "W",
                    Param::ReadWrite(_) => "RW",
                }
            ))?;
        }
        f.write_str("}")
    }
}

impl<'dk> Device<'dk> {
    pub fn name(&self) -> &'dk String {
        &self.kind.name
    }

    /// Try to get a specified parameter reference.
    pub fn get(&self, key: &String) -> Result<&Param, DeviceError> {
        self.params
            .get(key)
            .ok_or(DeviceError::UnknownParam(key.clone()))
    }

    /// Try to get a mutable specified parameter reference.
    pub fn get_mut(&mut self, key: &String) -> Result<&mut Param, DeviceError> {
        self.params
            .get_mut(key)
            .ok_or(DeviceError::UnknownParam(key.clone()))
    }

    /// Read from a parameter.
    ///
    /// Fails if a parameter for the key does not exist, or if the parameter kind is write only.
    pub fn read<K>(&self, key: K) -> Result<f64, DeviceError>
    where
        K: Into<String>,
    {
        self.get(&key.into()).map(Param::read).flatten()
    }

    /// Read from a parameter, ignoring the parameter kind.
    ///
    /// Fails if a parameter for the key does not exist.
    pub fn read_internal<K: Into<String>>(&self, key: K) -> Result<f64, DeviceError> {
        self.get(&key.into()).map(Param::read_internal)
    }

    /// Write to a parameter.
    ///
    /// Fails if a parameter for the key does not exist, or if the parameter kind is read only.
    pub fn write<K, V>(&mut self, key: K, val: V) -> Result<(), DeviceError>
    where
        K: Into<String>,
        V: Into<f64>,
    {
        self.get_mut(&key.into())
            .map(|p| p.write(val.into()))
            .flatten()
    }

    /// Write to a parameter, ignoring the parameter kind.
    ///
    /// Fails if a parameter for the key does not exist.
    pub fn write_internal<K, V>(&mut self, key: K, val: V) -> Result<(), DeviceError>
    where
        K: Into<String>,
        V: Into<f64>,
    {
        self.get_mut(&key.into())
            .map(|p| p.write_internal(val.into()))
    }
}
