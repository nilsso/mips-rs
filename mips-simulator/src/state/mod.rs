//! Integrated Circuit (IC10) simulator state.
#![allow(dead_code)]
use std::collections::HashMap;
use std::{fmt, fmt::Debug};

use mips_parser::prelude::*;
use util::is_as_inner;

use super::device::Device;

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
    AliasUnset(String),
    AliasWrongKind(String),
    OutOfBounds(OutOfBounds),
}

/// Shortcut type for ICState error results.
pub type ICStateResult<T> = Result<T, ICStateError>;

/// Alias kind type.
#[derive(Clone)]
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

impl Debug for AliasKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use AliasKind::*;
        match self {
            MemId(i) => write!(f, "MemId({})", i),
            DevId(i) => write!(f, "DevId({})", i),
            Label(i) => write!(f, "Label({})", i),
            Def(v)   => write!(f, "Def({})", v),
        }
    }
}

/// Integrated Circuit (IC10) simulator state.
#[derive(Clone, Debug)]
pub struct ICState<'dk> {
    mem: Vec<f64>,
    dev: Vec<Option<Device<'dk>>>,
    map: HashMap<String, AliasKind>,
    sp: usize,
    ra: usize,
    pub next_line_index: usize,
}

mod exec;

impl<'dk> ICState<'dk> {
    /// New MIPS state with specified number of memory reigsters and device (IO) ports.
    pub fn new(mem_size: usize, dev_size: usize) -> Self {
        Self {
            mem: vec![0_f64; mem_size + 2],
            dev: vec![None; dev_size],
            map: HashMap::new(),
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

    pub fn iter_dev(&self) -> impl Iterator<Item = &Option<Device>> {
        self.dev.iter()
    }

    /// Set an alias.
    pub fn set_alias(&mut self, a: String, r: AliasKind) {
        self.map.insert(a, r);
    }

    /// Try to set the next line index via an `isize`.
    ///
    /// Yields an out of bounds error if `i` is negative.
    pub fn try_jump(&mut self, i: isize) -> ICStateResult<()> {
        use std::convert::TryFrom;
        if let Ok(i) = usize::try_from(i) {
            self.next_line_index = i;
            Ok(())
        } else {
            Err(ICStateError::OutOfBounds(OutOfBounds::Line(i)))
        }
    }

    /// Try to set the next line index via an `isize`, saving the current index in `ra`.
    pub fn try_jump_save(&mut self, i: isize) -> ICStateResult<()> {
        let old_i = self.next_line_index;
        self.try_jump(i)?;
        self.mem[self.ra] = old_i as f64;
        Ok(())
    }

    /// Try to jump to line.
    ///
    /// * `i` - The line to jump to.
    /// * `relative` - Is the jump index relative?
    /// * `save` - Save the current next line in `ra`?
    /// * `condition` - Execute this jump?
    pub(self) fn try_jump_helper(
        &mut self,
        i: f64,
        relative: bool,
        save: bool,
        condition: bool,
    ) -> ICStateResult<bool> {
        if condition {
            let i = if relative {
                self.next_line_index as isize + i as isize
            } else {
                i as isize
            };
            if save {
                self.try_jump_save(i)?
            } else {
                self.try_jump(i)?
            }
        }
        Ok(condition)
    }

    /// Try to get an alias.
    pub(self) fn try_get_alias(&self, a: &String) -> ICStateResult<&AliasKind> {
        self.map.get(a).ok_or(ICStateError::AliasUnset(a.into()))
    }

    /// Try to get a memory register value reference.
    pub(self) fn try_get_mem(&self, i: usize) -> ICStateResult<&f64> {
        self.mem
            .get(i)
            .ok_or(ICStateError::OutOfBounds(OutOfBounds::Mem(i)))
    }

    /// Try to set a memory register value.
    pub(self) fn try_set_mem(&mut self, i: usize, v: f64) -> ICStateResult<()> {
        self.mem
            .get_mut(i)
            .ok_or(ICStateError::OutOfBounds(OutOfBounds::Mem(i)))
            .map(|m| *m = v)
    }

    // /// Try to get a device reference.
    // pub(self) fn try_get_dev(&self, i: usize) -> ICStateResult<&f64> {
    //     self.dev
    //         .get(i)
    //         .ok_or(ICStateError::OutOfBounds(OutOfBounds::Dev(i)))
    // }

    /// Try to get a mutable device reference.
    pub(self) fn try_get_dev_mut(&mut self, i: usize) -> ICStateResult<&mut Option<Device<'dk>>> {
        self.dev
            .get_mut(i)
            .ok_or(ICStateError::OutOfBounds(OutOfBounds::Dev(i)))
    }

    /// Try to set a device.
    pub fn try_set_dev(&mut self, i: usize, dev: Option<Device<'dk>>) -> ICStateResult<()> {
        self.try_get_dev_mut(i).map(|d| *d = dev)
    }

    /// Try to reduce a memory index by a number of indirections.
    pub(self) fn try_index_reduce(
        &self,
        i: usize,
        num_indirections: usize,
    ) -> ICStateResult<usize> {
        (0..num_indirections).try_fold(i, |i, _| self.try_get_mem(i).map(|j| i + (*j) as usize))
    }

    /// Try to reduce a memory node to a memory `usize` index.
    pub(self) fn try_mem_reduce(&self, mem: &Mem) -> ICStateResult<usize> {
        match mem {
            Mem::MemAlias(a) => self.try_get_alias(a)?.mem_id().cloned(),
            &Mem::MemLit(i, num_indirections) => self.try_index_reduce(i, num_indirections),
        }
    }

    /// Try to reduce a device node to a device `usize` index.
    pub(self) fn try_dev_reduce(&self, dev: &Dev) -> ICStateResult<usize> {
        match dev {
            Dev::DevAlias(a) => self.try_get_alias(a)?.dev_id().cloned(),
            &Dev::DevLit(i, num_indirections) => self.try_index_reduce(i, num_indirections),
        }
    }

    /// Try to reduce a value node to an `f64`.
    pub(self) fn try_val_reduce(&self, val: &Val) -> ICStateResult<f64> {
        match val {
            Val::ValLit(v) => Ok(*v),
            Val::ValMem(m) => self
                .try_mem_reduce(m)
                .and_then(|i| self.try_get_mem(i))
                .cloned(),
        }
    }

    pub(self) fn arg_reducer<'a, 'b>(&'a self, args: &'b Vec<Arg>) -> ArgReducer<'a, 'b> {
        ArgReducer { state: self, args }
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
        let mem = arg.mem().map_err(ICStateError::AstError)?;
        self.state.try_mem_reduce(mem)
    }

    pub fn dev(&self, i: usize) -> ICStateResult<usize> {
        let arg = self.get(i)?;
        let dev = arg.dev().map_err(ICStateError::AstError)?;
        self.state.try_dev_reduce(dev)
    }

    pub fn reg(&self, i: usize) -> ICStateResult<AliasKind> {
        let ak = match self.get(i)? {
            Arg::ArgMem(m) => {
                let i = self.state.try_mem_reduce(m)?;
                AliasKind::MemId(i)
            }
            Arg::ArgDev(d) => {
                let i = self.state.try_dev_reduce(d)?;
                AliasKind::DevId(i)
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
        let val = arg.val().map_err(ICStateError::AstError)?;
        self.state.try_val_reduce(val)
    }

    pub fn token(&self, i: usize) -> ICStateResult<String> {
        let arg = self.get(i)?;
        arg.token().map_err(ICStateError::AstError).cloned()
    }

    pub fn mvv(&self) -> ICStateResult<(usize, f64, f64)> {
        let i = self.mem(0)?;
        let a = self.val(1)?;
        let b = self.val(2)?;
        Ok((i, a, b))
    }
}
