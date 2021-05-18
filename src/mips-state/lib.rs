//! IC10 state simulator.

use std::collections::HashMap;

use thiserror::Error;

use crate::ast::Register;

#[derive(Copy, Clone)]
pub enum Alias {
    Mem(usize),
    Dev(usize),
}

#[derive(Clone, PartialEq, Debug, Error)]
pub enum StateError {
    #[error("Cannot find value a from register of type {0}")]
    InvalidRegister(Register),
    #[error("Invalid alias {0}")]
    InvalidAlias(String),
    #[error("Invalid memory index {0}")]
    InvalidMemoryIndex(usize),
}

pub struct MipsState {
    pub mem: Vec<f32>,
    pub dev: Vec<f32>,
    pub aliases: HashMap<String, Alias>,
}

impl MipsState {
    pub fn new(mem: Vec<f32>, dev: Vec<f32>, aliases: HashMap<String, Alias>) -> Self {
        Self { mem, dev, aliases }
    }

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
}

pub struct MipsStateBuilder {
    mem_size: usize,
    dev_size: usize,
    mem: Option<Vec<f32>>,
    dev: Option<Vec<f32>>,
    aliases: HashMap<String, Alias>,
}

impl MipsStateBuilder {
    pub fn new(mem_size: usize, dev_size: usize) -> Self {
        Self {
            mem_size,
            dev_size,
            mem: None,
            dev: None,
            aliases: HashMap::new(),
        }
    }

    pub fn with_mem(mut self, mut mem: Vec<f32>) -> Self {
        mem.resize(self.mem_size, 0_f32);
        self.mem = Some(mem);
        self
    }

    pub fn with_dev(mut self, mut dev: Vec<f32>) -> Self {
        dev.resize(self.dev_size, 0_f32);
        self.dev = Some(dev);
        self
    }

    pub fn with_aliases(mut self, aliases: HashMap<String, Alias>) -> Self {
        self.aliases = aliases;
        self
    }

    pub fn with_default_aliases(mut self) -> Self {
        for (k, v) in (0..self.mem_size)
            .map(|i| (format!("r{}", i), Alias::Mem(i)))
            .chain((0..self.dev_size).map(|i| (format!("d{}", i), Alias::Dev(i))))
        {
            self.aliases.insert(k, v);
        }
        self
    }

    pub fn with_alias(mut self, k: String, v: Alias) -> Self {
        self.aliases.insert(k, v);
        self
    }

    pub fn build(self) -> MipsState {
        let (m, d) = (self.mem_size, self.dev_size);
        let mem = self.mem.unwrap_or(vec![Default::default(); m]);
        let dev = self.dev.unwrap_or(vec![Default::default(); d]);
        let aliases = self.aliases;
        MipsState { mem, dev, aliases }
    }
}

impl Default for MipsState {
    fn default() -> Self {
        MipsStateBuilder::new(16, 6).with_default_aliases().build()
    }
}
