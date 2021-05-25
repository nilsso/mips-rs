#![feature(result_flattening)]
#![allow(unused_imports)]
#![allow(unused_mut)]
// use mips_parser::prelude::*;
// use mips_simulator::prelude::*;
use std::collections::HashMap;
use std::{fmt, fmt::Debug};

use maplit::hashmap;

// use mips_simulator::device::*;

use ron::ser::to_string as to_ron_string;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
pub enum DeviceError {
    Unset,
    UnknownParam(String),
    ReadOnly,
    WriteOnly,
}

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

/// Parameter type.
#[derive(Clone)]
pub enum Param {
    Read(f64),
    Write(f64),
    ReadWrite(f64),
}

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

fn main() {
    // let d = DeviceKind {
    //     name: "SolarPanel".into(),
    //     hash: -2045627372,
    //     params: vec![ParamKind::Read("a".into())],
    // };
    // let kinds = hashmap!{ "SolarPanel".to_string() => d };
    // println!("{}", to_ron_string(&kinds).unwrap());

    let file = std::fs::File::open("./device-kinds.ron").unwrap();
    let device_kinds: HashMap<String, DeviceKind> = ron::de::from_reader(file).unwrap();
    let mut d = device_kinds["Autolathe"].make();

    println!("{:?}", d.read("ClearMemory"));
    println!("{:#?}", d);

    // let k: HashMap<String, DeviceKind> = devices
    //     .into_iter()
    //     .map(|d| (d.name.clone(), d)).collect();
}
