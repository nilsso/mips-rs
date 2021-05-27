//! Integrated Circuit (IC10) simulator state.
#![allow(dead_code)]
use std::collections::HashMap;
use std::convert::TryFrom;
use std::{fmt, fmt::Debug, fmt::Display};

use mips_parser::prelude::*;
use util::{impl_from_error, is_as_inner};

use std::num::TryFromIntError;

use super::device::{Device, DeviceError};

/// Alias kind type.
#[derive(Clone, PartialEq)]
pub enum AliasKind {
    MemId(usize),
    DevId(usize),
    Label(usize),
    Def(f64),
}

impl AliasKind {
    #[rustfmt::skip]
    is_as_inner!(AliasKind, ICStateError, ICStateError::AliasWrongKind, [
        (AliasKind::MemId, is_mem_id, as_mem_id, mem_id, &usize),
        (AliasKind::DevId, is_dev_id, as_dev_id, dev_id, &usize),
        (AliasKind::Label, is_label,  as_label,  label,  &usize),
        (AliasKind::Def,   is_def,    as_def,    def,    &f64),
    ]);
}

impl Display for AliasKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use AliasKind::*;
        match self {
            MemId(i) => write!(f, "MemId({})", i),
            DevId(i) => write!(f, "DevId({})", i),
            Label(i) => write!(f, "Label({})", i),
            Def(v) => write!(f, "Def({})", v),
        }
    }
}

impl Debug for AliasKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_fmt(format_args!("{}", self))
    }
}

pub enum BatchMode {
    Avg,
    Sum,
    Min,
    Max,
}

#[derive(Debug)]
pub enum UnknownConstantError {
    BatchMode(f64),
}

impl TryFrom<f64> for BatchMode {
    // TODO: Maybe come up with a different error kind? General mode/int constant type?
    type Error = UnknownConstantError;

    fn try_from(v: f64) -> Result<BatchMode, UnknownConstantError> {
        use BatchMode::*;
        let mode = match v as i32 {
            0 => Avg,
            1 => Sum,
            2 => Min,
            3 => Max,
            _ => return Err(UnknownConstantError::BatchMode(v)),
        };
        Ok(mode)
    }
}

/// Out of bound kinds.
#[derive(Debug)]
pub enum OutOfBounds {
    Arg(usize),
    Mem(usize),
    Dev(usize),
    Line(isize),
}

/// State simulator error type.
#[derive(Debug)]
pub enum ICStateError {
    AstError(AstError),
    DeviceError(DeviceError),
    UnknownConstantError(UnknownConstantError),
    TryFromIntError(TryFromIntError),
    AliasUnset(String),
    AliasWrongKind(String),
    OutOfBounds(OutOfBounds),
}

impl_from_error!(
    ICStateError,
    AstError,
    DeviceError,
    UnknownConstantError,
    TryFromIntError
);

/// Shortcut type for ICState error results.
pub type ICStateResult<T> = Result<T, ICStateError>;

/// Integrated Circuit (IC10) simulator state.
#[derive(Clone, Debug)]
pub struct ICState<'dk> {
    mem: Vec<f64>,
    dev: Vec<Option<Device<'dk>>>,
    map: HashMap<String, AliasKind>,
    network: HashMap<i64, Vec<Device<'dk>>>,
    sp: usize,
    ra: usize,
    pub next_line_index: usize,
}

mod exec;

impl<'dk> ICState<'dk> {
    // ============================================================================================
    // Builder methods
    // ============================================================================================

    /// New MIPS state with specified number of memory reigsters and device (IO) ports.
    pub fn new(mem_size: usize, dev_size: usize) -> Self {
        Self {
            mem: vec![0_f64; mem_size],
            dev: vec![None; dev_size],
            map: HashMap::new(),
            network: HashMap::new(),
            sp: mem_size - 2,
            ra: mem_size - 1,
            next_line_index: 0,
        }
    }

    /// Builder helper to set a memory register.
    pub fn with_mem<V: Into<f64>>(mut self, i: usize, v: V) -> Self {
        self.mem.get_mut(i).map(|r| *r = v.into());
        self
    }

    /// Builder helper to set a device.
    pub fn with_dev(mut self, i: usize, dev: Device<'dk>) -> Self {
        self.dev.get_mut(i).map(|d| *d = Some(dev));
        self
    }

