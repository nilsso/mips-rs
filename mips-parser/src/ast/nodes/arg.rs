use std::{fmt, fmt::Display};

use pest::iterators::Pair;

use util::impl_is_as_inner;

use crate::Rule;
use crate::ast::{pair_to_float, FirstInner, AstError, AstResult};

use super::{Mem, Dev, Val};

/// Function argument node.
#[derive(Clone, PartialEq, Debug)]
pub enum Arg {
    ArgMem(Mem),
    ArgDev(Dev),
    ArgAlias(String),
    ArgVal(Val),
    ArgToken(String),
}

impl Arg {
    pub fn try_from_pair(pair: Pair<Rule>) -> AstResult<Self> {
        let rule = pair.as_rule();
        let arg = match rule {
            Rule::reg => Arg::try_from_pair(pair.first_inner()?)?,
            Rule::mem | Rule::mem_lit => Arg::ArgMem(Mem::try_from_pair(pair)?),
            Rule::dev | Rule::dev_lit => Arg::ArgDev(Dev::try_from_pair(pair)?),
            Rule::alias => Arg::ArgAlias(pair.as_str().into()),
            Rule::val => Arg::ArgVal(Val::try_from_pair(pair)?),
            Rule::tkn => Arg::ArgToken(pair.as_str().into()),
            Rule::num => Arg::ArgVal(Val::ValLit(pair_to_float(pair)?)),
            _ => return Err(AstError::Arg(format!("{:?}", pair))),
        };
        Ok(arg)
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
            Arg::ArgAlias(a) => write!(fmt, "{}", a),
            Arg::ArgVal(v) => write!(fmt, "{}", v),
            Arg::ArgToken(t) => write!(fmt, "{}", t),
        }
    }
}

