use std::{fmt, fmt::Display};
use std::iter::once;

use itertools::join;
use pest::iterators::Pair;

use crate::Rule;

use super::{Func, Arg};

/// Expression node.
///
/// Stationeers MIPS code consists of only expressions, in the form of labels
/// or function calls.
#[derive(Clone, PartialEq, Debug)]
pub struct Expr(pub Func, pub Vec<Arg>);

impl Expr {
    /// New expression node from Pest pair.
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        match pair.as_rule() {
            Rule::expr => {
                let inner = pair.into_inner().next().unwrap();
                let func = Func::from_rule(inner.as_rule());
                let args = inner.into_inner().map(|arg| Arg::from_pair(arg)).collect();
                Expr(func, args)
            }
            _ => unreachable!(),
        }
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
                let i = once(f).chain(args.iter().map(|arg| arg.to_string()));
                write!(fmt, "{}", join(i, " "))
            },
        }
    }
}
