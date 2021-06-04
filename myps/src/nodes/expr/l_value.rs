use std::{fmt, fmt::Display};

use crate::ast_includes::*;
use crate::nodes::Dev;

#[derive(Clone, PartialEq, Debug)]
pub enum LValue {
    Param(Dev, String),
    Var(String),
}

impl<'i> AstNode<'i, Rule, MypsParser, MypsParserError> for LValue {
    type Output = Self;

    // l_value = _{ l_param | var }
    //     l_param = ${ dev ~ "." ~ param }
    const RULE: Rule = Rule::l_value;

    fn try_from_pair(pair: Pair<Rule>) -> MypsParserResult<Self> {
        match pair.as_rule() {
            Rule::l_param => {
                let mut inner_pairs = pair.into_inner();
                let dev = inner_pairs.next_pair()?.try_into_ast()?;
                let param = inner_pairs.next_pair()?.as_str().into();
                Ok(Self::Param(dev, param))
            }
            Rule::var => Ok(Self::Var(pair.as_str().into())),
            _ => return Err(MypsParserError::wrong_rule("an l-value", pair)),
        }
    }
}

impl Display for LValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LValue::Param(d, p) => write!(f, "{}.{}", d, p),
            LValue::Var(v) => write!(f, "{}", v),
        }
    }
}
