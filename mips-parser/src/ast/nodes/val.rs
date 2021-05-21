use std::{fmt, fmt::Display};

use super::mem::Mem;

use pest::iterators::Pair;

use crate::Rule;

/// Value node.
///
/// Values in MIPS expressions can be floating-point literals or those stored in state memory.
/// The former is encapsulated in [`ValLit(f64)`](Val::ValLit),
/// while the later involves reducing a [`Mem`] node to a `StateIndex::Mem(i)`
/// with which the ith value from state memory is obtained.
#[derive(Clone, PartialEq, Debug)]
pub enum Val {
    ValLit(f64),
    ValMem(Mem),
}

impl Val {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        match pair.as_rule() {
            Rule::val => {
                let inner = pair.into_inner().next().unwrap();
                Val::from_pair(inner)
            }
            Rule::num => {
                let x = pair.as_str().parse().unwrap();
                Val::ValLit(x)
            },
            Rule::mem => {
                Val::ValMem(Mem::from_pair(pair))
            },
            _ => unreachable!(),
        }
    }
}

impl Display for Val {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Val::ValLit(x) => write!(fmt, "{}", x),
            Val::ValMem(m) => write!(fmt, "{}", m),
        }
    }
}
