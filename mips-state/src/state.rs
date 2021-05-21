//! MIPS state simulator.
use std::collections::HashMap;

use thiserror::Error;

use crate::{Device, DeviceKind};
use mips_parser::ast::Register;

/// Alias type enumeration.
#[derive(Copy, Clone)]
pub enum Alias {
    Mem(usize),
    Dev(usize),
}

/// State error enumeration.
#[derive(Clone, PartialEq, Debug, Error)]
pub enum StateError {
    #[error("Cannot find value a from register of type {0}")]
    InvalidRegister(Register),
    #[error("Invalid alias {0}")]
    InvalidAlias(String),
    #[error("Invalid memory index {0}")]
    InvalidMemoryIndex(usize),
}

/// MIPS state struct.
pub struct MipsState {
    pub mem: Vec<f32>,
    pub dev: Vec<Device>,
    pub aliases: HashMap<String, Alias>,
}

impl MipsState {
    /// New MIPS state with initialized memory values, device values, and aliases.
    pub fn new(mem: Vec<f32>, dev: Vec<Device>, aliases: HashMap<String, Alias>) -> Self {
        Self { mem, dev, aliases }
    }

    /// Get memory value from register.
    pub fn value(&self, reg: &Register) -> Result<f32, StateError> {
        match reg {
            Register::Alias(k) => {
                let a = self
                    .aliases
                    .get(k)
                    .ok_or(StateError::InvalidAlias(k.clone()))?;
                if let &Alias::Mem(i) = a {
                    self.mem
                        .get(i)
                        .cloned()
                        .ok_or(StateError::InvalidMemoryIndex(i))
                } else {
                    Err(StateError::InvalidAlias(k.clone()))
                }
            }
            Register::Memory(r) => match r.as_ref() {
                Register::Memory(_) => self.value(r).and_then(|x| {
                    let i = x as usize;
                    self.mem
                        .get(i)
                        .cloned()
                        .ok_or(StateError::InvalidMemoryIndex(i))
                }),
                _ => self.value(r),
            },
            _ => Err(StateError::InvalidRegister(reg.clone())),
        }
    }

    pub fn device(&self, reg: &Register) -> Result<&Device, StateError> {
        match reg {
            Register::Alias(k) => {
                let a = self
                    .aliases
                    .get(k)
                    .ok_or(StateError::InvalidAlias(k.clone()))?;
                if let &Alias::Dev(i) = a {
                    self.dev.get(i).ok_or(StateError::InvalidMemoryIndex(i))
                } else {
                    Err(StateError::InvalidAlias(k.clone()))
                }
            }
            Register::Device(_r) => self.dev.get(0).ok_or(StateError::InvalidMemoryIndex(0)),
            _ => Err(StateError::InvalidRegister(reg.clone())),
        }
    }
}
