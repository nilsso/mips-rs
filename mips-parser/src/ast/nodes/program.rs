use pest::iterators::Pair;

use crate::Rule;

use super::Expr;

/// Program node.
#[derive(PartialEq, Debug)]
pub struct Program(pub Vec<(usize, Expr)>);

impl Program {
    pub fn new(pair: Pair<Rule>) -> Self {
        match pair.as_rule() {
            Rule::program => Self(
                pair.into_inner()
                    .enumerate()
                    .filter(|(_, inner)| inner.as_rule() != Rule::blank)
                    .map(|(i, inner)| (i, Expr::new(inner)))
                    .collect(),
            ),
            _ => unreachable!(),
        }
    }

    /// Iterator over the expressions of this node.
    pub fn iter(&self) -> impl Iterator<Item = &(usize, Expr)> {
        self.0.iter()
    }
}
