use std::iter::once;
use std::{fmt, fmt::Display};

use itertools::join;
use pest::iterators::Pair;

use crate::ast::{AstError, AstResult};
use crate::Rule;

use super::{Arg, Func};

/// Expression node.
///
/// Stationeers MIPS code consists of only expressions, in the form of labels
/// or function calls.
#[derive(Clone, PartialEq, Debug)]
pub struct Expr(pub Func, pub Vec<Arg>);

impl Expr {
    /// New expression node from Pest pair.
    pub fn from_pair(pair: Pair<Rule>) -> AstResult<Self> {
        let expr = match pair.as_rule() {
            Rule::expr => {
                let arg_pairs = pair.into_inner().next().unwrap();
                let func = Func::from_rule(arg_pairs.as_rule())?;
                let args = arg_pairs
                    .into_inner()
                    .map(Arg::from_pair)
                    .collect::<AstResult<Vec<Arg>>>()?;
                Expr(func, args)
            }
            _ => return Err(AstError::Expr(format!("{:?}", pair))),
        };
        Ok(expr)
    }
}

impl Display for Expr {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let func = &self.0;
        let args = &self.1;
        match func {
            Func::Label => write!(fmt, "{}:", args[0]),
            _ => {
                let f = format!("{}", func);
                let i = once(f).chain(args.iter().map(Arg::to_string));
                write!(fmt, "{}", join(i, " "))
            }
        }
    }
}
