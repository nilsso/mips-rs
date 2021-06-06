use std::{fmt, fmt::Display};

use crate::ast_includes::*;

#[derive(Clone, PartialEq, Debug)]
pub enum Int {
    Lit(i64),
    Var(String),
}

impl<'i> AstNode<'i, Rule, MypsParser, MypsParserError> for Int {
    type Output = Self;

    // int = { int_lit | var }
    const RULE: Rule = Rule::int;

    fn try_from_pair(pair: Pair<Rule>) -> MypsParserResult<Self> {
        match pair.as_rule() {
            Rule::int => pair.first_inner()?.try_into_ast(),
            Rule::int_lit => Ok(Self::Lit(pair.as_str().parse()?)),
            Rule::var => Ok(Self::Var(pair.as_str().into())),
            _ => Err(MypsParserError::wrong_rule("an integer or variable", pair)),
        }
    }
}

impl Display for Int {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Int::Lit(n) => write!(f, "{}", n),
            Int::Var(v) => write!(f, "{}", v),
        }
    }
}
