use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use super::param::ParamKind;
use super::Device;

/// Device kind type.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeviceKind {
    pub name: String,
    pub hash: i64,
    pub params: Vec<ParamKind>,
}

/// Shortcut type for HashMap<String, DeviceKind>.
pub type DeviceKinds = HashMap<String, DeviceKind>;

impl DeviceKind {
    /// Make a new device of this kind.
    pub fn make(&self) -> Device {
        let name = self.name.clone();
        let hash = self.hash;
        let params = self.params.iter().map(|pk| pk.make()).collect();
        Device { name, hash, params }
    }
}

