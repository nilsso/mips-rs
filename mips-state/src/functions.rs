use mips_parser::ast::nodes::Arg;
use crate::{MipsState, MipsStateError, AliasKind, Result};

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

#[inline]
pub fn f_and(args: &Vec<Arg>, state: &mut MipsState) -> Result<()> {
    let (i, a, b) = mem_and_vals(args, state)?;
    let v = bool_to_val((a > 0.0) || (b > 0.0));
    state.set_mem(i, v)
}

#[inline]
pub fn f_nor(args: &Vec<Arg>, state: &mut MipsState) -> Result<()> {
    f_or(args, state)
}

#[inline]
pub fn f_or(args: &Vec<Arg>, state: &mut MipsState) -> Result<()> {
    let (i, a, b) = mem_and_vals(args, state)?;
    let v = bool_to_val((a > 0.0) || (b > 0.0));
    state.set_mem(i, v)
}

#[inline]
pub fn f_xor(args: &Vec<Arg>, state: &mut MipsState) -> Result<()> {
    let (i, a, b) = mem_and_vals(args, state)?;
    let v = bool_to_val(a != b);
    state.set_mem(i, v)
}

#[inline]
pub fn f_alias(args: &Vec<Arg>, state: &mut MipsState) -> Result<()> {
    // TODO: Use the write functions
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
    Ok(())
}

#[inline]
pub fn f_move(args: &Vec<Arg>, state: &mut MipsState) -> Result<()> {
    state.arg_set_mem(&args[0], &args[1])
}

