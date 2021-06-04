use std::{fmt, fmt::Display};

use crate::ast_includes::*;

#[derive(Clone, PartialEq, Debug)]
pub struct Var(String);

impl<'i> AstNode<'i, Rule, MypsParser, MypsParserError> for Var {
    type Output = Self;

    const RULE: Rule = Rule::var;

    fn try_from_pair(pair: Pair<Rule>) -> MypsParserResult<Self> {
        match pair.as_rule() {
            Rule::var => Ok(Self(pair.as_str().into())),
            _ => Err(MypsParserError::wrong_rule("an integer or variable", pair)),
        }
    }
}

impl Display for Var {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

