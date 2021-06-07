use std::{fmt, fmt::Display};

use crate::superprelude::*;

#[derive(Debug)]
pub enum Statement {
    AssignAlias(String, Dev),
    AssignValue(LValue, RValue),
    FunctionCall(String),
}

impl<'i> AstNode<'i, Rule, MypsParser, MypsLexerError> for Statement {
    type Output = Self;

    const RULE: Rule = Rule::stmt;

    fn try_from_pair(pair: Pair<Rule>) -> MypsLexerResult<Self> {
        match pair.as_rule() {
            Rule::stmt => pair.first_inner()?.try_into_ast(),
            Rule::assign_alias => {
                let mut inner_pairs = pair.into_inner();
                let alias = inner_pairs.next_pair()?.as_str().into();
                let dev = inner_pairs.next_pair()?.try_into_ast()?;
                Ok(Self::AssignAlias(alias, dev))
            }
            Rule::assign_value => {
                let mut inner_pairs = pair.into_inner();
                let l_value = inner_pairs.next_pair()?.try_into_ast()?;
                let r_value = inner_pairs.next_pair()?.try_into_ast()?;
                Ok(Self::AssignValue(l_value, r_value))
            }
            Rule::func => {
                Ok(Self::FunctionCall(pair.first_inner()?.as_str().into()))
            },
            _ => Err(MypsLexerError::wrong_rule("a statement", pair)),
        }
    }
}
