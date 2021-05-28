// #![feature(result_flattening)]
// #![feature(map_into_keys_values)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(dead_code)]
// use mips_parser::prelude::*;
// use mips_simulator::prelude::*;
use std::borrow::Borrow;
use std::convert::{TryFrom, TryInto};

#[derive(Clone, Debug)]
struct Mem(i32);

#[derive(Clone, Debug)]
struct Val(i32);

#[derive(Clone, Debug)]
enum Arg {
    Mem(i32),
    Val(i32),
}

impl TryFrom<Arg> for Mem {
    type Error = ();

    fn try_from(arg: Arg) -> Result<Mem, ()> {
        match arg {
            Arg::Mem(x) => Ok(Mem(x)),
            _ => Err(()),
        }
    }
}

impl TryFrom<Arg> for Val {
    type Error = ();

    fn try_from(arg: Arg) -> Result<Self, ()> {
        match arg {
            Arg::Val(x) => Ok(Self(x)),
            _ => Err(()),
        }
    }
}

struct Args(Vec<Arg>);

impl<A> TryFrom<&Args> for (A,)
where
    A: TryFrom<Arg, Error = ()>,
{
    type Error = ();

    fn try_from(args: &Args) -> Result<(A,), ()> {
        let args = &args.0;
        if args.len() == 1 {
            let a: A = args[0].clone().try_into()?;
            Ok((a,))
        } else {
            Err(())
        }
    }
}

impl<A, B> TryFrom<&Args> for (A, B)
where
    A: TryFrom<Arg, Error = ()>,
    B: TryFrom<Arg, Error = ()>,
{
    type Error = ();

    fn try_from(args: &Args) -> Result<(A, B), ()> {
        let args = &args.0;
        if args.len() == 2 {
            let a: A = args[0].clone().try_into()?;
            let b: B = args[1].clone().try_into()?;
            Ok((a, b))
        } else {
            Err(())
        }
    }
}

impl<A, B, C> TryFrom<&Args> for (A, B, C)
where
    A: TryFrom<Arg, Error = ()>,
    B: TryFrom<Arg, Error = ()>,
    C: TryFrom<Arg, Error = ()>,
{
    type Error = ();

    fn try_from(args: &Args) -> Result<(A, B, C), ()> {
        let args = &args.0;
        if args.len() == 3 {
            let a: A = args[0].clone().try_into()?;
            let b: B = args[1].clone().try_into()?;
            let c: C = args[2].clone().try_into()?;
            Ok((a, b, c))
        } else {
            Err(())
        }
    }
}

fn main() {
    let args = Args(vec![Arg::Val(1), Arg::Mem(2), Arg::Val(3)]);

    println!("{:?}", <(Val, Mem, Val)>::try_from(&args));
    // let res: Result<(Mem, Val), ()> = args.borrow().try_into();
    // println!("{:?}", res);
}
