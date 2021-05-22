//! Integrated Circuit (IC10) simulator state.
//!
//! A new simulator state is constructed via [`MipsState::new`],
//! and a few helper-builder methods exist for setting the
//! state memory register values ([`with_mem`][`MipsState::with_mem`]),
//! state devices ([`with_dev`][`MipsState::with_dev`]) and
//! aliases ([`with_alias`][`MipsState::with_alias`]) at the call site.
#![feature(bool_to_option)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(dead_code)]

use std::collections::HashMap;

use util::impl_is_as_inner;

use mips_parser::prelude::*;

mod exec;

pub mod prelude {
    pub use crate::{AliasKind, MipsState, MipsStateError, Result as MipsResult};
}

/// Alias kind type.
#[derive(Clone, PartialEq, Debug)]
pub enum AliasKind {
    MemId(usize),
    DevId(usize),
    Label(usize),
    Def(f64),
}

#[rustfmt::skip]
impl_is_as_inner!(AliasKind, AliasKind::MemId, is_mem_id, as_mem_id, mem_id, usize);
#[rustfmt::skip]
impl_is_as_inner!(AliasKind, AliasKind::DevId, is_dev_id, as_dev_id, dev_id, usize);
#[rustfmt::skip]
impl_is_as_inner!(AliasKind, AliasKind::Label, is_label,  as_label,  label,  usize);
#[rustfmt::skip]
impl_is_as_inner!(AliasKind, AliasKind::Def,   is_def,    as_def,    def,    f64);

/// State simulator error type.
#[derive(Debug)]
pub enum MipsStateError {
    AliasUnset,
    AliasWrongKind,
    ArgWrongKind,
    OutOfBounds,
}

pub type Result<T> = std::result::Result<T, MipsStateError>;

/// Integrated Circuit (IC10) simulator state.
#[derive(Clone, PartialEq, Debug)]
pub struct MipsState {
    mem: Vec<f64>,
    dev: Vec<f64>,
    map: HashMap<String, AliasKind>,
}

impl MipsState {
    /// New MIPS state with specified number of memory reigsters and device (IO) ports.
    pub fn new(mem_size: usize, dev_size: usize) -> Self {
        let mem = vec![0_f64; mem_size];
        let dev = vec![0_f64; dev_size];
        let map = HashMap::new();
        Self { mem, dev, map }
    }

    /// Builder helper to set a memory register.
    pub fn with_mem<V: Into<f64>>(mut self, i: usize, v: V) -> Self {
        self.mem.get_mut(i).map(|r| *r = v.into());
        self
    }

    /// Builder helper to set a device.
    pub fn with_dev(mut self, i: usize, v: f64) -> Self {
        self.dev.get_mut(i).map(|r| *r = v);
        self
    }

    /// Builder helper to set a map alias.
    pub fn with_alias<K: Into<String>>(mut self, k: K, a: AliasKind) -> Self {
        self.map.insert(k.into(), a);
        self
    }

    /// Try to get an alias.
    pub fn get_alias(&self, a: &String) -> Result<&AliasKind> {
        self.map.get(a).ok_or(MipsStateError::AliasUnset)
    }

    pub fn set_mem(&mut self, i: usize, v: f64) -> Result<()> {
        self.mem.get_mut(i).map(|r| *r = v).ok_or(MipsStateError::OutOfBounds)
    }

    pub fn set_dev(&mut self, i: usize, d: f64) -> Result<()> {
        self.dev.get_mut(i).map(|r| *r = d).ok_or(MipsStateError::OutOfBounds)
    }

    /// Try to reduce a memory index by a number of indirections.
    pub fn index_reduce(&self, i: usize, num_indirections: usize) -> Result<usize> {
        Ok((0..num_indirections).try_fold(i, |i, _| {
            self.mem
                .get(i)
                .map(|j| i + (*j) as usize)
                .ok_or(MipsStateError::OutOfBounds)
        })?)
    }

    /// Try to reduce a memory node to a memory `usize` index.
    pub fn mem_reduce(&self, mem: &Mem) -> Result<usize> {
        match mem {
            Mem::MemAlias(a) => self
                .get_alias(a)?
                .mem_id()
                .map(|i| i)
                .ok_or(MipsStateError::AliasWrongKind),
            &Mem::MemLit(i, num_indirections) => self.index_reduce(i, num_indirections),
        }
    }