    /// Builder helper to set a map alias.
    pub fn with_alias<K: Into<String>>(mut self, k: K, a: AliasKind) -> Self {
        self.map.insert(k.into(), a);
        self
    }

    // ============================================================================================
    // Utility methods
    // ============================================================================================

    /// Try to reduce a memory index by a number of indirections.
    pub fn index_reduce(&self, mut i: usize, num_indirections: usize) -> ICStateResult<usize> {
        for _ in 0..num_indirections {
            let j = self.get_mem(i)?;
            i = usize::try_from(i as isize * (*j) as isize)?;
        }
        Ok(i)
    }

    pub fn arg_reducer<'a, 'b>(&'a self, args: &'b Vec<Arg>) -> ArgReducer<'a, 'b> {
        ArgReducer { state: self, args }
    }

    // ============================================================================================
    // Jump methods
    // ============================================================================================

    /// Try to set the next `usize` line index.
    pub fn jump(&mut self, i: usize) {
        self.next_line_index = i;
    }

    /// Try to set the next `usize` line index, saving the current index in `ra`.
    pub fn jump_save(&mut self, i: usize) {
        let old_i = self.next_line_index;
        self.jump(i);
        self.mem[self.ra] = old_i as f64;
    }

    /// Try to jump to line.
    ///
    /// * `i` - The line to jump to.
    /// * `relative` - Is the jump index relative?
    /// * `save` - Save the current next line in `ra`?
    /// * `condition` - Execute this jump?
    pub fn jump_helper(
        &mut self,
        mut i: f64,
        relative: bool,
        save: bool,
        condition: bool,
    ) -> ICStateResult<bool> {
        if condition {
            if relative {
                i += self.next_line_index as f64;
            };
            let i = usize::try_from(i as isize)?;
            if save {
                self.jump_save(i);
            } else {
                self.jump(i);
            }
        }
        Ok(condition)
    }

    // ============================================================================================
    // Alias methods
    // ============================================================================================

    /// Set an alias.
    pub fn set_alias(&mut self, a: String, r: AliasKind) {
        self.map.insert(a, r);
    }

    /// Try to get an alias.
    pub fn get_alias(&self, a: &String) -> ICStateResult<&AliasKind> {
        self.map.get(a).ok_or(ICStateError::AliasUnset(a.into()))
    }

    // ============================================================================================
    // Memory methods
    // ============================================================================================

    /// Try to get a memory register value reference.
    pub fn get_mem(&self, i: usize) -> ICStateResult<&f64> {
        self.mem
            .get(i)
            .ok_or(ICStateError::OutOfBounds(OutOfBounds::Mem(i)))
    }

    /// Try to set a memory register value.
    pub fn set_mem(&mut self, i: usize, v: f64) -> ICStateResult<()> {
        self.mem
            .get_mut(i)
            .ok_or(ICStateError::OutOfBounds(OutOfBounds::Mem(i)))
            .map(|m| *m = v)
    }

    /// Try to reduce a memory node to a memory `usize` index.
    pub fn mem_reduce(&self, mem: &Mem) -> ICStateResult<usize> {
        match mem {
            Mem::MemAlias(a) => self.get_alias(a)?.mem_id().cloned(),
            &Mem::MemLit(i, num_indirections) => self.index_reduce(i, num_indirections),
        }
    }

    // ============================================================================================
    // Device methods
    // ============================================================================================

    /// Iterator over the state device options.
    pub fn iter_dev(&self) -> impl Iterator<Item = &Option<Device>> {
        self.dev.iter()
    }

    /// Is a device set.
    pub fn is_dev_set(&self, i: usize) -> ICStateResult<bool> {
        self.dev
            .get(i)
            .map(Option::is_some)
            .ok_or(ICStateError::OutOfBounds(OutOfBounds::Dev(i)))
    }

