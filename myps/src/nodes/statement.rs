use std::{fmt, fmt::Display};

use crate::ast_includes::*;
use crate::nodes::{Dev, LValue, RValue};

#[derive(Clone, PartialEq, Debug)]
pub enum Statement {
    AssignAlias(String, Dev),
    AssignValue(LValue, RValue),
}

impl<'i> AstNode<'i, Rule, MypsParser, MypsParserError> for Statement {
    type Output = Self;

    const RULE: Rule = Rule::stmt;

    fn try_from_pair(pair: Pair<Rule>) -> MypsParserResult<Self> {
        match pair.as_rule() {
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
            _ => Err(MypsParserError::wrong_rule("a statement", pair)),
        }
    }
}

impl Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Statement::AssignAlias(a, b) => write!(f, "{} = {}", a, b),
            Statement::AssignValue(a, b) => {
                let b = match b {
                    RValue::Expr(e) => format!("{}", e),
                    b => format!("{}", b),
                };
                write!(f, "{} = {}", a, b)
            }
        }
    }
}
