use std::{fmt, fmt::Display};
use std::convert::TryFrom;

use pest::iterators::Pair;

use crate::Rule;
use crate::ast::{Node, pair_to_int, FirstInner, AstError, AstResult};
use crate::prelude::Arg;

/// Memory register node.
#[derive(Clone, PartialEq, Debug)]
pub enum Mem {
    MemLit(usize, usize),
    MemAlias(String),
}

impl TryFrom<&Arg> for Mem {
    type Error = AstError;

    fn try_from(arg: &Arg) -> Result<Mem, AstError> {
        match arg {
            Arg::ArgMem(m) => Ok(m.clone()),
            _ => Err(AstError::WrongArg("".into())),
        }
    }
}

impl Node for Mem {
    /// Rule [`Rule::mem`].
    const RULE: Rule = Rule::mem;

    fn try_from_pair(pair: Pair<Rule>) -> AstResult<Self> {
        let mem = match pair.as_rule() {
            Rule::reg | Rule::mem => Mem::try_from_pair(pair.first_inner()?)?,
            Rule::mem_lit => {
                let s = pair.as_str();
                let indirections = s.bytes().filter(|b| *b == b'r').count() as usize - 1;
                let base_index = pair_to_int(pair.first_inner()?)?;
                Mem::MemLit(base_index, indirections)
            },
            Rule::alias => Mem::MemAlias(pair.as_str().into()),
            _ => return Err(AstError::Mem(format!("{:?}", pair))),
        };
        Ok(mem)
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

