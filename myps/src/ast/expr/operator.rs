use std::{fmt, fmt::Display};

use crate::ast_includes::*;

fn bool_to_num(b: bool) -> f64 {
    if b {
        1.0
    } else {
        0.0
    }
}

// ================================================================================================
// Unary operator
// ================================================================================================
#[rustfmt::skip]
#[derive(Clone, PartialEq, Debug)]
pub enum UnaryOp { Inv, Not }

impl UnaryOp {
    pub fn operate(&self, rhs: f64) -> f64 {
        match self {
            UnaryOp::Inv => -rhs,
            UnaryOp::Not => bool_to_num(!(rhs != 0.0)),
        }
    }
}

impl<'i> AstNode<'i, Rule, MypsParser, MypsParserError> for UnaryOp {
    type Output = Self;

    const RULE: Rule = Rule::b_op;

    fn try_from_pair(pair: Pair<Rule>) -> MypsParserResult<Self> {
        Ok(match pair.as_rule() {
            Rule::inv => Self::Inv,
            Rule::not => Self::Not,
            _ => return Err(MypsParserError::wrong_rule("a unary operator", pair)),
        })
    }
}

impl Display for UnaryOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use UnaryOp::*;

        match self {
            Inv => write!(f, "-"),
            Not => write!(f, "!"),
        }
    }
}

// ================================================================================================
// Binary operator
// ================================================================================================
#[rustfmt::skip]
#[derive(Clone, PartialEq, Debug)]
pub enum BinaryOp {
    // Numerical
    Add, Sub, Mul, Div, Rem,
    // Logical
    And, Or, Xor,
    // Relational
    EQ, GE, GT, LE, LT, NE,
}

impl BinaryOp {
    pub fn operate(&self, lhs: f64, rhs: f64) -> f64 {
        use BinaryOp::*;

        match self {
            // Numerical
            Add => lhs + rhs,
            Sub => lhs - rhs,
            Mul => lhs * rhs,
            Div => lhs / rhs,
            Rem => lhs.rem_euclid(rhs),
            // Logical
            And => bool_to_num((lhs != 0.0) && (rhs != 0.0)),
            Or => bool_to_num((lhs != 0.0) || (rhs != 0.0)),
            Xor => bool_to_num((lhs != 0.0) != (rhs != 0.0)),
            // Relational
            EQ => bool_to_num(lhs == rhs),
            GE => bool_to_num(lhs >= rhs),
            GT => bool_to_num(lhs > rhs),
            LE => bool_to_num(lhs <= rhs),
            LT => bool_to_num(lhs < rhs),
            NE => bool_to_num(lhs != rhs),
        }
    }
}

impl<'i> AstNode<'i, Rule, MypsParser, MypsParserError> for BinaryOp {
    type Output = Self;

    const RULE: Rule = Rule::b_op;

    fn try_from_pair(pair: Pair<Rule>) -> MypsParserResult<Self> {
        Ok(match pair.as_rule() {
            // Numerical
            Rule::add => Self::Add,
            Rule::sub => Self::Sub,
            Rule::mul => Self::Mul,
            Rule::div => Self::Div,
            Rule::rem => Self::Rem,
            // Logical
            Rule::and => Self::And,
            Rule::or => Self::Or,
            Rule::xor => Self::Xor,
            // Relational
            Rule::eq => Self::EQ,
            Rule::ge => Self::GE,
            Rule::gt => Self::GT,
            Rule::le => Self::LE,
            Rule::lt => Self::LT,
            Rule::ne => Self::NE,
            _ => return Err(MypsParserError::wrong_rule("a binary operator", pair)),
        })
    }
}

impl Display for BinaryOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use BinaryOp::*;

        match self {
            // Numerical
            Add => write!(f, "+"),
            Sub => write!(f, "-"),
            Mul => write!(f, "*"),
            Div => write!(f, "/"),
            Rem => write!(f, "%"),
            // Logical
            And => write!(f, " and "),
            Or => write!(f, " or "),
            Xor => write!(f, " xor "),
            // Relational
            EQ => write!(f, "=="),
            GE => write!(f, ">="),
            GT => write!(f, ">"),
            LE => write!(f, "<="),
            LT => write!(f, "<"),
            NE => write!(f, "!="),
        }
    }
}
