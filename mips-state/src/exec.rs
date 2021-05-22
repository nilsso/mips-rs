use mips_parser::prelude::*;

use crate::{AliasKind, MipsState, MipsStateError, Result};
// use crate::functions::*;

#[inline]
fn bool_to_val(b: bool) -> f64 {
    if b {
        1.0
    } else {
        0.0
    }
}

#[inline]
fn mem_and_vals(args: &Vec<Arg>, state: &mut MipsState) -> Result<(usize, f64, f64)> {
    let i = state.arg_mem_reduce(&args[0])?;
    let a = state.arg_val_reduce(&args[1])?;
    let b = state.arg_val_reduce(&args[2])?;
    Ok((i, a, b))
}
macro_rules! f_logic {
    ($args:expr, $state:expr, $a:ident, $b:ident, $body:block) => {{
        let (i, a, b) = mem_and_vals($args, $state)?;
        let $a = a;
        let $b = b;
        let v = bool_to_val($body);
        $state.set_mem(i, v)?;
    }}
}

pub fn exec_expr(state: &mut MipsState, expr: &Expr) -> Result<()> {
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
        And    => f_logic!(args, state, a, b, { (a > 0.0) && (b > 0.0)  }),
        Nor    => f_logic!(args, state, a, b, { !((a > 0.0) || (b > 0.0)) }),
        Or     => f_logic!(args, state, a, b, { (a > 0.0) || (b > 0.0) }),
        Xor    => f_logic!(args, state, a, b, { a != b }),
        Peek   => {}
        Pop    => {}
        Push   => {}
        Alias  => {
            let a = args[0].token().unwrap();
            match &args[1] {
                Arg::ArgMem(m) => {
                    let i = state.mem_reduce(m)?;
                    state.map.insert(a, AliasKind::MemId(i));
                },
                Arg::ArgDev(d) => {
                    let i = state.dev_reduce(d)?;
                    state.map.insert(a, AliasKind::DevId(i));
                },
                _ => unreachable!(),
            };
        }
        Define => {}
        Hcf    => {}
        Move   => state.arg_set_mem(&args[0], &args[1])?,
        Sleep  => {}
        Yield  => {}
        Label  => {}
    };

    Ok(())
}