    /// Try to reduce a device node to a device `usize` index.
    pub fn dev_reduce(&self, dev: &Dev) -> Result<usize> {
        match dev {
            Dev::DevAlias(a) => self
                .get_alias(a)?
                .dev_id()
                .map(|i| i)
                .ok_or(MipsStateError::AliasWrongKind),
            &Dev::DevLit(i, num_indirections) => self.index_reduce(i, num_indirections),
        }
    }

    /// Try to reduce a value node to an `f64`.
    pub fn val_reduce(&self, val: &Val) -> Result<f64> {
        match val {
            Val::ValLit(v) => Ok(*v),
            Val::ValMem(m) => self
                .mem
                .get(self.mem_reduce(m)?)
                .cloned()
                .ok_or(MipsStateError::OutOfBounds),
        }
    }

    pub fn arg_mem_reduce(&self, m_arg: &Arg) -> Result<usize> {
        Ok(self.mem_reduce(&m_arg.mem().ok_or(MipsStateError::ArgWrongKind)?)?)
    }

    /// Try to reduce a device argument to a device `usize` index.
    ///
    /// * `d_arg` - Device argument (expects [`Arg::ArgDev`])
    pub fn arg_dev_reduce(&self, d_arg: &Arg) -> Result<usize> {
        Ok(self.dev_reduce(&d_arg.dev().ok_or(MipsStateError::ArgWrongKind)?)?)
    }

    /// Try to reduce a value argument to an `f64`.
    ///
    /// * `v_arg` - Value argument (expects [`Arg::ArgVal`])
    pub fn arg_val_reduce(&self, v_arg: &Arg) -> Result<f64> {
        Ok(self.val_reduce(&v_arg.val().ok_or(MipsStateError::ArgWrongKind)?)?)
    }

    /// Try to set an alias.
    ///
    /// * `t_arg` - Token argument (expects [`Arg::ArgToken`])
    /// * `r_arg` - Register argument (expects [`Arg::ArgMem`] or [`Arg::ArgDev`])
    pub fn arg_set_alias(&mut self, t_arg: &Arg, r_arg: &Arg) -> Result<()> {
        let t = t_arg.token().ok_or(MipsStateError::ArgWrongKind)?;
        match r_arg {
            Arg::ArgMem(m) => {
                let i = self.mem_reduce(m)?;
                self.map.insert(t, AliasKind::MemId(i));
            },
            Arg::ArgDev(d) => {
                let i = self.dev_reduce(d)?;
                self.map.insert(t, AliasKind::DevId(i));
            },
            _ => return Err(MipsStateError::ArgWrongKind),
        };
        Ok(())
    }

    // pub fn set_val(&mut self, i: usize, v: f32) -> Result<(), Mips

    /// Try to set a memory register.
    ///
    /// * `m_arg` - Memory argument (expects [`Arg::ArgMem`])
    /// * `v_arg` - Value argument (expects [`Arg::ArgVal`])
    pub fn arg_set_mem(&mut self, m_arg: &Arg, v_arg: &Arg) -> Result<()> {
        let i = self.arg_mem_reduce(m_arg)?;
        let v = self.arg_val_reduce(v_arg)?;
        let dest = self.mem.get_mut(i).ok_or(MipsStateError::OutOfBounds)?;
        *dest = v;
        Ok(())
    }

    /// Try to set a device.
    ///
    /// * `d_arg` - Device argument (expects [`Arg::ArgDev`])
    /// * `dv_arg` - Device value argument
    pub fn arg_set_dev(&mut self, d_arg: &Arg, dv_arg: &Arg) -> Result<()> {
        let i = self.arg_dev_reduce(d_arg)?;
        let d = self.arg_val_reduce(dv_arg)?;
        let dest = self.mem.get_mut(i).ok_or(MipsStateError::OutOfBounds)?;
        *dest = d;
        Ok(())
    }

    /// Execute an expression.
    pub fn exec_expr(&mut self, expr: &Expr) -> Result<()> {
        crate::exec::exec_expr(self, expr)
    }
}

impl Default for MipsState {
    fn default() -> Self {
        Self::new(16, 6)
    }
}


