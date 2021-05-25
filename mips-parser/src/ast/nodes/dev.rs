use std::{fmt, fmt::Display};

use pest::iterators::Pair;

use crate::ast::{Node, pair_to_int, AstError, AstResult, FirstInner};
use crate::Rule;

/// Device register node.
#[derive(Clone, PartialEq, Debug)]
pub enum Dev {
    DevLit(usize, usize),
    DevAlias(String),
}

impl Node for Dev {
    /// Rule [`Rule::dev`]
    const RULE: Rule = Rule::dev;

    fn try_from_pair(pair: Pair<Rule>) -> AstResult<Self> {
        let mem = match pair.as_rule() {
            Rule::reg | Rule::dev => Dev::try_from_pair(pair.first_inner()?)?,
            Rule::dev_lit => {
                let s = pair.as_str();
                let indirections = s.bytes().filter(|b| *b == b'r').count() as usize;
                let base_index = pair_to_int(pair.first_inner()?)?;
                Dev::DevLit(base_index, indirections)
            }
            Rule::alias => Dev::DevAlias(pair.as_str().into()),
            _ => return Err(AstError::Dev(format!("{:?}", pair))),
        };
        Ok(mem)
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
            }
            Dev::DevAlias(a) => write!(fmt, "{}", a),
        }
    }
}
