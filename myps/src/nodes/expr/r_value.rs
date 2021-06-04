use std::{fmt, fmt::Display};

use super::Expr;
use crate::ast_includes::*;
use crate::nodes::{Dev, Int};

#[derive(Clone, PartialEq, Debug)]
pub enum Mode {
    Avg,
    Sum,
    Min,
    Max,
}

impl<'i> AstNode<'i, Rule, MypsParser, MypsParserError> for Mode {
    type Output = Self;

    const RULE: Rule = Rule::batch_mode;

    fn try_from_pair(pair: Pair<Rule>) -> MypsParserResult<Self> {
        match pair.as_rule() {
            Rule::batch_avg => Ok(Mode::Avg),
            Rule::batch_sum => Ok(Mode::Sum),
            Rule::batch_min => Ok(Mode::Min),
            Rule::batch_max => Ok(Mode::Max),
            _ => Err(MypsParserError::wrong_rule("a batch mode", pair)),
        }
    }
}

impl Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Mode::*;

        match self {
            Avg => write!(f, "avg"),
            Sum => write!(f, "sum"),
            Min => write!(f, "min"),
            Max => write!(f, "max"),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum RValue {
    Lit(f64),
    NetParam(Int, Mode, String),
    DevParam(Dev, String),
    Var(String),
    Expr(Box<Expr>),
}

impl<'i> AstNode<'i, Rule, MypsParser, MypsParserError> for RValue {
    type Output = Self;

    // r_value = { num_lit | net_param | dev_param | "(" ~ expr ~ ")" | var }
    //     net_param = ${ int ~ "." ~ batch_mode ~ "." ~ param }
    //     dev_param = ${ dev ~ "." ~ param }
    const RULE: Rule = Rule::r_value;

    fn try_from_pair(pair: Pair<Rule>) -> MypsParserResult<Self> {
        match pair.as_rule() {
            Rule::r_value => pair.first_inner()?.try_into_ast(),
            Rule::num_lit => Ok(Self::Lit(pair.as_str().parse()?)),
            Rule::net_param => {
                let mut pairs = pair.into_inner();
                let hash = pairs.next_pair()?.try_into_ast()?;
                let mode = pairs.next_pair()?.try_into_ast()?;
                let param = pairs.next_pair()?.as_str().into();
                Ok(Self::NetParam(hash, mode, param))
            }
            Rule::dev_param => {
                let mut pairs = pair.into_inner();
                let dev = pairs.next_pair()?.try_into_ast()?;
                let param = pairs.next_pair()?.as_str().into();
                Ok(Self::DevParam(dev, param))
            }
            Rule::expr | Rule::u_expr | Rule::b_expr => {
                Ok(Self::Expr(Box::new(pair.try_into_ast()?)))
            }
            Rule::var => Ok(Self::Var(pair.as_str().into())),
            _ => return Err(MypsParserError::wrong_rule("an r-value", pair)),
        }
    }
}

impl Display for RValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use RValue::*;

        match self {
            Lit(n) => write!(f, "{}", n),
            NetParam(hash, mode, param) => write!(f, "{}.{}.{}", hash, mode, param),
            DevParam(dev, param) => write!(f, "{}.{}", dev, param),
            Var(s) => write!(f, "{}", s),
            Expr(e) => write!(f, "({})", e),
        }
    }
}
