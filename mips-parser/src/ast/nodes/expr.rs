use std::iter::once;
use std::{fmt, fmt::Display};

use itertools::join;
use pest::iterators::Pair;

use crate::ast::{FirstInner, AstError, AstResult};
use crate::Rule;

use super::{Arg, Func};

/// Expression node.
///
/// Stationeers MIPS code consists of only expressions, in the form of labels
/// or function calls.
#[derive(Clone, PartialEq, Debug)]
pub struct Expr(pub Func, pub Vec<Arg>);

impl Expr {
    /// New expression node from Pest `func` pair.
    pub fn try_from_pair(pair: Pair<Rule>) -> AstResult<Option<Self>> {
        let expr = match pair.as_rule() {
            Rule::expr => {
                pair.first_inner().map_or(Ok(None), |func_pair| {
                    let func = Func::try_from_rule(func_pair.as_rule())?;
                    let args = func_pair
                        .into_inner()
                        .map(Arg::try_from_pair)
                        .collect::<AstResult<Vec<Arg>>>()?;
                    Ok(Some(Expr(func, args)))
                })?
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
