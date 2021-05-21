#![allow(dead_code)]

use std::collections::HashMap;

pub type Parameters = HashMap<String, f32>;

#[derive(Clone, Debug)]
pub struct DeviceKind {
    name: String,
    logic_parameters: Parameters,
    slot_parameters: Parameters,
    reagent_parameters: Parameters,
}

impl DeviceKind {
    pub fn logic(&self, k: &String) -> Option<&f32> {
        self.logic_parameters.get(k)
    }

    pub fn slot(&self, k: &String) -> Option<&f32> {
        self.slot_parameters.get(k)
    }

    pub fn reagent(&self, k: &String) -> Option<&f32> {
        self.reagent_parameters.get(k)
    }
}

#[derive(Clone, Debug)]
pub enum Device {
    Unset,
    Device(DeviceKind),
}

impl Device {
    pub fn device(&self) -> Option<&DeviceKind> {
        match self {
            Device::Unset => None,
            Device::Device(d) => Some(d),
        }
    }
}

impl Default for Device {
    fn default() -> Device {
        Device::Unset
    }
}
