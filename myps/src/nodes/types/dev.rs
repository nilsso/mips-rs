use std::{fmt, fmt::Display};

use crate::ast_includes::*;

/// Device node.
#[derive(Clone, PartialEq, Debug)]
pub enum Dev {
    Lit(usize, usize),
    Batch(i64),
    Var(String),
}

impl<'i> AstNode<'i, Rule, MypsParser, MypsParserError> for Dev {
    type Output = Self;

    // dev = _{ dev_batch | dev_lit | var }
    //     dev_batch = ${ int ~ ".all" }
    //     dev_lit   = ${ "d" ~ "r"* ~ !".all" ~ int_lit }
    const RULE: Rule = Rule::dev;

    fn try_from_pair(pair: Pair<Rule>) -> MypsParserResult<Self> {
        match pair.as_rule() {
            Rule::dev_lit => {
                let s = pair.as_str();
                let indirections = s.bytes().filter(|b| *b == b'r').count() as usize;
                let base_index = pair.first_inner()?.as_str().parse()?;
                Ok(Dev::Lit(base_index, indirections))
            }
            Rule::dev_batch => {
                let hash = pair.first_inner()?.as_str().parse()?;
                Ok(Self::Batch(hash))
            }
            Rule::var => Ok(Self::Var(pair.as_str().into())),
            _ => return Err(MypsParserError::wrong_rule("a device", pair)),
        }
    }
}

impl Display for Dev {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Dev::Lit(base, num_indirections) => {
                write!(f, "d")?;
                for _ in 0..*num_indirections {
                    write!(f, "r")?;
                }
                write!(f, "{}", base)
            },
            Dev::Batch(hash) => write!(f, "{}", hash),
            Dev::Var(v) => write!(f, "{}", v),
        }
    }
}
