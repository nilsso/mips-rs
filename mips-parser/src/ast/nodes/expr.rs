use pest::iterators::Pair;

use crate::Rule;

use super::{Function, Arg};

/// Expression node.
///
/// Stationeers MIPS code consists of only expressions, in the form of labels
/// or function calls.
#[derive(PartialEq, Debug)]
pub enum Expr {
    ExprLabel(String),
    ExprFunc(Function, Vec<Arg>),
}

impl Expr {
    /// New expression node from Pest pair.
    pub fn new(pair: Pair<Rule>) -> Self {
        match pair.as_rule() {
            Rule::label => {
                let inner = pair.into_inner().next().unwrap();
                Expr::ExprLabel(inner.as_str().into())
            }
            Rule::fun => {
                let inner = pair.into_inner().next().unwrap();
                let f = Function::new(inner.as_rule());
                let args = inner.into_inner().map(|arg| Arg::new(arg)).collect();
                Expr::ExprFunc(f, args)
            }
            _ => unreachable!(),
        }
    }
}

