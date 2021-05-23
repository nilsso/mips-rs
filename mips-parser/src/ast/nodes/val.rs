use std::{fmt, fmt::Display};

use super::mem::Mem;

use pest::iterators::Pair;

use crate::ast::{pair_to_float, FirstInner, AstError, AstResult};
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
    pub fn try_from_pair(pair: Pair<Rule>) -> AstResult<Self> {
        let val = match pair.as_rule() {
            Rule::val => Val::try_from_pair(pair.first_inner()?)?,
            Rule::num => Val::ValLit(pair_to_float(pair)?),
            Rule::mem => Val::ValMem(Mem::try_from_pair(pair)?),
            _ => return Err(AstError::Val(format!("{:?}", pair))),
        };
        Ok(val)
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
