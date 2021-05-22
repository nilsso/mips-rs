//! IC10 state;

use std::collections::HashMap;

use mips_parser::prelude::*;

use util::impl_is_as_inner;

use crate::{ICError, ICResult};

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

/// Integrated Circuit (IC10) simulator state.
#[derive(Clone, PartialEq, Debug)]
pub struct ICState {
    pub mem: Vec<f64>,
    pub dev: Vec<f64>,
    pub map: HashMap<String, AliasKind>,
}

impl ICState {
    /// New MIPS state with specified number of memory reigsters and device (IO) ports.
    pub fn new(mem_size: usize, dev_size: usize) -> Self {
        let mem = vec![0_f64; mem_size + 2];
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
    pub fn get_alias(&self, a: &String) -> ICResult<&AliasKind> {
        self.map.get(a).ok_or(ICError::AliasUnset)
    }

    pub fn set_mem(&mut self, i: usize, v: f64) -> ICResult<()> {
        self.mem
            .get_mut(i)
            .map(|r| *r = v)
            .ok_or(ICError::OutOfBounds)
    }

    pub fn set_dev(&mut self, i: usize, d: f64) -> ICResult<()> {
        self.dev
            .get_mut(i)
            .map(|r| *r = d)
            .ok_or(ICError::OutOfBounds)
    }

    /// Try to reduce a memory index by a number of indirections.
    pub fn index_reduce(&self, i: usize, num_indirections: usize) -> ICResult<usize> {
        Ok((0..num_indirections).try_fold(i, |i, _| {
            self.mem
                .get(i)
                .map(|j| i + (*j) as usize)
                .ok_or(ICError::OutOfBounds)
        })?)
    }

    /// Try to reduce a memory node to a memory `usize` index.
    pub fn mem_reduce(&self, mem: &Mem) -> ICResult<usize> {
        match mem {
            Mem::MemAlias(a) => self
                .get_alias(a)?
                .mem_id()
                .map(|i| i)
                .ok_or(ICError::AliasWrongKind),
            &Mem::MemLit(i, num_indirections) => self.index_reduce(i, num_indirections),
        }
    }

    /// Try to reduce a device node to a device `usize` index.
    pub fn dev_reduce(&self, dev: &Dev) -> ICResult<usize> {
        match dev {
            Dev::DevAlias(a) => self
                .get_alias(a)?
                .dev_id()
                .map(|i| i)
                .ok_or(ICError::AliasWrongKind),
            &Dev::DevLit(i, num_indirections) => self.index_reduce(i, num_indirections),
        }
    }

    /// Try to reduce a value node to an `f64`.
    pub fn val_reduce(&self, val: &Val) -> ICResult<f64> {
        match val {
            Val::ValLit(v) => Ok(*v),
            Val::ValMem(m) => self
                .mem
                .get(self.mem_reduce(m)?)
                .cloned()
                .ok_or(ICError::OutOfBounds),
        }
    }

    pub fn arg_mem_reduce(&self, m_arg: &Arg) -> ICResult<usize> {
        Ok(self.mem_reduce(&m_arg.mem().ok_or(ICError::ArgWrongKind)?)?)
    }

    /// Try to reduce a device argument to a device `usize` index.
    ///
    /// * `d_arg` - Device argument (expects [`Arg::ArgDev`])
    pub fn arg_dev_reduce(&self, d_arg: &Arg) -> ICResult<usize> {
        Ok(self.dev_reduce(&d_arg.dev().ok_or(ICError::ArgWrongKind)?)?)
    }

    /// Try to reduce a value argument to an `f64`.
    ///
    /// * `v_arg` - Value argument (expects [`Arg::ArgVal`])
    pub fn arg_val_reduce(&self, v_arg: &Arg) -> ICResult<f64> {
        Ok(self.val_reduce(&v_arg.val().ok_or(ICError::ArgWrongKind)?)?)
    }

    /// Try to set an alias.
    ///
    /// * `t_arg` - Token argument (expects [`Arg::ArgToken`])
    /// * `r_arg` - Register argument (expects [`Arg::ArgMem`] or [`Arg::ArgDev`])
    pub fn arg_set_alias(&mut self, t_arg: &Arg, r_arg: &Arg) -> ICResult<()> {
        let t = t_arg.token().ok_or(ICError::ArgWrongKind)?;
        match r_arg {
            Arg::ArgMem(m) => {
                let i = self.mem_reduce(m)?;
                self.map.insert(t, AliasKind::MemId(i));
            }
            Arg::ArgDev(d) => {
                let i = self.dev_reduce(d)?;
                self.map.insert(t, AliasKind::DevId(i));
            }
            _ => return Err(ICError::ArgWrongKind),
        };
        Ok(())
    }

    // pub fn set_val(&mut self, i: usize, v: f32) -> ICResult<(), Mips

    /// Try to set a memory register.
    ///
    /// * `m_arg` - Memory argument (expects [`Arg::ArgMem`])
    /// * `v_arg` - Value argument (expects [`Arg::ArgVal`])
    pub fn arg_set_mem(&mut self, m_arg: &Arg, v_arg: &Arg) -> ICResult<()> {
        let i = self.arg_mem_reduce(m_arg)?;
        let v = self.arg_val_reduce(v_arg)?;
        let dest = self.mem.get_mut(i).ok_or(ICError::OutOfBounds)?;
        *dest = v;
        Ok(())
    }

    /// Try to set a device.
    ///
    /// * `d_arg` - Device argument (expects [`Arg::ArgDev`])
    /// * `dv_arg` - Device value argument
    pub fn arg_set_dev(&mut self, d_arg: &Arg, dv_arg: &Arg) -> ICResult<()> {
        let i = self.arg_dev_reduce(d_arg)?;
        let d = self.arg_val_reduce(dv_arg)?;
        let dest = self.mem.get_mut(i).ok_or(ICError::OutOfBounds)?;
        *dest = d;
        Ok(())
    }

    /// Execute an expression.
    pub fn exec_expr(&mut self, expr: &Expr) -> ICResult<()> {
        crate::exec::exec_expr(self, expr)
    }
}

impl Default for ICState {
    fn default() -> Self {
        Self::new(18, 6)
            .with_alias("sp", AliasKind::MemId(16))
            .with_alias("ra", AliasKind::MemId(17))
    }
}

