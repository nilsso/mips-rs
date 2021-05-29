//! Integrated Circuit (IC10) simulator state.
#![allow(dead_code)]
use std::collections::HashMap;
use std::convert::TryFrom;
use std::{fmt, fmt::Debug, fmt::Display};

use mips_parser::prelude::*;
use util::{impl_from_error, is_as_inner};

use std::num::{IntErrorKind, TryFromIntError};

use crate::device::{Device, DeviceError};
use crate::Line;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum DevId {
    DevBuf(usize),
    DevSelf,
}

/// Alias kind type.
#[derive(Clone, PartialEq)]
pub enum AliasKind {
    MemId(usize),
    DevId(DevId),
    DevSelf,
    Label(usize),
    Def(f64),
}

impl AliasKind {
    #[rustfmt::skip]
    is_as_inner!(AliasKind, ICStateError, ICStateError::AliasWrongKind, [
        (AliasKind::MemId, is_mem_id, as_mem_id, mem_id, &usize,    "Expected MemId, found {:?}"),
        (AliasKind::DevId, is_dev_id, as_dev_id, dev_id, &DevId, "Expected DevId, found {:?}"),
        (AliasKind::Label, is_label,  as_label,  label,  &usize,    "Expected Label, found {:?}"),
        (AliasKind::Def,   is_def,    as_def,    def,    &f64,      "Expected Def, found {:?}"),
    ]);
}

impl Display for AliasKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use AliasKind::*;
        match self {
            MemId(i) => write!(f, "{}", i),
            DevId(di) => write!(f, "{:?}", di),
            DevSelf => write!(f, "(self)"),
            Label(i) => write!(f, "{}", i),
            Def(v) => write!(f, "{}", v),
        }
    }
}

impl Debug for AliasKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use AliasKind::*;
        match self {
            MemId(i) => write!(f, "MemId({})", i),
            DevId(di) => write!(f, "DevId({:?})", di),
            DevSelf => write!(f, "(self)"),
            Label(i) => write!(f, "Label({})", i),
            Def(v) => write!(f, "Def({})", v),
        }
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
    Stack(usize),
}

/// State simulator error type.
#[derive(Debug)]
pub enum ICStateError {
    AstError(AstError),
    DeviceError(DeviceError),
    UnknownConstantError(UnknownConstantError),
    TryFromIntError(TryFromIntError),
    IntErrorKind(IntErrorKind),

    AliasUnset(String),
    AliasWrongKind(String),
    OutOfBounds(OutOfBounds),
    StackFull,
    StackEmpty,
    WrongNumberOfArgs(String),
    WrongArg(String),
    HaltAndCatchFire,
}

impl_from_error!(
    ICStateError,
    AstError,
    DeviceError,
    UnknownConstantError,
    TryFromIntError,
    IntErrorKind,
);

/// Ok result type for ICState::exec_line.
pub enum ExecResult {
    Normal(bool),
    Sleep(f64),
    Yield,
}

/// Shortcut type for ICState error results.
pub type ICStateResult<T> = Result<T, ICStateError>;

pub type MemRegs<const MS: usize> = [f64; MS];
pub type Devices<'dk, const DS: usize> = [Option<Device<'dk>>; DS];

/// Integrated Circuit (IC10) simulator state.
#[derive(Clone, Debug)]
pub struct ICState<'dk, const MS: usize, const DS: usize, const SS: usize> {
    // Memory register, device IO and stack buffers
    mem: MemRegs<MS>,
    dev: Devices<'dk, DS>,
    stk: MemRegs<SS>,
    // Device of the state itself, if set
    dev_self: Option<Device<'dk>>,
    // Alias map (alias string -> alias kind)
    map: HashMap<String, AliasKind>,
    // Network devices (hash -> devices of hash)
    network: HashMap<i64, Vec<Device<'dk>>>,
    // Index of next line in program (used for jumps, but more so by `ICSimulator`)
    pub(crate) next_line_index: usize,
}

// Argument reducer helper
mod arg_reducer;

/// Stationeers/C# constants
pub const EPS: f64 = 1.121039e-44; // floating-point epsilon

