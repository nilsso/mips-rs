use std::{fmt, fmt::Display};

use pest::iterators::Pair;

use util::impl_is_as_inner;

use crate::Rule;
use crate::InnerUnchecked;

use super::{Mem, Dev, Val};

/// Function argument node.
#[derive(Clone, PartialEq, Debug)]
pub enum Arg {
    ArgMem(Mem),
    ArgDev(Dev),
    ArgVal(Val),
    ArgToken(String),
}

impl Arg {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        let rule = pair.as_rule();
        match rule {
            Rule::reg => Arg::from_pair(pair.inner()),
            Rule::mem | Rule::mem_lit => Arg::ArgMem(Mem::from_pair(pair)),
            Rule::dev | Rule::dev_lit => Arg::ArgDev(Dev::from_pair(pair)),
            Rule::val => Arg::ArgVal(Val::from_pair(pair)),
            Rule::tkn => Arg::ArgToken(pair.as_str().into()),
            Rule::num => Arg::ArgVal(Val::ValLit(pair.as_str().parse().unwrap())),
            _ => unreachable!(),
        }
    }

    pub fn as_string(&self) -> &String {
        match self {
            Self::ArgToken(s) => s,
            _ => unreachable!(),
        }
    }
}

impl_is_as_inner!(Arg, Arg::ArgMem,   is_mem,   as_mem,   mem,   Mem);
impl_is_as_inner!(Arg, Arg::ArgDev,   is_dev,   as_dev,   dev,   Dev);
impl_is_as_inner!(Arg, Arg::ArgVal,   is_val,   as_val,   val,   Val);
impl_is_as_inner!(Arg, Arg::ArgToken, is_token, as_token, token, String);

impl Display for Arg {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Arg::ArgMem(m) => write!(fmt, "{}", m),
            Arg::ArgDev(d) => write!(fmt, "{}", d),
            Arg::ArgVal(v) => write!(fmt, "{}", v),
            Arg::ArgToken(t) => write!(fmt, "{}", t),
        }
    }
}

