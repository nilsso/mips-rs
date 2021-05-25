use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
pub enum DeviceError {
    Unset,
    UnknownParam(String),
    ReadOnly,
    WriteOnly,
}

mod param;
pub use param::{Param, ParamKind};

/// Device kind.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeviceKind {
    pub name: String,
    pub hash: i64,
    pub params: Vec<ParamKind>,
}

impl DeviceKind {
    /// Make a new device of this kind.
    pub fn make(&self) -> Device {
        let params = self.params.iter().map(|pk| pk.make()).collect();
        Device { kind: self, params }
    }
}

#[derive(Clone, Debug)]
pub struct Device<'a> {
    pub kind: &'a DeviceKind,
    pub params: HashMap<String, Param>,
}

impl<'a> Device<'a> {
    /// Try to get a specified parameter reference.
    pub fn try_get(&self, key: &String) -> Result<&Param, DeviceError> {
        self.params
            .get(key)
            .ok_or(DeviceError::UnknownParam(key.clone()))
    }

    /// Try to get a mutable specified parameter reference.
    pub fn try_get_mut(&mut self, key: &String) -> Result<&mut Param, DeviceError> {
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
        self.try_get(&key.into()).map(Param::read).flatten()
    }

    /// Read from a parameter, ignoring the parameter kind.
    ///
    /// Fails if a parameter for the key does not exist.
    pub fn read_internal<K: Into<String>>(&'a self, key: K) -> Result<f64, DeviceError> {
        self.try_get(&key.into()).map(Param::read_internal)
    }

    /// Write to a parameter.
    ///
    /// Fails if a parameter for the key does not exist, or if the parameter kind is read only.
    pub fn write<K, V>(&mut self, key: K, val: V) -> Result<(), DeviceError>
    where
        K: Into<String>,
        V: Into<f64>,
    {
        self.try_get_mut(&key.into())
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
        self.try_get_mut(&key.into())
            .map(|p| p.write_internal(val.into()))
    }
}