impl<'dk, const MS: usize, const DS: usize, const SS: usize> ICState<'dk, MS, DS, SS> {
    // Special memory indices
    // (`sp` and `ra` aliases can change, but these stay internally constant)
    pub const SP: usize = MS - 2;
    pub const RA: usize = MS - 1;

    // ============================================================================================
    // Builder methods
    // ============================================================================================

    /// New MIPS state with specified number of memory reigsters and device (IO) ports.
    pub fn new() -> Self {
        const NONE: Option<Device<'static>> = None;

        Self {
            mem: [0_f64; MS],
            dev: [NONE; DS],
            stk: [0.0; SS],
            dev_self: NONE,
            map: HashMap::new(),
            network: HashMap::new(),
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

    pub fn with_dev_self(mut self, dev: Device<'dk>) -> Self {
        self.dev_self = Some(dev);
        self
    }

    // ============================================================================================
    // Utility methods
    // ============================================================================================

    /// Try to reduce a memory index by a number of indirections.
    pub fn index_reduce(&self, mut i: usize, num_indirections: usize) -> ICStateResult<usize> {
        for _ in 0..num_indirections {
            let j = self.get_mem(i)?;
            i = usize::try_from((*j) as isize)?;
        }
        Ok(i)
    }

    fn try_index(v: f64) -> Result<usize, IntErrorKind> {
        // NOTE: This may be subject to change, but Stationeering ignores f64 values not near enough to
        // an integer (abs(v - round(v) <= EPS)), but Stationeers in game rounds all the time.
        // let a = <usize>::try_from();
        if -0.05 < v && v < (SS as f64 + 0.5) {
            Ok(v as usize)
        } else {
            Err(IntErrorKind::NegOverflow)
        }
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
        self.mem[Self::RA] = old_i as f64;
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

    pub fn get_mem_buffer(&self) -> &[f64; MS] {
        &self.mem
    }

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

    /// Get a device option reference.
    fn get_dev_unchecked(&self, di: DevId) -> Option<&Option<Device<'dk>>> {
        match di {
            DevId::DevBuf(i) => self.dev.get(i),
            DevId::DevSelf => Some(&self.dev_self),
        }
    }

    /// Get a mutable device option reference.
    fn get_mut_dev_unchecked(&mut self, di: DevId) -> Option<&mut Option<Device<'dk>>> {
        match di {
            DevId::DevBuf(i) => self.dev.get_mut(i),
            DevId::DevSelf => Some(&mut self.dev_self),
        }
    }

    /// Try to get a device option reference.
    ///
    /// Fails if the index is out of bounds.
    pub fn get_dev_opt(&self, di: DevId) -> ICStateResult<&Option<Device<'dk>>> {
        match di {
            DevId::DevBuf(i) => Ok(self
                .get_dev_unchecked(di)
                .ok_or(ICStateError::OutOfBounds(OutOfBounds::Dev(i)))?),
            DevId::DevSelf => Ok(&self.dev_self),
        }
    }

