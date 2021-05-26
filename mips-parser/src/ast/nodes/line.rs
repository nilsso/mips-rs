use pest::iterators::Pair;

use crate::ast::{AstError, AstResult, FirstInner, Node};
use crate::Rule;

use super::Expr;

#[derive(Clone, PartialEq, Debug)]
pub struct Line(Option<Expr>);

impl Node for Line {
    /// `None` if the pair is empty (i.e. a blank line, or one with comments only),
    /// else [`Some(Expr)`](#).
    type Output = Option<Expr>;

    /// Rule [`Rule::line`]
    const RULE: Rule = Rule::line;

    fn try_from_pair(pair: Pair<Rule>) -> AstResult<Self::Output> {
        match pair.as_rule() {
            Rule::line => Ok(if let Ok(inner) = pair.first_inner() {
                Some(Expr::try_from_pair(inner)?)
            } else {
                None
            }),
            _ => Err(AstError::Expr(format!("{:?}", pair))),
        }
    }
}
