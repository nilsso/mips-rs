use std::collections::HashMap;
use std::{fmt, fmt::Debug};

use serde::{Deserializer, Deserialize, Serialize};
use serde::de::{Visitor, MapAccess};

#[derive(Serialize, Deserialize, Debug)]
pub enum ParamKind {
    Read(String),
    Write(String),
    ReadWrite(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AnonymousParams {
    read: Vec<String>,
    write: Vec<String>,
}

impl Into<(Vec<String>, Vec<String>)> for AnonymousParams {
    fn into(self) -> (Vec<String>, Vec<String>) {
        (self.read, self.write)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct AnonymousDeviceKind {
    pub hash: i64,
    pub logicTypes: AnonymousParams,
}

impl Into<(i64, (Vec<String>, Vec<String>))> for AnonymousDeviceKind {
    fn into(self) -> (i64, (Vec<String>, Vec<String>)) {
        (self.hash, self.logicTypes.into())
    }
}

#[derive(Serialize, Debug)]
pub struct DeviceKind {
    pub name: String,
    pub hash: i64,
    pub params: Vec<ParamKind>,
}

#[derive(Serialize, Debug)]
pub struct DeviceKinds(pub HashMap<String, DeviceKind>);

impl<'de> Deserialize<'de> for DeviceKinds {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {

        struct DeviceKindVisitor;

        impl<'de> Visitor<'de> for DeviceKindVisitor {
            type Value = Vec<(String, DeviceKind)>;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("a DeviceKind string and map pair")
            }

            fn visit_map<V: MapAccess<'de>>(self, mut map: V) -> Result<Vec<(String, DeviceKind)>, V::Error> {
                let mut device_kinds = Vec::new();

                let prefix = regex::Regex::new(r"Structure|Item").unwrap();
                while let Some(name) = map.next_key::<String>()? {
                    if let Ok(annonymous_dk) = map.next_value::<AnonymousDeviceKind>() {
                        let name = if let Some(re_match) = prefix.find(&name) {
                            let e = re_match.end();
                            name[e..].to_string()
                        } else {
                            name.to_string()
                        };

                        let (hash, (read, write)) = annonymous_dk.into();
                        let mut params: HashMap<String, ParamKind> = HashMap::new();
                        // First collect the Read marked params...
                        for s in read.into_iter() {
                            let param_kind = ParamKind::Read(s.clone());
                            params.insert(s, param_kind);
                        }
                        // Then the Write marked...
                        for s in write.into_iter() {
                            let param_kind = if params.contains_key(&s) {
                                // ReadWrite if already marked Read...
                                ParamKind::ReadWrite(s.clone())
                            } else {
                                // else Write.
                                ParamKind::Write(s.clone())
                            };
                            params.insert(s, param_kind);
                        }

                        let params = params.into_values().collect();
                        let device_kind = DeviceKind {
                            name: name.clone(),
                            hash,
                            params,
                        };
                        device_kinds.push((name, device_kind));
                    }
                }

                Ok(device_kinds)
            }
        }

        let device_kinds = deserializer
            .deserialize_struct("AnonymousDeviceKind", &[], DeviceKindVisitor)?;
        Ok(DeviceKinds(device_kinds.into_iter().collect()))
    }
}