    /// Try to get a device reference.
    pub fn get_dev(&self, i: usize) -> ICStateResult<&Device<'dk>> {
        self.dev
            .get(i)
            .and_then(Option::as_ref)
            .ok_or(ICStateError::OutOfBounds(OutOfBounds::Dev(i)))
    }

    /// Try to get a mutable device reference.
    pub fn get_dev_mut(&mut self, i: usize) -> ICStateResult<&mut Device<'dk>> {
        self.dev
            .get_mut(i)
            .and_then(Option::as_mut)
            .ok_or(ICStateError::OutOfBounds(OutOfBounds::Dev(i)))
    }

    /// Try to set a device.
    pub fn set_dev(&mut self, i: usize, dev: Option<Device<'dk>>) -> ICStateResult<()> {
        self.dev
            .get_mut(i)
            .map(|d| *d = dev)
            .ok_or(ICStateError::OutOfBounds(OutOfBounds::Dev(i)))
    }

    /// Try to reduce a device node to a device `usize` index.
    pub fn dev_reduce(&self, dev: &Dev) -> ICStateResult<usize> {
        match dev {
            Dev::DevAlias(a) => self.get_alias(a)?.dev_id().cloned(),
            &Dev::DevLit(i, num_indirections) => self.index_reduce(i, num_indirections),
        }
    }

    /// Add device to the network.
    pub fn dev_network_add(&mut self, dev: Device<'dk>) {
        let hash = dev.kind.hash;
        let devices = self.network.entry(hash).or_insert(Vec::new());
        devices.push(dev);
    }

    /// Try to read a parameter value from all devices of a given hash on the network.
    ///
    /// If none of the particular device are on the network, returns zero.
    ///
    /// * `hash` - Hash of devices to read from
    /// * `var` - Parameter to read
    /// * `mode` - Mode (0: average, 1: sum, 2: min, 3: max)
    pub fn dev_network_read(&self, hash: i64, var: &str, mode: f64) -> ICStateResult<f64> {
        let mode = BatchMode::try_from(mode)?;
        if let Some(devices) = self.network.get(&hash) {
            let vals: Vec<f64> = devices
                .iter()
                .map(|dev| dev.read(var))
                .collect::<Result<Vec<f64>, DeviceError>>()?;
            let iter = vals.into_iter();
            let val = match mode {
                BatchMode::Avg => iter.sum::<f64>() / devices.len() as f64,
                BatchMode::Sum => iter.sum::<f64>(),
                BatchMode::Min => iter.fold(f64::INFINITY, f64::min),
                BatchMode::Max => iter.fold(f64::NEG_INFINITY, f64::max),
            };
            Ok(val)
        } else {
            Ok(0.0)
        }
    }

    /// Try to write a parameter value to all devices of a given hash on the network.
    ///
    /// * `hash` - Hash of devices to write to
    /// * `var` - Parameter to write
    /// * `val` - Value to write
    pub fn dev_network_write(&mut self, hash: i64, var: &String, val: f64) -> ICStateResult<()> {
        if let Some(devices) = self.network.get_mut(&hash) {
            for dev in devices.iter_mut() {
                dev.write(var, val)?;
            }
        }
        Ok(())
    }

    // ============================================================================================
    // Value methods
    // ============================================================================================

    /// Try to reduce a value node to an `f64`.
    pub fn val_reduce(&self, val: &Val) -> ICStateResult<f64> {
        match val {
            Val::ValLit(v) => Ok(*v),
            Val::ValMem(m) => match m {
                Mem::MemAlias(a) => match self.get_alias(a)? {
                    AliasKind::MemId(i) => self.get_mem(*i).cloned(),
                    AliasKind::Label(i) => Ok(*i as f64),
                    AliasKind::Def(v) => Ok(*v),
                    _ => Err(ICStateError::AliasWrongKind(a.clone())),
                },
                Mem::MemLit(_, _) => self.mem_reduce(m).and_then(|i| self.get_mem(i)).cloned(),
            },
        }
    }
}

