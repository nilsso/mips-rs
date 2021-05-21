use std::{fmt, fmt::Display};

use itertools::join;
use pest::iterators::Pair;

use crate::Rule;

use super::Expr;

/// Program node.
#[derive(Clone, PartialEq, Debug)]
pub struct Program(pub Vec<(usize, Expr)>);

impl Program {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        match pair.as_rule() {
            Rule::program => Self(
                pair.into_inner()
                    .enumerate()
                    .filter(|(_, inner)| {
                        let rule = inner.as_rule();
                        rule != Rule::blank
                    })
                    .map(|(i, inner)| (i, Expr::from_pair(inner)))
                    .collect(),
            ),
            _ => unreachable!(),
        }
    }

    pub fn empty() -> Self {
        Self(Vec::new())
    }

    /// Iterator over the expressions of this node.
    pub fn iter(&self) -> impl Iterator<Item = &(usize, Expr)> {
        self.0.iter()
    }

    pub fn push(&mut self, expr: Expr) {
        let i = self.0.len();
        self.0.push((i, expr));
    }
}

impl Display for Program {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        writeln!(fmt, "{}", join(self.iter().map(|(_, expr)| expr.to_string()), "\n"))
    }
}
