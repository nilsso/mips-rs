use std::{fmt, fmt::Display};

use itertools::join;
use pest::iterators::Pair;

use crate::Rule;
use crate::ast::{AstError, AstResult};

use super::Expr;

/// Program node.
#[derive(Clone, PartialEq, Debug)]
pub struct Program(pub Vec<(usize, Expr)>);

impl Program {
    pub fn from_pair(pair: Pair<Rule>) -> AstResult<Self> {
        let program = match pair.as_rule() {
            Rule::program => {
                let expressions = pair
                    .into_inner()
                    .enumerate()
                    .filter(|(_, expr_pair)| expr_pair.as_rule() != Rule::blank)
                    .map(|(i, expr_pair)| Expr::from_pair(expr_pair).map(|expr| (i, expr)))
                    .collect::<AstResult<Vec<(usize, Expr)>>>()?;
                Self(expressions)
            }
            _ => return Err(AstError::Program),
        };
        Ok(program)
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
        writeln!(
            fmt,
            "{}",
            join(self.iter().map(|(_, expr)| expr).map(Expr::to_string), "\n")
        )
    }
}