    /// Try to get a mutable device option reference.
    ///
    /// Fails if the index is out of bounds.
    pub fn get_mut_dev_opt(&mut self, di: DevId) -> ICStateResult<&mut Option<Device<'dk>>> {
        match di {
            DevId::DevBuf(i) => Ok(self
                .get_mut_dev_unchecked(di)
                .ok_or(ICStateError::OutOfBounds(OutOfBounds::Dev(i)))?),
            DevId::DevSelf => Ok(&mut self.dev_self),
        }
    }

    /// Try to get a device reference.
    ///
    /// Fails if the index is out of bounds or if the underlying device is unset.
    pub fn get_dev(&self, di: DevId) -> ICStateResult<&Device<'dk>> {
        self.get_dev_opt(di)?
            .as_ref()
            .ok_or(ICStateError::DeviceError(DeviceError::Unset))
    }

    /// Try to get a mutable device reference.
    ///
    /// Fails if the index is out of bounds or if the underlying device is unset.
    pub fn get_mut_dev(&mut self, di: DevId) -> ICStateResult<&mut Device<'dk>> {
        self.get_mut_dev_opt(di)?
            .as_mut()
            .ok_or(ICStateError::DeviceError(DeviceError::Unset))
    }

    /// Try to check if a device is set.
    ///
    /// Fails if the index is out of bounds.
    pub fn is_dev_set(&self, di: DevId) -> ICStateResult<bool> {
        Ok(self.get_dev_opt(di)?.is_some())
    }

    /// Try to set a device.
    pub fn set_dev(&mut self, di: DevId, dev_opt: Option<Device<'dk>>) -> ICStateResult<()> {
        Ok(*self.get_mut_dev_opt(di)? = dev_opt)
    }

    /// Try to reduce a device node to a device `usize` index.
    ///
    /// Note that `db` is an alias to the self device, which can be overwritten.
    pub fn dev_reduce(&self, dev: &Dev) -> ICStateResult<DevId> {
        match dev {
            Dev::DevAlias(a) => match self.get_alias(a)? {
                AliasKind::DevId(di) => Ok(*di),
                AliasKind::DevSelf => Ok(DevId::DevSelf),
                _ => unreachable!(),
            },
            Dev::DevLit(i, num_indirections) => {
                let i = self.index_reduce(*i, *num_indirections)?;
                Ok(DevId::DevBuf(i))
            }
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
                Mem::MemLit(_, _) => {
                    let m = self.mem_reduce(m);
                    m.and_then(|i| self.get_mem(i)).cloned()
                }
            },
        }
    }

    // ============================================================================================
    // Stack methods
    // ============================================================================================

    pub fn get_stack_buffer(&self) -> &[f64; SS] {
        &self.stk
    }

    pub fn get_stack_head(&mut self) -> ICStateResult<(&mut f64, &mut f64)> {
        let sp = &mut self.mem[Self::SP];
        let i = Self::try_index(*sp)?;
        if i < SS {
            Ok((sp, &mut self.stk[i]))
        } else {
            Err(ICStateError::OutOfBounds(OutOfBounds::Stack(i)))
        }
    }

    pub fn push(&mut self, v: f64) -> ICStateResult<()> {
        let sp = &mut self.mem[Self::SP];
        let i = Self::try_index(*sp)?;
        *sp += 1.0;
        self.stk[i] = v;
        Ok(())
    }

    pub fn peek(&mut self) -> ICStateResult<f64> {
        let sp = &mut self.mem[Self::SP];
        let i = Self::try_index(*sp - 1.0)?;
        Ok(self.stk[i])
    }

    pub fn pop(&mut self) -> ICStateResult<f64> {
        let sp = &mut self.mem[Self::SP];
        let i = Self::try_index(*sp - 1.0)?;
        *sp -= 1.0;
        Ok(self.stk[i])
    }

    // ============================================================================================
    // MIPS implementation
    // ============================================================================================

    pub fn exec_line(&mut self, line: &Line) -> ICStateResult<ExecResult> {
        use arg_reducer::{ArgReducer, D, M, R, T, V};
        use std::convert::TryInto;
        use Func::*;

        #[inline]
        fn bool_to_val(b: bool) -> f64 {
            if b {
                1.0
            } else {
                0.0
            }
        }

        #[inline]
        fn f_ap(a: f64, b: f64, c: f64) -> bool {
            (a - b).abs() <= (c * a.abs().max(b.abs()).max(EPS))
        }

        let mut jumped = false;
        if let Line::Expr(i, expr) = line {
            let (func, args) = expr.into();
            let reducer = &ArgReducer::new(self, &args);

            match func {
                // ================================================================================
                // Device IO
                // ================================================================================
                Bdns => {
                    let (D(d), V(l)) = reducer.try_into()?;
                    jumped = self.jump_helper(l, false, false, !self.is_dev_set(d)?)?;
                }
                Bdnsal => {
                    let (D(d), V(l)) = reducer.try_into()?;
                    jumped = self.jump_helper(l, false, true, !self.is_dev_set(d)?)?;
                }
                Bdse => {
                    let (D(d), V(l)) = reducer.try_into()?;
                    jumped = self.jump_helper(l, false, false, self.is_dev_set(d)?)?;
                }
                Bdseal => {
                    let (D(d), V(l)) = reducer.try_into()?;
                    jumped = self.jump_helper(l, false, true, self.is_dev_set(d)?)?;
                }
                Brdns => {
                    let (D(d), V(l)) = reducer.try_into()?;
                    jumped = self.jump_helper(l, true, false, !self.is_dev_set(d)?)?;
                }
                Brdse => {
                    let (D(d), V(l)) = reducer.try_into()?;
                    jumped = self.jump_helper(l, true, false, self.is_dev_set(d)?)?;
                }
                L => {
                    let (M(r), D(d), T(t)) = reducer.try_into()?;
                    let dev = self.get_dev(d)?;
                    let param_value = dev.read(t)?;
                    self.set_mem(r, param_value)?;
                }
                Lb => {
                    let (M(r), V(h), T(p), V(m)) = reducer.try_into()?;
                    let val = self.dev_network_read(h as i64, &p, m)?;
                    self.set_mem(r, val)?;
                }
                Lr => {
                    // TODO: Reagent everything
                }
                Ls => {
                    // TODO: Slot everything
                }
                S => {
                    let (D(d), T(p), V(v)) = reducer.try_into()?;
                    let dev = self.get_mut_dev(d)?;
                    dev.write(p, v)?;
                }
                Sb => {
                    let (V(h), T(p), V(v)) = reducer.try_into()?;
                    self.dev_network_write(h as i64, &p, v)?;
                }
                // ================================================================================
                // Flow Control, Branches and Jumps
                // ================================================================================
                Bap => {
                    let (V(a), V(b), V(c), V(l)) = reducer.try_into()?;
                    jumped = self.jump_helper(l, false, false, f_ap(a, b, c))?;
                }
                Bapal => {
                    let (V(a), V(b), V(c), V(l)) = reducer.try_into()?;
                    jumped = self.jump_helper(l, false, true, f_ap(a, b, c))?;
                }
                Bapz => {
                    let (V(a), V(b), V(l)) = reducer.try_into()?;
                    jumped = self.jump_helper(l, false, false, f_ap(a, 0.0, b))?;
                }
                Bapzal => {
                    let (V(a), V(b), V(l)) = reducer.try_into()?;
                    jumped = self.jump_helper(l, false, true, f_ap(a, 0.0, b))?;
                }
                Beq => {
                    let (V(a), V(b), V(l)) = reducer.try_into()?;
                    jumped = self.jump_helper(l, false, false, a == b)?;
                }
                Beqal => {
                    let (V(a), V(b), V(l)) = reducer.try_into()?;
                    jumped = self.jump_helper(l, false, true, a == b)?;
                }
                Beqz => {
                    let (V(a), V(l)) = reducer.try_into()?;
                    jumped = self.jump_helper(l, false, false, a == 0.0)?;
                }
                Beqzal => {
                    let (V(a), V(l)) = reducer.try_into()?;
                    jumped = self.jump_helper(l, false, true, a == 0.0)?;
                }
                Bge => {
                    let (V(a), V(b), V(l)) = reducer.try_into()?;
                    jumped = self.jump_helper(l, false, false, a >= b)?;
                }
                Bgeal => {
                    let (V(a), V(b), V(l)) = reducer.try_into()?;
                    jumped = self.jump_helper(l, false, true, a >= b)?;
                }
                Bgez => {
                    let (V(a), V(l)) = reducer.try_into()?;
                    jumped = self.jump_helper(l, false, false, a >= 0.0)?;
                }
                Bgezal => {
                    let (V(a), V(l)) = reducer.try_into()?;
                    jumped = self.jump_helper(l, false, true, a >= 0.0)?;
                }
                Bgt => {
                    let (V(a), V(b), V(l)) = reducer.try_into()?;
                    jumped = self.jump_helper(l, false, false, a > b)?;
                }
                Bgtal => {
                    let (V(a), V(b), V(l)) = reducer.try_into()?;
                    jumped = self.jump_helper(l, false, true, a > b)?;
                }
                Bgtz => {
                    let (V(a), V(l)) = reducer.try_into()?;
                    jumped = self.jump_helper(l, false, false, a > 0.0)?;
                }
                Bgtzal => {
                    let (V(a), V(l)) = reducer.try_into()?;
                    jumped = self.jump_helper(l, false, true, a > 0.0)?;
                }
                Ble => {
                    let (V(a), V(b), V(l)) = reducer.try_into()?;
                    jumped = self.jump_helper(l, false, false, a <= b)?;
                }
                Bleal => {
                    let (V(a), V(b), V(l)) = reducer.try_into()?;
                    jumped = self.jump_helper(l, false, true, a <= b)?;
                }
                Blez => {
                    let (V(a), V(l)) = reducer.try_into()?;
                    jumped = self.jump_helper(l, false, false, a <= 0.0)?;
                }
                Blezal => {
                    let (V(a), V(l)) = reducer.try_into()?;
                    jumped = self.jump_helper(l, false, true, a <= 0.0)?;
                }
                Blt => {
                    let (V(a), V(b), V(l)) = reducer.try_into()?;
                    jumped = self.jump_helper(l, false, false, a < b)?;
                }
                Bltal => {
                    let (V(a), V(b), V(l)) = reducer.try_into()?;
                    jumped = self.jump_helper(l, false, true, a < b)?;
                }
                Bltz => {
                    let (V(a), V(l)) = reducer.try_into()?;
                    jumped = self.jump_helper(l, false, false, a < 0.0)?;
                }
                Bltzal => {
                    let (V(a), V(l)) = reducer.try_into()?;
                    jumped = self.jump_helper(l, false, true, a < 0.0)?;
                }
                Bna => {
                    let (V(a), V(b), V(c), V(l)) = reducer.try_into()?;
                    jumped = self.jump_helper(l, false, false, f_ap(a, b, c))?;
                }
                Bnaal => {
                    let (V(a), V(b), V(c), V(l)) = reducer.try_into()?;
                    jumped = self.jump_helper(l, false, true, f_ap(a, b, c))?;
                }
                Bnaz => {
                    let (V(a), V(b), V(l)) = reducer.try_into()?;
                    jumped = self.jump_helper(l, false, false, f_ap(a, 0.0, b))?;
                }
                Bnazal => {
                    let (V(a), V(b), V(l)) = reducer.try_into()?;
                    jumped = self.jump_helper(l, false, true, f_ap(a, 0.0, b))?;
                }
                Bne => {
                    let (V(a), V(b), V(l)) = reducer.try_into()?;
                    jumped = self.jump_helper(l, false, false, a != b)?;
                }
                Bneal => {
                    let (V(a), V(b), V(l)) = reducer.try_into()?;
                    jumped = self.jump_helper(l, false, true, a != b)?;
                }
                Bnez => {
                    let (V(a), V(l)) = reducer.try_into()?;
                    jumped = self.jump_helper(l, false, false, a != 0.0)?;
                }
                Bnezal => {
                    let (V(a), V(l)) = reducer.try_into()?;
                    jumped = self.jump_helper(l, false, true, a != 0.0)?;
                }
                Brap => {
                    let (V(a), V(b), V(c), V(l)) = reducer.try_into()?;
                    jumped = self.jump_helper(l, true, false, f_ap(a, b, c))?;
                }
                Brapz => {
                    let (V(a), V(b), V(l)) = reducer.try_into()?;
                    jumped = self.jump_helper(l, true, false, f_ap(a, 0.0, b))?;
                }
                Breq => {
                    let (V(a), V(b), V(l)) = reducer.try_into()?;
                    jumped = self.jump_helper(l, true, false, a == b)?;
                }
                Breqz => {
                    let (V(a), V(l)) = reducer.try_into()?;
                    jumped = self.jump_helper(l, true, false, a == 0.0)?;
                }
                Brge => {
                    let (V(a), V(b), V(l)) = reducer.try_into()?;
                    jumped = self.jump_helper(l, true, false, a >= b)?;
                }
                Brgez => {
                    let (V(a), V(l)) = reducer.try_into()?;
                    jumped = self.jump_helper(l, true, false, a >= 0.0)?;
                }
                Brgt => {
                    let (V(a), V(b), V(l)) = reducer.try_into()?;
                    jumped = self.jump_helper(l, true, false, a > b)?;
                }
                Brgtz => {
                    let (V(a), V(l)) = reducer.try_into()?;
                    jumped = self.jump_helper(l, true, false, a > 0.0)?;
                }
                Brle => {
                    let (V(a), V(b), V(l)) = reducer.try_into()?;
                    jumped = self.jump_helper(l, true, false, a <= b)?;
                }
                Brlez => {
                    let (V(a), V(l)) = reducer.try_into()?;
                    jumped = self.jump_helper(l, true, false, a <= 0.0)?;
                }
                Brlt => {
                    let (V(a), V(b), V(l)) = reducer.try_into()?;
                    jumped = self.jump_helper(l, true, false, a < b)?;
                }
                Brltz => {
                    let (V(a), V(l)) = reducer.try_into()?;
                    jumped = self.jump_helper(l, true, false, a < 0.0)?;
                }
                Brna => {
                    let (V(a), V(b), V(c), V(l)) = reducer.try_into()?;
                    jumped = self.jump_helper(l, true, false, !f_ap(a, b, c))?;
                }
                Brnaz => {
                    let (V(a), V(b), V(l)) = reducer.try_into()?;
                    jumped = self.jump_helper(l, true, false, !f_ap(a, 0.0, b))?;
                }
                Brne => {
                    let (V(a), V(b), V(l)) = reducer.try_into()?;
                    jumped = self.jump_helper(l, true, false, a != b)?;
                }
                Brnez => {
                    let (V(a), V(l)) = reducer.try_into()?;
                    jumped = self.jump_helper(l, true, false, a != 0.0)?;
                }
                J => {
                    let (V(l),) = reducer.try_into()?;
                    jumped = self.jump_helper(l, false, false, true)?;
                }
                Jal => {
                    let (V(l),) = reducer.try_into()?;
                    jumped = self.jump_helper(l, false, true, true)?;
                }
                Jr => {
                    let (V(l),) = reducer.try_into()?;
                    jumped = self.jump_helper(l, true, false, true)?;
                }
                // ================================================================================
                // Variable Selection
                // ================================================================================
                Sap => {
                    let (M(r), V(a), V(b), V(c)) = reducer.try_into()?;
                    self.set_mem(r, bool_to_val(f_ap(a, b, c)))?;
                }
                Sapz => {
                    let (M(r), V(a), V(b)) = reducer.try_into()?;
                    self.set_mem(r, bool_to_val(f_ap(a, 0.0, b)))?;
                }
                Sdns => {
                    let (M(r), D(d)) = reducer.try_into()?;
                    self.set_mem(r, bool_to_val(!self.is_dev_set(d)?))?;
                }
                Sdse => {
                    let (M(r), D(d)) = reducer.try_into()?;
                    self.set_mem(r, bool_to_val(self.is_dev_set(d)?))?;
                }
                Select => {
                    let (M(r), V(a), V(b), V(c)) = reducer.try_into()?;
                    // TODO: Test whether the game uses a approximately zero, or absolutely
                    self.set_mem(r, if a == 0.0 { b } else { c })?;
                }
                Seq => {
                    let (M(r), V(a), V(b)) = reducer.try_into()?;
                    self.set_mem(r, bool_to_val(a == b))?;
                }
                Seqz => {
                    let (M(r), V(a)) = reducer.try_into()?;
                    self.set_mem(r, bool_to_val(a == 0.0))?;
                }
                Sge => {
                    let (M(r), V(a), V(b)) = reducer.try_into()?;
                    self.set_mem(r, bool_to_val(a >= b))?;
                }
                Sgez => {
                    let (M(r), V(a)) = reducer.try_into()?;
                    self.set_mem(r, bool_to_val(a >= 0.0))?;
                }
                Sgt => {
                    let (M(r), V(a), V(b)) = reducer.try_into()?;
                    self.set_mem(r, bool_to_val(a > b))?;
                }
                Sgtz => {
                    let (M(r), V(a)) = reducer.try_into()?;
                    self.set_mem(r, bool_to_val(a > 0.0))?;
                }
                Sle => {
                    let (M(r), V(a), V(b)) = reducer.try_into()?;
                    self.set_mem(r, bool_to_val(a <= b))?;
                }
                Slez => {
                    let (M(r), V(a)) = reducer.try_into()?;
                    self.set_mem(r, bool_to_val(a <= 0.0))?;
                }
                Slt => {
                    let (M(r), V(a), V(b)) = reducer.try_into()?;
                    self.set_mem(r, bool_to_val(a < b))?;
                }
                Sltz => {
                    let (M(r), V(a)) = reducer.try_into()?;
                    self.set_mem(r, bool_to_val(a < 0.0))?;
                }
                Sna => {
                    let (M(r), V(a), V(b), V(c)) = reducer.try_into()?;
                    self.set_mem(r, bool_to_val(!f_ap(a, b, c)))?;
                }
                Snaz => {
                    let (M(r), V(a), V(b)) = reducer.try_into()?;
                    self.set_mem(r, bool_to_val(!f_ap(a, 0.0, b)))?;
                }
                Sne => {
                    let (M(r), V(a), V(b)) = reducer.try_into()?;
                    self.set_mem(r, bool_to_val(a != b))?;
                }
                Snez => {
                    let (M(r), V(a)) = reducer.try_into()?;
                    self.set_mem(r, bool_to_val(a != 0.0))?;
                }
                // ================================================================================
                // Mathematical Operations
                // ================================================================================
                Abs => {
                    let (M(r), V(a)) = reducer.try_into()?;
                    self.set_mem(r, a.abs())?;
                }
                Acos => {
                    let (M(r), V(a)) = reducer.try_into()?;
                    self.set_mem(r, a.acos())?;
                }
                Add => {
                    let (M(r), V(a), V(b)) = reducer.try_into()?;
                    self.set_mem(r, a + b)?;
                }
                Asin => {
                    let (M(r), V(a)) = reducer.try_into()?;
                    self.set_mem(r, a.asin())?;
                }
                Atan => {
                    let (M(r), V(a)) = reducer.try_into()?;
                    self.set_mem(r, a.atan())?;
                }
                Ceil => {
                    let (M(r), V(a)) = reducer.try_into()?;
                    self.set_mem(r, a.ceil())?;
                }
                Cos => {
                    let (M(r), V(a)) = reducer.try_into()?;
                    self.set_mem(r, a.cos())?;
                }
                Div => {
                    let (M(r), V(a), V(b)) = reducer.try_into()?;
                    self.set_mem(r, a / b)?;
                }
                Exp => {
                    let (M(r), V(a)) = reducer.try_into()?;
                    self.set_mem(r, a.exp())?;
                }
                Floor => {
                    let (M(r), V(a)) = reducer.try_into()?;
                    self.set_mem(r, a.floor())?;
                }
                Log => {
                    let (M(r), V(a)) = reducer.try_into()?;
                    self.set_mem(r, a.ln())?;
                }
                Max => {
                    let (M(r), V(a), V(b)) = reducer.try_into()?;
                    self.set_mem(r, a.max(b))?;
                }
                Min => {
                    let (M(r), V(a), V(b)) = reducer.try_into()?;
                    self.set_mem(r, a.min(b))?;
                }
                Mod => {
                    let (M(r), V(a), V(b)) = reducer.try_into()?;
                    self.set_mem(r, a.rem_euclid(b))?;
                }
                Mul => {
                    let (M(r), V(a), V(b)) = reducer.try_into()?;
                    self.set_mem(r, a * b)?;
                }
                Rand => {
                    let (M(r),) = reducer.try_into()?;
                    self.set_mem(r, rand::random())?;
                }
                Round => {
                    let (M(r), V(a)) = reducer.try_into()?;
                    self.set_mem(r, a.round())?;
                }
                Sin => {
                    let (M(r), V(a)) = reducer.try_into()?;
                    self.set_mem(r, a.sin())?;
                }
                Sqrt => {
                    let (M(r), V(a)) = reducer.try_into()?;
                    self.set_mem(r, a.sqrt())?;
                }
                Sub => {
                    let (M(r), V(a), V(b)) = reducer.try_into()?;
                    self.set_mem(r, a - b)?;
                }
                Tan => {
                    let (M(r), V(a)) = reducer.try_into()?;
                    self.set_mem(r, a.tan())?;
                }
                Trunc => {
                    let (M(r), V(a)) = reducer.try_into()?;
                    self.set_mem(r, a.trunc())?;
                }
                // ================================================================================
                // Logic
                // ================================================================================
                And => {
                    let (M(r), V(a), V(b)) = reducer.try_into()?;
                    self.set_mem(r, bool_to_val((a > 0.0) || (b > 0.0)))?;
                }
                Nor => {
                    let (M(r), V(a), V(b)) = reducer.try_into()?;
                    self.set_mem(r, bool_to_val(!((a > 0.0) || (b > 0.0))))?;
                }
                Or => {
                    let (M(r), V(a), V(b)) = reducer.try_into()?;
                    self.set_mem(r, bool_to_val((a > 0.0) || (b > 0.0)))?;
                }
                Xor => {
                    let (M(r), V(a), V(b)) = reducer.try_into()?;
                    self.set_mem(r, bool_to_val(a != b))?;
                }
                // ================================================================================
                // Stack
                // ================================================================================
                Peek => {
                    let (M(r),) = reducer.try_into()?;
                    let v = self.peek()?;
                    self.set_mem(r, v)?;
                }
                Pop => {
                    let (M(r),) = reducer.try_into()?;
                    let v = self.pop()?;
                    self.set_mem(r, v)?;
                }
                Push => {
                    let (V(v),) = reducer.try_into()?;
                    self.push(v)?;
                }
                // ================================================================================
                // Misc
                // ================================================================================
                Alias => {
                    let (T(a), R(r)) = reducer.try_into()?;
                    self.set_alias(a, r);
                }
                Define => {
                    let (T(a), V(v)) = reducer.try_into()?;
                    self.set_alias(a, AliasKind::Def(v));
                }
                Hcf => {
                    return Err(ICStateError::HaltAndCatchFire);
                }
                Move => {
                    let (M(r), V(a)) = reducer.try_into()?;
                    self.set_mem(r, a)?;
                }
                Sleep => {
                    let (V(v),) = reducer.try_into()?;
                    return Ok(ExecResult::Sleep(v));
                }
                Yield => {
                    return Ok(ExecResult::Yield);
                }
                // ================================================================================
                // Label
                // ================================================================================
                Label => {
                    let (T(l),) = reducer.try_into()?;
                    self.set_alias(l, AliasKind::Label(*i));
                }
            };
        }
        Ok(ExecResult::Normal(jumped))
    }
}

impl<'dk, const MS: usize, const DS: usize, const SS: usize> Display for ICState<'dk, MS, DS, SS> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use itertools::join;

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

        // Write status of map (aliases, labels, defines)
        f.write_str("    map: {\n")?;
        let mut iter = self.map.iter();
        if let Some((k, v)) = iter.next() {
            f.write_fmt(format_args!("        {}: {:?}", k, v))?;
            while let Some((k, v)) = iter.next() {
                f.write_fmt(format_args!(",\n        {}: {:?}", k, v))?;
            }
        }
        f.write_str("\n    },\n")?;

        // Write stack values
        f.write_str("    stack: [\n")?;
        const PER_ROW: usize = 16;
        let stack_str = join(
            (0..SS).step_by(PER_ROW).map(|i| {
                join(
                    self.stk
                        .iter()
                        .skip(i)
                        .take(PER_ROW)
                        .map(|v| format!("{:5.2}", v)),
                    ", ",
                )
            }),
            ",\n        ",
        );
        f.write_fmt(format_args!("        {}\n", stack_str))?;
        f.write_str("    ],\n")?;

        // Write next line index
        f.write_fmt(format_args!(
            "    next_line_index: {},\n",
            self.next_line_index
        ))?;

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
