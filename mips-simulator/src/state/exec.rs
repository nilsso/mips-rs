#![allow(unused_macros)]
#[allow(unused_imports)]
use std::convert::TryFrom;

use mips_parser::prelude::*;

use crate::state::{AliasKind, ICState, ICStateResult};
use crate::Line;

macro_rules! mvv {
    ($args:ident) => {{
        let i = $args.mem(0)?;
        let a = $args.val(1)?;
        let b = $args.val(2)?;
        (i, a, b)
    }};
}

macro_rules! di {
    ($args:ident) => {{
        let d = $args.dev(0)?;
        let i = $args.index(1)?;
        (d, i)
    }};
}

macro_rules! dv {
    ($args:ident) => {{
        let d = $args.dev(0)?;
        let v = $args.val(1)?;
        (d, v)
    }};
}

macro_rules! vvv {
    ($args:ident) => {{
        let a = $args.val(0)?;
        let b = $args.val(1)?;
        let c = $args.val(2)?;
        (a, b, c)
    }};
}

macro_rules! i {
    ($args:ident) => {{
        $args.val(0)? as isize
    }};
}

macro_rules! vvi {
    ($args:ident) => {{
        let a = $args.val(0)?;
        let b = $args.val(1)?;
        let i = $args.index(2)?;
        (a, b, i)
    }};
}

impl<'dk> ICState<'dk> {
    pub fn exec_line(&mut self, line: &Line) -> ICStateResult<bool> {
        use Func::*;

        let mut jumped = false;
        if let Line::Expr(i, expr) = line {
            let func = &expr.0;
            let args = self.arg_reducer(&expr.1);

            #[rustfmt::skip]
            match func {
                // Device IO
                Bdns   => {
                    let (d, l) = dv!(args);
                    jumped = self.jump_helper(l, false, false, !self.is_dev_set(d)?)?;
                },
                Bdnsal => {
                    let (d, l) = dv!(args);
                    jumped = self.jump_helper(l, false, true, !self.is_dev_set(d)?)?;
                },
                Bdse   => {
                    let (d, l) = dv!(args);
                    jumped = self.jump_helper(l, false, false, self.is_dev_set(d)?)?;
                },
                Bdseal => {
                    let (d, l) = dv!(args);
                    jumped = self.jump_helper(l, false, true, self.is_dev_set(d)?)?;
                },
                Brdns  => {
                    let (d, l) = dv!(args);
                    jumped = self.jump_helper(l, true, false, !self.is_dev_set(d)?)?;
                },
                Brdse  => {
                    let (d, l) = dv!(args);
                    jumped = self.jump_helper(l, true, false, self.is_dev_set(d)?)?;
                },
                L      => {
                    let r = args.mem(0)?;
                    let d = args.dev(1)?;
                    let t = args.tkn(2)?;
                    let dev = self.get_dev(d)?;
                    let param_value = dev.read(t)?;
                    self.set_mem(r, param_value)?;
                },
                Lb     => {},
                Lr     => {},
                Ls     => {},
                S      => {},
                Sb     => {
                    let h = args.val(0)?;
                    let t = args.tkn(1)?;
                    let v = args.val(2)?;
                    self.dev_network_write(h as i64, &t, v)?;
                },
                // Flow Control, Branches and Jumps
                Bap    => {},
                Bapal  => {},
                Bapz   => {},
                Bapzal => {},
                Beq    => {},
                Beqal  => {},
                Beqz   => {},
                Beqzal => {},
                Bge    => {},
                Bgeal  => {},
                Bgez   => {},
                Bgezal => {},
                Bgt    => {},
                Bgtal  => {},
                Bgtz   => {},
                Bgtzal => {},
                Ble    => {},
                Bleal  => {},
                Blez   => {},
                Blezal => {},
                Blt    => {
                    let a = args.val(0)?;
                    let b = args.val(1)?;
                    let c = args.val(2)?;
                    jumped = self.jump_helper(c, false, false, a < b)?;
                },
                Bltal  => {},
                Bltz   => {},
                Bltzal => {},
                Bna    => {},
                Bnaal  => {},
                Bnaz   => {},
                Bnazal => {},
                Bne    => {},
                Bneal  => {},
                Bnez   => {},
                Bnezal => {},
                Brap   => {},
                Brapz  => {},
                Breq   => {},
                Breqz  => {},
                Brge   => {},
                Brgez  => {},
                Brgt   => {},
                Brgtz  => {},
                Brle   => {},
                Brlez  => {},
                Brlt   => {
                    let (a, b, l) = vvv!(args);
                    jumped = self.jump_helper(l, true, false, a < b)?;
                },
                Brltz  => {},
                Brna   => {},
                Brnaz  => {},
                Brne   => {},
                Brnez  => {},
                J      => {
                    let l = args.val(0)?;
                    jumped = self.jump_helper(l, false, false, true)?;
                },
                Jal    => {
                    let l = args.val(0)?;
                    jumped = self.jump_helper(l, false, true, true)?;
                },
                Jr     => {
                    let l = args.val(0)?;
                    jumped = self.jump_helper(l, true, false, true)?;
                },
                // Variable Selection
                Sap    => {},
                Sapz   => {},
                Sdns   => {},
                Sdse   => {},
                Select => {},
                Seq    => {},
                Seqz   => {},
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
                // Mathematical Operations
                Abs    => {},
                Acos   => {},
                Add    => {
                    let (i, a, b) = mvv!(args);
                    self.set_mem(i, a + b)?;
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
                // Logic
                And    => {
                    let (i, a, b) = mvv!(args);
                    self.set_mem(i, bool_to_val((a > 0.0) || (b > 0.0)))?;
                },
                Nor    => {
                    let (i, a, b) = mvv!(args);
                    self.set_mem(i, bool_to_val(!((a > 0.0) || (b > 0.0))))?;
                },
                Or     => {
                    let (i, a, b) = mvv!(args);
                    self.set_mem(i, bool_to_val((a > 0.0) || (b > 0.0)))?;
                },
                Xor    => {
                    let (i, a, b) = mvv!(args);
                    self.set_mem(i, bool_to_val(a != b))?;
                },
                // Stack
                Peek   => {},
                Pop    => {},
                Push   => {},
                // Misc
                Alias  => {
                    let t = args.tkn(0)?;
                    let a = args.reg(1)?;
                    self.set_alias(t, a);
                },
                Define => {},
                Hcf    => {},
                Move   => {
                    let m = args.mem(0)?;
                    let v = args.val(1)?;
                    self.set_mem(m, v)?;
                },
                Sleep  => {},
                Yield  => {},
                // Label
                Label  => {
                    let l = args.tkn(0)?;
                    self.set_alias(l, AliasKind::Label(*i));
                },
            };
        }
        Ok(jumped)
    }
}

#[inline]
fn bool_to_val(b: bool) -> f64 {
    if b {
        1.0
    } else {
        0.0
    }
}
