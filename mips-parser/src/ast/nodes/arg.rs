use std::{fmt, fmt::Display};

use pest::iterators::Pair;

use crate::Rule;
use crate::InnerUnchecked;

use super::{Memory, Device, Value};

/// Function argument node.
#[derive(PartialEq, Debug)]
pub enum Arg {
    ArgMem(Memory),
    ArgDev(Device),
    ArgVal(Value),
    ArgToken(String),
}

impl Arg {
    pub fn new(pair: Pair<Rule>) -> Self {
        let rule = pair.as_rule();
        match rule {
            Rule::mem | Rule::mem_recur => Arg::ArgMem(Memory::new(pair)),
            Rule::dev | Rule::dev_recur => Arg::ArgDev(Device::new(pair)),
            Rule::reg => Arg::new(pair.inner()),
            Rule::value => Arg::ArgVal(Value::new(pair)),
            Rule::num => Arg::ArgVal(Value::ValLit(pair.as_str().parse().unwrap())),
            Rule::token => Arg::ArgToken(pair.as_str().into()),
            _ => unreachable!(),
        }
    }
}

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