impl<'dk> Display for ICState<'dk> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fn write_dev(indent: &'static str, dev: &Device, f: &mut fmt::Formatter) -> fmt::Result {
            let d_str = dev.to_string();
            let mut iter = d_str.split("\n");
            f.write_str(indent)?;
            f.write_fmt(format_args!("{}\n", iter.next().unwrap()))?;
            if let Some(line) = iter.next() {
                f.write_str(indent)?;
                f.write_fmt(format_args!("{}", line))?;
                while let Some(line) = iter.next() {
                    f.write_str("\n")?;
                    f.write_str(indent)?;
                    f.write_fmt(format_args!("{}", line))?;
                }
            }
            f.write_str(",\n")
        }

        f.write_str("ICState {\n")?;

        // Write status of memory
        f.write_str("    mem: [\n")?;
        let n = self.mem.len() - 2;
        let mut iter = self.mem.iter().enumerate();
        for _ in 0..n {
            let (i, v) = iter.next().unwrap();
            f.write_fmt(format_args!("        {}: {}\n", i, v))?;
        }
        let (i, v) = iter.next().unwrap();
        f.write_fmt(format_args!("        {}: {} (sp)\n", i, v))?;
        let (i, v) = iter.next().unwrap();
        f.write_fmt(format_args!("        {}: {} (ra)\n", i, v))?;
        f.write_str("    ],\n")?;

        // Write status of devices
        f.write_str("    dev: [\n")?;
        for (i, dev) in self.dev.iter().enumerate() {
            if let Some(d) = dev {
                f.write_fmt(format_args!("        {}: \n", i))?;
                write_dev("        ", &d, f)?;
            } else {
                f.write_fmt(format_args!("        {}: (unset),\n", i))?;
            }
        }
        f.write_str("    ],\n")?;

        // Write next line index
        f.write_fmt(format_args!(
            "    next_line_index: {},\n",
            self.next_line_index
        ))?;

        // Write status of map (aliases, labels, defines)
        f.write_str("    map: {\n")?;
        let mut iter = self.map.iter();
        if let Some((k, v)) = iter.next() {
            f.write_fmt(format_args!("        {}: {}", k, v))?;
            while let Some((k, v)) = iter.next() {
                f.write_fmt(format_args!(",\n        {}: {}", k, v))?;
            }
        }
        f.write_str("\n    },\n")?;

        fn write_devices(devices: &Vec<Device>, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str("        [\n")?;
            for d in devices.iter() {
                write_dev("            ", d, f)?;
            }
            f.write_str("        ]\n")?;
            Ok(())
        }

        // Write status of network devices
        f.write_str("    network: [\n")?;
        let mut iter = self.network.values();
        if let Some(devices) = iter.next() {
            write_devices(devices, f)?;
            while let Some(devices) = iter.next() {
                f.write_str(",\n")?;
                write_devices(devices, f)?;
            }
        }
        for _devices in self.network.values() {}
        f.write_str("    ]\n}")
    }
}

impl<'dk> Default for ICState<'dk> {
    fn default() -> Self {
        Self::new(18, 6)
            .with_alias("sp", AliasKind::MemId(16))
            .with_alias("ra", AliasKind::MemId(17))
    }
}

pub struct ArgReducer<'dk, 'arg> {
    state: &'dk ICState<'dk>,
    args: &'arg Vec<Arg>,
}

impl<'dk, 'arg> ArgReducer<'dk, 'arg> {
    fn get(&self, i: usize) -> ICStateResult<&Arg> {
        self.args
            .get(i)
            .ok_or(ICStateError::OutOfBounds(OutOfBounds::Arg(i)))
    }

    pub fn mem(&self, i: usize) -> ICStateResult<usize> {
        let arg = self.get(i)?;
        let mem = arg.mem()?;
        self.state.mem_reduce(mem)
    }

    pub fn dev(&self, i: usize) -> ICStateResult<usize> {
        let arg = self.get(i)?;
        let dev = arg.dev()?;
        self.state.dev_reduce(dev)
    }

    pub fn reg(&self, i: usize) -> ICStateResult<AliasKind> {
        let ak = match self.get(i)? {
            Arg::ArgMem(m) => {
                let i = self.state.mem_reduce(m)?;
                AliasKind::MemId(i as usize)
            }
            Arg::ArgDev(d) => {
                let i = self.state.dev_reduce(d)?;
                AliasKind::DevId(i as usize)
            }
            arg @ _ => {
                let ast_err = AstError::WrongArg(format!(
                    "Expected a memory or device argument, found {:?}",
                    arg
                ));
                return Err(ICStateError::AstError(ast_err));
            }
        };
        Ok(ak)
    }

    pub fn val(&self, i: usize) -> ICStateResult<f64> {
        let arg = self.get(i)?;
        let val = arg.val()?;
        self.state.val_reduce(val)
    }

    pub fn tkn(&self, i: usize) -> ICStateResult<String> {
        let arg = self.get(i)?;
        arg.token().map_err(ICStateError::AstError).cloned()
    }

    pub fn index(&self, i: usize) -> ICStateResult<usize> {
        Ok(usize::try_from(self.val(i)? as isize)?)
    }
}
