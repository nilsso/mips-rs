use std::{fmt, fmt::Display};

use pest::iterators::Pair;

use crate::Rule;
use crate::InnerUnchecked;

/// Memory register node.
#[derive(Clone, PartialEq, Debug)]
pub enum Mem {
    MemLit(usize, usize),
    MemAlias(String),
}

impl Mem {
    /// New Mem register node from Pest pair.
    ///
    /// Should be called on outer most `Rule::mem` pair,
    /// so that all scenarios (base, alias, indirect) are handled.
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        match pair.as_rule() {
            Rule::reg | Rule::mem => Mem::from_pair(pair.inner()),
            Rule::mem_lit => {
                let s = pair.as_str();
                let indirections = s.bytes().filter(|b| *b == b'r').count() as usize - 1;
                let inner = pair.inner();
                let base_index = inner.as_str().parse().unwrap();
                Mem::MemLit(base_index, indirections)
            },
            Rule::alias => Mem::MemAlias(pair.as_str().into()),
            _ => unreachable!(),
        }
    }
}

impl Display for Mem {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Mem::MemLit(i, j) => {
                for _ in 0..(j + 1) {
                    write!(fmt, "r")?;
                }
                write!(fmt, "{}", i)
            },
            Mem::MemAlias(a) => write!(fmt, "{}", a),
        }
    }
}

