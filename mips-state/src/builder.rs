//! MIPS state builder utility.
use std::collections::HashMap;

use crate::{Device, DeviceKind, Alias, MipsState};

/// Utility struct for building a MIPS state object.
pub struct MipsStateBuilder {
    mem_size: usize,
    dev_size: usize,
    mem: Option<Vec<f32>>,
    dev: Option<Vec<Device>>,
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

    pub fn with_dev(mut self, mut dev: Vec<Device>) -> Self {
        dev.resize(self.dev_size, Device::Unset);
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

