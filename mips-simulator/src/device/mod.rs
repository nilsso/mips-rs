use serde::{Serialize, Deserialize};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum DeviceParam {
    Logic,
    Slot,
    Reagent,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct DeviceKind {
    name: String,
    params: Vec<DeviceParam>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Device;
