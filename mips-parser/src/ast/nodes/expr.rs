use std::iter::once;
use std::{fmt, fmt::Display};

use itertools::join;
use pest::iterators::Pair;

use crate::ast::{FirstInner, Node, AstError, AstResult};
use crate::Rule;

use super::{Arg, Func};

/// Expression node.
///
/// Stationeers MIPS code consists of only expressions
/// in the form of labels or function calls
/// (though for simplicity's sake `label` is expressed as a node with no arguments).
#[derive(Clone, PartialEq, Debug)]
pub struct Expr(pub Func, pub Vec<Arg>);

impl<'a> Into<(&'a Func, &'a Vec<Arg>)> for &'a Expr {
    fn into(self) -> (&'a Func, &'a Vec<Arg>) {
        (&self.0, &self.1)
    }
}

impl Node for Expr {
    type Output = Self;

    /// Rule [`Rule::expr`]
    const RULE: Rule = Rule::expr;

    fn try_from_pair(pair: Pair<Rule>) -> AstResult<Self> {
        let expr = match pair.as_rule() {
            Rule::expr => {
                let inner = pair.first_inner()?;
                let func = Func::try_from_rule(inner.as_rule())?;
                let args = inner
                    .into_inner()
                    .map(Arg::try_from_pair)
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
