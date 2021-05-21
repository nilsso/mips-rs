#![feature(bool_to_option)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(dead_code)]

use std::collections::HashMap;

use util::impl_is_as_inner;

use mips_parser::prelude::*;

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

#[derive(Debug)]
pub enum ArgReduce<'a> {
    Mem(usize),
    Dev(usize),
    Value(f64),
    Token(&'a String),
}

#[derive(Debug)]
pub enum MipsStateError {
    AliasUnset,
    AliasWrongKind,
    ArgWrongKind,
    OutOfBounds,
}

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
    pub fn get_alias(&self, a: &String) -> Result<&AliasKind, MipsStateError> {
        self.map.get(a).ok_or(MipsStateError::AliasUnset)
    }

    /// Try to reduce a memory index by a number of indirections.
    pub fn index_reduce(&self, i: usize, num_indirections: usize) -> Result<usize, MipsStateError> {
        Ok((0..num_indirections).try_fold(i, |i, _| {
            self.mem
                .get(i)
                .map(|j| i + (*j) as usize)
                .ok_or(MipsStateError::OutOfBounds)
        })?)
    }

    /// Try to reduce a memory node to an alias kind.
    pub fn mem_reduce(&self, mem: &Mem) -> Result<usize, MipsStateError> {
        match mem {
            Mem::MemAlias(a) => self
                .get_alias(a)?
                .mem_id()
                .map(|i| i)
                .ok_or(MipsStateError::AliasWrongKind),
            &Mem::MemLit(i, num_indirections) => self.index_reduce(i, num_indirections),
        }
    }

    /// Try to reduce a device node to an alias kind.
    pub fn dev_reduce(&self, dev: &Dev) -> Result<usize, MipsStateError> {
        match dev {
            Dev::DevAlias(a) => self
                .get_alias(a)?
                .dev_id()
                .map(|i| i)
                .ok_or(MipsStateError::AliasWrongKind),
            &Dev::DevLit(i, num_indirections) => self.index_reduce(i, num_indirections),
        }
    }

    /// Try to reduce a value node to a value in memory (an `f64`).
    pub fn val_reduce(&self, val: &Val) -> Result<f64, MipsStateError> {
        match val {
            Val::ValLit(v) => Ok(*v),
            Val::ValMem(m) => self
                .mem
                .get(self.mem_reduce(m)?)
                .cloned()
                .ok_or(MipsStateError::OutOfBounds),
        }
    }

    pub fn arg_mem_reduce(&self, m_arg: &Arg) -> Result<usize, MipsStateError> {
        Ok(self.mem_reduce(&m_arg.mem().ok_or(MipsStateError::ArgWrongKind)?)?)
    }

    pub fn arg_dev_reduce(&self, d_arg: &Arg) -> Result<usize, MipsStateError> {
        Ok(self.dev_reduce(&d_arg.dev().ok_or(MipsStateError::ArgWrongKind)?)?)
    }

    pub fn arg_val_reduce(&self, v_arg: &Arg) -> Result<f64, MipsStateError> {
        Ok(self.val_reduce(&v_arg.val().ok_or(MipsStateError::ArgWrongKind)?)?)
    }

    /// Try to set an alias.
    ///
    /// * `t_arg` - Token argument (expects [`Arg::ArgToken`])
    /// * `r_arg` - Register argument (expects [`Arg::ArgMem`] or [`Arg::ArgDev`])
    pub fn arg_set_alias(&mut self, t_arg: &Arg, r_arg: &Arg) -> Result<(), MipsStateError> {
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

    pub fn arg_set_mem(&mut self, m_arg: &Arg, v_arg: &Arg) -> Result<(), MipsStateError> {
        let i = self.arg_mem_reduce(m_arg)?;
        let v = self.arg_val_reduce(v_arg)?;
        let dest = self.mem.get_mut(i).ok_or(MipsStateError::OutOfBounds)?;
        *dest = v;
        Ok(())
    }

    pub fn arg_set_dev(&mut self, d_arg: &Arg, v_arg: &Arg) -> Result<(), MipsStateError> {
        let i = self.arg_dev_reduce(d_arg)?;
        let d = self.arg_val_reduce(v_arg)?;
        let dest = self.mem.get_mut(i).ok_or(MipsStateError::OutOfBounds)?;
        *dest = d;
        Ok(())
    }

    // pub fn arg_reduce<'a>(&self, arg: &'a Arg) -> Result<ArgReduce<'a>, MipsStateError> {
    //     let ar = match arg {
    //         Arg::ArgMem(m) => ArgReduce::Mem(self.mem_reduce(m)?),
    //         // Arg::ArgDev(d)
    //         Arg::ArgVal(v) => ArgReduce::Value(self.val_reduce(v)?),
    //         Arg::ArgToken(t) => ArgReduce::Token(t),
    //         _ => unreachable!(),
    //     };
    //     Ok(ar)
    // }

    /// Execute an expression.
    pub fn exec_expr(&mut self, expr: &Expr) -> Result<(), MipsStateError> {
        use Func::*;

        let func = &expr.0;
        let args = &expr.1;

        #[rustfmt::skip]
        match func {
            Bdns   => {}
            Bdnsal => {}
            Bdse   => {}
            Bdseal => {}
            Brdns  => {}
            Brdse  => {}
            L      => {}
            Lb     => {}
            Lr     => {}
            Ls     => {}
            S      => {}
            Sb     => {}
            Bap    => {}
            Bapal  => {}
            Bapz   => {}
            Bapzal => {}
            Beq    => {}
            Beqal  => {}
            Beqz   => {}
            Beqzal => {}
            Bge    => {}
            Bgeal  => {}
            Bgez   => {}
            Bgezal => {}
            Bgt    => {}
            Bgtal  => {}
            Bgtz   => {}
            Bgtzal => {}
            Ble    => {}
            Bleal  => {}
            Blez   => {}
            Blezal => {}
            Blt    => {}
            Bltal  => {}
            Bltz   => {}
            Bltzal => {}
            Bna    => {}
            Bnaal  => {}
            Bnaz   => {}
            Bnazal => {}
            Bne    => {}
            Bneal  => {}
            Bnez   => {}
            Bnezal => {}
            Brap   => {}
            Brapz  => {}
            Breq   => {}
            Breqz  => {}
            Brge   => {}
            Brgez  => {}
            Brgt   => {}
            Brgtz  => {}
            Brle   => {}
            Brlez  => {}
            Brlt   => {}
            Brltz  => {}
            Brna   => {}
            Brnaz  => {}
            Brne   => {}
            Brnez  => {}
            J      => {}
            Jal    => {}
            Jr     => {}
            Sap    => {}
            Sapz   => {}
            Sdns   => {}
            Sdse   => {}
            Select => {}
            Seq    => {}
            Seqz   => {}
            Sge    => {}
            Sgez   => {}
            Sgt    => {}
            Sgtz   => {}
            Sle    => {}
            Slez   => {}
            Slt    => {}
            Sltz   => {}
            Sna    => {}
            Snaz   => {}
            Sne    => {}
            Snez   => {}
            Abs    => {}
            Acos   => {}
            Add    => {}
            Asin   => {}
            Atan   => {}
            Ceil   => {}
            Cos    => {}
            Div    => {}
            Exp    => {}
            Floor  => {}
            Log    => {}
            Max    => {}
            Min    => {}
            Mod    => {}
            Mul    => {}
            Rand   => {}
            Round  => {}
            Sin    => {}
            Sqrt   => {}
            Sub    => {}
            Tan    => {}
            Trunc  => {}
            And    => {}
            Nor    => {}
            Or     => {}
            Xor    => {}
            Peek   => {}
            Pop    => {}
            Push   => {}
            Alias  => {
                let a = args[0].token().unwrap();
                match &args[1] {
                    Arg::ArgMem(m) => {
                        let i = self.mem_reduce(m)?;
                        self.map.insert(a, AliasKind::MemId(i));
                    },
                    Arg::ArgDev(d) => {
                        let i = self.dev_reduce(d)?;
                        self.map.insert(a, AliasKind::DevId(i));
                    },
                    _ => unreachable!(),
                };
            }
            Define => {}
            Hcf    => {}
            Move   => self.arg_set_mem(&args[0], &args[1])?,
            Sleep  => {}
            Yield  => {}
            Label  => {}
        };

        Ok(())
    }
}

impl Default for MipsState {
    fn default() -> Self {
        Self::new(16, 6)
    }
}
