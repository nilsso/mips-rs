//! Integrated Circuit (IC10) simulator state.
#![allow(dead_code)]
use std::collections::HashMap;
use std::convert::TryFrom;
use std::{fmt, fmt::Debug, fmt::Display};

use mips_parser::prelude::*;
use util::{impl_from_error, is_as_inner};

use std::num::{TryFromIntError, IntErrorKind};

use crate::device::{Device, DeviceError};
use crate::Line;

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
        (AliasKind::MemId, is_mem_id, as_mem_id, mem_id, &usize, "Expected MemId, found {:?}"),
        (AliasKind::DevId, is_dev_id, as_dev_id, dev_id, &usize, "Expected DevId, found {:?}"),
        (AliasKind::Label, is_label,  as_label,  label,  &usize, "Expected Label, found {:?}"),
        (AliasKind::Def,   is_def,    as_def,    def,    &f64,   "Expected Def, found {:?}"),
    ]);
}

impl Display for AliasKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use AliasKind::*;
        match self {
            MemId(i) => write!(f, "{}", i),
            DevId(i) => write!(f, "{}", i),
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
            DevId(i) => write!(f, "DevId({})", i),
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

/// Integrated Circuit (IC10) simulator state.
#[derive(Clone, Debug)]
pub struct ICState<'dk, const STACKSIZE: usize> {
    mem: Vec<f64>,
    dev: Vec<Option<Device<'dk>>>,
    map: HashMap<String, AliasKind>,
    network: HashMap<i64, Vec<Device<'dk>>>,
    stack: [f64; STACKSIZE],
    sp: usize,
    ra: usize,
    pub next_line_index: usize,
}

// Argument reducer helper
mod arg_reducer;

/// Stationeers/C# floating-point epsilon
pub const EPS: f64 = 1.121039e-44;

fn try_index(v: f64) -> Result<usize, IntErrorKind> {
    // NOTE: This may be subject to change, but Stationeering ignores f64 values not near enough to
    // an integer (abs(v - round(v) <= EPS)), but Stationeers in game rounds all the time.
    // let a = <usize>::try_from();
    if v > -0.05 {
        Ok(v as usize)
    } else {
        Err(IntErrorKind::NegOverflow)
    }
}

