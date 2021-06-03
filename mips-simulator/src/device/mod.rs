//! Device and device logic parameter types.
use std::{fmt, fmt::Display};

use maplit::hashmap;
use serde::{Deserialize, Serialize};

mod device_kind;
mod param;

pub use device_kind::{DeviceKind, DeviceKinds};
pub use param::{Param, ParamKind, Params};

#[derive(Clone, Debug)]
pub enum DeviceError {
    Unset,
    ParamUnknown(String),
    ParamReadOnly,
    ParamWriteOnly,
}

/// Device type.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Device {
    pub name: String,
    pub hash: i64,
    pub params: Params,
}

impl Device {
    /// Try to get a specified parameter reference.
    pub fn try_get_param(&self, param: &String) -> Result<&Param, DeviceError> {
        self.params
            .get(param)
            .ok_or(DeviceError::ParamUnknown(param.clone()))
    }

    /// Try to get a mutable specified parameter reference.
    pub fn try_get_mut_param(&mut self, param: &String) -> Result<&mut Param, DeviceError> {
        self.params
            .get_mut(param)
            .ok_or(DeviceError::ParamUnknown(param.clone()))
    }

    /// Read from a parameter.
    ///
    /// Fails if a parameter for the key does not exist, or if the parameter kind is write only.
    pub fn read<K>(&self, param: K) -> Result<f64, DeviceError>
    where
        K: Into<String>,
    {
        self.try_get_param(&param.into()).map(Param::read).flatten()
    }

    /// Write to a parameter.
    ///
    /// Fails if a parameter for the key does not exist, or if the parameter kind is read only.
    pub fn write<K, V>(&mut self, param: K, val: V) -> Result<(), DeviceError>
    where
        K: Into<String>,
        V: Into<f64>,
    {
        self.try_get_mut_param(&param.into())
            .map(|p| p.write(val.into()))
            .flatten()
    }

    /// Construct a new Stationeers circuit housing device
    ///
    /// Used for the default state self device.
    pub fn circuit_housing() -> Device {
        Device {
            name: "CircuitHousing".to_string(),
            hash: -128473777,
            params: hashmap! {
                "On".into()            => Param::ReadWrite(0.0),
                "RequiredPower".into() => Param::Read(0.0),
                "Activate".into()      => Param::ReadWrite(0.0),
                "PrefabHash".into()    => Param::Read(0.0),
                "Error".into()         => Param::Read(0.0),
                "Setting".into()       => Param::ReadWrite(0.0),
                "Power".into()         => Param::Read(0.0)
            },
        }
    }
}

impl Display for Device {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_fmt(format_args!("{} {{\n", self.name))?;
        for (key, param) in self.params.iter() {
            f.write_fmt(format_args!(
                "    {}: {} ({})\n",
                key,
                param.value(),
                match param {
                    Param::Read(_) => "R",
                    Param::Write(_) => "W",
                    Param::ReadWrite(_) => "RW",
                }
            ))?;
        }
        f.write_str("}")
    }
}
