//! Program node.
use std::{fmt, fmt::Display};

use itertools::join;
use pest::iterators::Pair;

use crate::ast::{AstError, AstResult, Node};
use crate::Rule;

use super::{Expr, Line};

/// Program node.
#[derive(Clone, PartialEq, Debug)]
pub struct Program(pub Vec<(usize, Expr)>);

impl Node for Program {
    /// Rule [`Rule::program`].
    const RULE: Rule = Rule::program;

    fn try_from_pair(pair: Pair<Rule>) -> AstResult<Self> {
        let program = match pair.as_rule() {
            Rule::program => {
                let results = pair
                    .into_inner()
                    .map(|line_pair| Line::try_from_pair(line_pair))
                    .collect::<AstResult<Vec<Option<Expr>>>>()?;
                let expressions = results
                    .into_iter()
                    .enumerate()
                    .filter_map(|(i, expr)| expr.map(|expr| (i, expr)))
                    .collect();
                Self(expressions)
            }
            _ => return Err(AstError::Program),
        };
        Ok(program)
    }
}

impl Program {
    pub fn new() -> Self {
        Program(Vec::new())
    }

    pub fn empty() -> Self {
        Self(Vec::new())
    }

    /// Iterator over the expressions of this node.
    pub fn iter(&self) -> impl Iterator<Item = &(usize, Expr)> {
        self.0.iter()
    }

    /// Consuming iterator.
    pub fn into_iter(self) -> impl Iterator<Item = (usize, Expr)> {
        self.0.into_iter()
    }

    pub fn push(&mut self, expr: Expr) {
        let i = self.0.len();
        self.0.push((i, expr));
    }
}

impl Display for Program {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let lines = join(self.iter().map(|(_, expr)| expr).map(Expr::to_string), "\n");
        writeln!(fmt, "{}", lines)
    }
}