impl<'dk, const STACKSIZE: usize> ICState<'dk, STACKSIZE> {
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
            stack: [0.0; STACKSIZE],
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
            i = usize::try_from((*j) as isize)?;
        }
        Ok(i)
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

    /// Helper to try to set a memory register value if a condition is true.
    pub fn set_mem_helper(&mut self, i: usize, v: f64, cond: bool) -> ICStateResult<()> {
        if cond {
            self.set_mem(i, v)?;
        }
        Ok(())
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

    pub fn get_stack_head(&mut self) -> ICStateResult<(&mut f64, &mut f64)> {
        let sp = &mut self.mem[self.sp];
        let i = try_index(*sp)?;
        if i < STACKSIZE {
            Ok((sp, &mut self.stack[i]))
        } else {
            Err(ICStateError::OutOfBounds(OutOfBounds::Stack(i)))
        }
    }

    pub fn push(&mut self, v: f64) -> ICStateResult<()> {
        let (i, sp) = self.get_stack_head()?;
        *i += 1.0;
        *sp = v;
        Ok(())
    }

    pub fn peek(&mut self) -> ICStateResult<f64> {
        let (_, sp) = self.get_stack_head()?;
        Ok(*sp)
    }

    pub fn pop(&mut self) -> ICStateResult<f64> {
        let (i, sp) = self.get_stack_head()?;
        *i -= 1.0;
        Ok(*sp)
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

        let mut jumped = false;
        if let Line::Expr(i, expr) = line {
            let (func, args) = expr.into();
            let reducer = &ArgReducer::new(self, &args);

            #[rustfmt::skip]
            match func {
                // ================================================================================
                // Device IO
                // ================================================================================
                Bdns   => {
                    let (D(d), V(l)) = reducer.try_into()?;
                    jumped = self.jump_helper(l, false, false, !self.is_dev_set(d)?)?;
                },
                Bdnsal => {
                    let (D(d), V(l)) = reducer.try_into()?;
                    jumped = self.jump_helper(l, false, true, !self.is_dev_set(d)?)?;
                },
                Bdse   => {
                    let (D(d), V(l)) = reducer.try_into()?;
                    jumped = self.jump_helper(l, false, false, self.is_dev_set(d)?)?;
                },
                Bdseal => {
                    let (D(d), V(l)) = reducer.try_into()?;
                    jumped = self.jump_helper(l, false, true, self.is_dev_set(d)?)?;
                },
                Brdns  => {
                    let (D(d), V(l)) = reducer.try_into()?;
                    jumped = self.jump_helper(l, true, false, !self.is_dev_set(d)?)?;
                },
                Brdse  => {
                    let (D(d), V(l)) = reducer.try_into()?;
                    jumped = self.jump_helper(l, true, false, self.is_dev_set(d)?)?;
                },
                L      => {
                    let (M(r), D(d), T(t)) = reducer.try_into()?;
                    let dev = self.get_dev(d)?;
                    let param_value = dev.read(t)?;
                    self.set_mem(r, param_value)?;
                },
                Lb     => {
                    let (M(r), V(h), T(p), V(m)) = reducer.try_into()?;
                    let val = self.dev_network_read(h as i64, &p, m)?;
                    self.set_mem(r, val)?;
                },
                Lr     => {
                    // TODO: Reagent everything
                },
                Ls     => {
                    // TODO: Slot everything
                },
                S      => {
                    let (D(_d), T(_p), V(_v)) = reducer.try_into()?;
                    // TODO: Set device parameter
                    // self.get_dev
                },
                Sb     => {
                    let (V(h), T(p), V(v)) = reducer.try_into()?;
                    self.dev_network_write(h as i64, &p, v)?;
                },
                // ================================================================================
                // Flow Control, Branches and Jumps
                // ================================================================================
                Bap    => {
                    let (V(a), V(b), V(c), V(l)) = reducer.try_into()?;
                    let cond = (a - b).abs() <= (c * a.abs().max(b.abs()).max(EPS));
                    jumped = self.jump_helper(l, false, false, cond)?;
                },
                Bapal  => {
                    let (V(a), V(b), V(c), V(l)) = reducer.try_into()?;
                    let cond = (a - b).abs() <= (c * a.abs().max(b.abs()).max(EPS));
                    jumped = self.jump_helper(l, false, true, cond)?;
                },
                Bapz   => {
                    // TODO: Figure out what the (ap/na)z functions are supposed to do
                },
                Bapzal => {
                    // TODO: Figure out what the (ap/na)z functions are supposed to do
                },
                Beq    => {
                    let (V(a), V(b), V(l)) = reducer.try_into()?;
                    jumped = self.jump_helper(l, false, false, a == b)?;
                },
                Beqal  => {
                    let (V(a), V(b), V(l)) = reducer.try_into()?;
                    jumped = self.jump_helper(l, false, true, a == b)?;
                },
                Beqz   => {
                    let (V(a), V(l)) = reducer.try_into()?;
                    jumped = self.jump_helper(l, false, false, a == 0.0)?;
                },
                Beqzal => {
                    let (V(a), V(l)) = reducer.try_into()?;
                    jumped = self.jump_helper(l, false, true, a == 0.0)?;
                },
                Bge    => {
                    let (V(a), V(b), V(l)) = reducer.try_into()?;
                    jumped = self.jump_helper(l, false, false, a >= b)?;
                },
                Bgeal  => {
                    let (V(a), V(b), V(l)) = reducer.try_into()?;
                    jumped = self.jump_helper(l, false, true, a >= b)?;
                },
                Bgez   => {
                    let (V(a), V(l)) = reducer.try_into()?;
                    jumped = self.jump_helper(l, false, false, a >= 0.0)?;
                },
                Bgezal => {
                    let (V(a), V(l)) = reducer.try_into()?;
                    jumped = self.jump_helper(l, false, true, a >= 0.0)?;
                },
                Bgt    => {
                    let (V(a), V(b), V(l)) = reducer.try_into()?;
                    jumped = self.jump_helper(l, false, false, a > b)?;
                },
                Bgtal  => {
                    let (V(a), V(b), V(l)) = reducer.try_into()?;
                    jumped = self.jump_helper(l, false, true, a > b)?;
                },
                Bgtz   => {
                    let (V(a), V(l)) = reducer.try_into()?;
                    jumped = self.jump_helper(l, false, false, a > 0.0)?;
                },
                Bgtzal => {
                    let (V(a), V(l)) = reducer.try_into()?;
                    jumped = self.jump_helper(l, false, true, a > 0.0)?;
                },
                Ble    => {
                    let (V(a), V(b), V(l)) = reducer.try_into()?;
                    jumped = self.jump_helper(l, false, false, a <= b)?;
                },
                Bleal  => {
                    let (V(a), V(b), V(l)) = reducer.try_into()?;
                    jumped = self.jump_helper(l, false, true, a <= b)?;
                },
                Blez   => {
                    let (V(a), V(l)) = reducer.try_into()?;
                    jumped = self.jump_helper(l, false, false, a <= 0.0)?;
                },
                Blezal => {
                    let (V(a), V(l)) = reducer.try_into()?;
                    jumped = self.jump_helper(l, false, true, a <= 0.0)?;
                },
                Blt    => {
                    let (V(a), V(b), V(l)) = reducer.try_into()?;
                    jumped = self.jump_helper(l, false, false, a < b)?;
                },
                Bltal  => {
                    let (V(a), V(b), V(l)) = reducer.try_into()?;
                    jumped = self.jump_helper(l, false, true, a < b)?;
                },
                Bltz   => {
                    let (V(a), V(l)) = reducer.try_into()?;
                    jumped = self.jump_helper(l, false, false, a < 0.0)?;
                },
                Bltzal => {
                    let (V(a), V(l)) = reducer.try_into()?;
                    jumped = self.jump_helper(l, false, true, a < 0.0)?;
                },
                Bna    => {
                    let (V(a), V(b), V(c), V(l)) = reducer.try_into()?;
                    let cond = (a - b).abs() > (c * a.abs().max(b.abs()).max(EPS));
                    jumped = self.jump_helper(l, false, false, cond)?;
                },
                Bnaal  => {
                    let (V(a), V(b), V(c), V(l)) = reducer.try_into()?;
                    let cond = (a - b).abs() > (c * a.abs().max(b.abs()).max(EPS));
                    jumped = self.jump_helper(l, false, true, cond)?;
                },
                Bnaz   => {
                    // TODO: Figure out what the (ap/na)z functions are supposed to do
                },
                Bnazal => {
                    // TODO: Figure out what the (ap/na)z functions are supposed to do
                },
                Bne    => {
                    let (V(a), V(b), V(l)) = reducer.try_into()?;
                    jumped = self.jump_helper(l, false, false, a != b)?;
                },
                Bneal  => {
                    let (V(a), V(b), V(l)) = reducer.try_into()?;
                    jumped = self.jump_helper(l, false, true, a != b)?;
                },
                Bnez   => {
                    let (V(a), V(l)) = reducer.try_into()?;
                    jumped = self.jump_helper(l, false, false, a != 0.0)?;
                },
                Bnezal => {
                    let (V(a), V(l)) = reducer.try_into()?;
                    jumped = self.jump_helper(l, false, true, a != 0.0)?;
                },
                Brap   => {
                    let (V(a), V(b), V(c), V(l)) = reducer.try_into()?;
                    let cond = (a - b).abs() <= (c * a.abs().max(b.abs()).max(EPS));
                    jumped = self.jump_helper(l, true, false, cond)?;
                },
                Brapz  => {
                    // TODO: Figure out what the (ap/na)z functions are supposed to do
                },
                Breq   => {
                    let (V(a), V(b), V(l)) = reducer.try_into()?;
                    jumped = self.jump_helper(l, true, false, a == b)?;
                },
                Breqz  => {
                    let (V(a), V(l)) = reducer.try_into()?;
                    jumped = self.jump_helper(l, true, false, a == 0.0)?;
                },
                Brge   => {
                    let (V(a), V(b), V(l)) = reducer.try_into()?;
                    jumped = self.jump_helper(l, true, false, a >= b)?;
                },
                Brgez  => {
                    let (V(a), V(l)) = reducer.try_into()?;
                    jumped = self.jump_helper(l, true, false, a >= 0.0)?;
                },
                Brgt   => {
                    let (V(a), V(b), V(l)) = reducer.try_into()?;
                    jumped = self.jump_helper(l, true, false, a > b)?;
                },
                Brgtz  => {
                    let (V(a), V(l)) = reducer.try_into()?;
                    jumped = self.jump_helper(l, true, false, a > 0.0)?;
                },
                Brle   => {
                    let (V(a), V(b), V(l)) = reducer.try_into()?;
                    jumped = self.jump_helper(l, true, false, a <= b)?;
                },
                Brlez  => {
                    let (V(a), V(l)) = reducer.try_into()?;
                    jumped = self.jump_helper(l, true, false, a <= 0.0)?;
                },
                Brlt   => {
                    let (V(a), V(b), V(l)) = reducer.try_into()?;
                    jumped = self.jump_helper(l, true, false, a < b)?;
                },
                Brltz  => {
                    let (V(a), V(l)) = reducer.try_into()?;
                    jumped = self.jump_helper(l, true, false, a < 0.0)?;
                },
                Brna   => {
                    let (V(a), V(b), V(c), V(l)) = reducer.try_into()?;
                    let cond = (a - b).abs() > (c * a.abs().max(b.abs()).max(EPS));
                    jumped = self.jump_helper(l, true, false, cond)?;
                },
                Brnaz  => {
                    // TODO: Figure out what the (ap/na)z functions are supposed to do
                },
                Brne   => {
                    let (V(a), V(b), V(l)) = reducer.try_into()?;
                    jumped = self.jump_helper(l, true, false, a != b)?;
                },
                Brnez  => {
                    let (V(a), V(l)) = reducer.try_into()?;
                    jumped = self.jump_helper(l, true, false, a != 0.0)?;
                },
                J      => {
                    let (V(l),) = reducer.try_into()?;
                    jumped = self.jump_helper(l, false, false, true)?;
                },
                Jal    => {
                    let (V(l),) = reducer.try_into()?;
                    jumped = self.jump_helper(l, false, true, true)?;
                },
                Jr     => {
                    let (V(l),) = reducer.try_into()?;
                    jumped = self.jump_helper(l, true, false, true)?;
                },
                // ================================================================================
                // Variable Selection
                // ================================================================================
                Sap    => {},
                Sapz   => {},
                Sdns   => {},
                Sdse   => {},
                Select => {},
                Seq    => {
                    let (M(r), V(a), V(b)) = reducer.try_into()?;
                    self.set_mem_helper(r, 1.0, a == b)?;
                },
                Seqz   => {
                },
                Sge    => {},
                Sgez   => {},
                Sgt    => {},
                Sgtz   => {},
                Sle    => {},
                Slez   => {},
                Slt    => {},
                Sltz   => {},
                Sna    => {},
                Snaz   => {},
                Sne    => {},
                Snez   => {},
                // ================================================================================
                // Mathematical Operations
                // ================================================================================
                Abs    => {},
                Acos   => {},
                Add    => {
                    let (M(r), V(a), V(b)) = reducer.try_into()?;
                    self.set_mem(r, a + b)?;
                },
                Asin   => {},
                Atan   => {},
                Ceil   => {},
                Cos    => {},
                Div    => {},
                Exp    => {},
                Floor  => {},
                Log    => {},
                Max    => {},
                Min    => {},
                Mod    => {},
                Mul    => {},
                Rand   => {},
                Round  => {},
                Sin    => {},
                Sqrt   => {},
                Sub    => {},
                Tan    => {},
                Trunc  => {},
                // ================================================================================
                // Logic
                // ================================================================================
                And    => {
                    let (M(r), V(a), V(b)) = reducer.try_into()?;
                    self.set_mem(r, bool_to_val((a > 0.0) || (b > 0.0)))?;
                },
                Nor    => {
                    let (M(r), V(a), V(b)) = reducer.try_into()?;
                    self.set_mem(r, bool_to_val(!((a > 0.0) || (b > 0.0))))?;
                },
                Or     => {
                    let (M(r), V(a), V(b)) = reducer.try_into()?;
                    self.set_mem(r, bool_to_val((a > 0.0) || (b > 0.0)))?;
                },
                Xor    => {
                    let (M(r), V(a), V(b)) = reducer.try_into()?;
                    self.set_mem(r, bool_to_val(a != b))?;
                },
                // ================================================================================
                // Stack
                // ================================================================================
                Peek   => {
                    let (M(r),) = reducer.try_into()?;
                    let v = self.peek()?;
                    self.set_mem(r, v)?;
                },
                Pop    => {
                    let (M(r),) = reducer.try_into()?;
                    let v = self.pop()?;
                    self.set_mem(r, v)?;
                },
                Push   => {
                    let (V(v),) = reducer.try_into()?;
                    self.push(v)?;
                },
                // ================================================================================
                // Misc
                // ================================================================================
                Alias  => {
                    let (T(a), R(r)) = reducer.try_into()?;
                    self.set_alias(a, r);
                },
                Define => {
                    let (T(a), V(v)) = reducer.try_into()?;
                    self.set_alias(a, AliasKind::Def(v));
                },
                Hcf    => {
                    return Err(ICStateError::HaltAndCatchFire);
                },
                Move   => {
                    let (M(r), V(a)) = reducer.try_into()?;
                    self.set_mem(r, a)?;
                },
                Sleep  => {
                    let (V(v),) = reducer.try_into()?;
                    return Ok(ExecResult::Sleep(v));
                },
                Yield  => {
                    return Ok(ExecResult::Yield);
                },
                // ================================================================================
                // Label
                // ================================================================================
                Label  => {
                    let (T(l),) = reducer.try_into()?;
                    self.set_alias(l, AliasKind::Label(*i));
                },
            };
        }
        Ok(ExecResult::Normal(jumped))
    }
}

impl<'dk, const STACKSIZE: usize> Display for ICState<'dk, STACKSIZE> {
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
            (0..STACKSIZE).step_by(PER_ROW).map(|_| {
                join(
                    self.stack
                        .iter()
                        .skip(i * PER_ROW)
                        .take(PER_ROW)
                        .map(|v| format!("{:.2}", v)),
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

impl<'dk> Default for ICState<'dk, 512> {
    fn default() -> Self {
        Self::new(18, 6)
            .with_alias("sp", AliasKind::MemId(16))
            .with_alias("ra", AliasKind::MemId(17))
    }
}
