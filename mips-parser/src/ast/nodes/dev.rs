use std::{fmt, fmt::Display};

use pest::iterators::Pair;

use crate::Rule;
use crate::InnerUnchecked;

/// Device register node.
#[derive(Clone, PartialEq, Debug)]
pub enum Dev {
    DevLit(usize, usize),
    DevAlias(String),
}

impl Dev {
    /// New device register node from Pest pair.
    ///
    /// Should be called on outer most `Rule::dev` pair,
    /// so that all scenarios (base, alias, indirect) are handled.
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        match pair.as_rule() {
            Rule::reg | Rule::dev => Dev::from_pair(pair.inner()),
            Rule::dev_lit => {
                let s = pair.as_str();
                let indirections = s.bytes().filter(|b| *b == b'r').count() as usize;
                let inner = pair.inner();
                let base_index = inner.as_str().parse().unwrap();
                Dev::DevLit(base_index, indirections)
            },
            Rule::alias => Dev::DevAlias(pair.as_str().into()),
            _ => unreachable!(),
        }
    }
}

impl Display for Dev {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Dev::DevLit(i, j) => {
                write!(fmt, "d")?;
                for _ in 0..*j {
                    write!(fmt, "r")?;
                }
                write!(fmt, "{}", i)
            },
            Dev::DevAlias(a) => write!(fmt, "{}", a),
        }
    }
}
