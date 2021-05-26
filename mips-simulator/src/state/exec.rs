use mips_parser::prelude::*;

use crate::Line;
use crate::state::{ICState, ICStateResult, AliasKind};

impl<'dk> ICState<'dk> {
    pub fn try_exec_line(&mut self, line: &Line) -> ICStateResult<bool> {
        use Func::*;

        let mut jumped = false;
        if let Line::Expr(i, expr) = line {
            let func = &expr.0;
            let args = self.arg_reducer(&expr.1);

            #[rustfmt::skip]
            match func {
                // Device IO
                Bdns   => {},
                Bdnsal => {},
                Bdse   => {},
                Bdseal => {},
                Brdns  => {},
                Brdse  => {},
                L      => {},
                Lb     => {},
                Lr     => {},
                Ls     => {},
                S      => {},
                Sb     => {},
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
                Blt    => {},
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
                    let a = args.val(0)?;
                    let b = args.val(1)?;
                    let i = args.val(2)?;
                    // println!("{} < {} = {}", a, b, a < b);
                    jumped = self.try_jump_helper(i, true, false, a < b)?;
                    // let a = self.try_arg_val_reduce(&
                },
                Brltz  => {},
                Brna   => {},
                Brnaz  => {},
                Brne   => {},
                Brnez  => {},
                J      => {
                    let i = args.val(0)?;
                    jumped = self.try_jump_helper(i, false, false, true)?;
                },
                Jal    => {
                    let i = args.val(0)?;
                    jumped = self.try_jump_helper(i, false, true, true)?;
                },
                Jr     => {},
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
                    let (i, a, b) = args.mvv()?;
                    self.try_set_mem(i, a + b)?;
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
                    let (i, a, b) = args.mvv()?;
                    self.try_set_mem(i, bool_to_val((a > 0.0) || (b > 0.0)))?;
                },
                Nor    => {
                    let (i, a, b) = args.mvv()?;
                    self.try_set_mem(i, bool_to_val(!((a > 0.0) || (b > 0.0))))?;
                },
                Or     => {
                    let (i, a, b) = args.mvv()?;
                    self.try_set_mem(i, bool_to_val((a > 0.0) || (b > 0.0)))?;
                },
                Xor    => {
                    let (i, a, b) = args.mvv()?;
                    self.try_set_mem(i, bool_to_val(a != b))?;
                },
                // Stack
                Peek   => {},
                Pop    => {},
                Push   => {},
                // Misc
                Alias  => {
                    let t = args.token(0)?;
                    let a = args.reg(1)?;
                    self.set_alias(t, a);
                },
                Define => {},
                Hcf    => {},
                Move   => {
                    let m = args.mem(0)?;
                    let v = args.val(1)?;
                    self.try_set_mem(m, v)?;
                },
                Sleep  => {},
                Yield  => {},
                // Label
                Label  => {
                    let l = args.token(0)?;
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
