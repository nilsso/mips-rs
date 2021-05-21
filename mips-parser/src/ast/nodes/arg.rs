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
            Rule::mem => Arg::ArgMem(Memory::new(pair)),
            Rule::dev => Arg::ArgDev(Device::new(pair)),
            Rule::reg => Arg::new(pair.inner()),
            Rule::value => Arg::ArgVal(Value::new(pair)),
            Rule::num => Arg::ArgVal(Value::ValLit(pair.as_str().parse().unwrap())),
            Rule::token => Arg::ArgToken(pair.as_str().into()),
            _ => unreachable!(),
        }
    }
}

