// #![allow(unused_imports)]
use std::{fmt, fmt::Display};

use lazy_static::lazy_static;
use pest::prec_climber::{Assoc, Operator, PrecClimber};

use crate::ast_includes::*;

pub mod operators;
pub mod l_value;
pub mod r_value;

use operators::{BinaryOp, UnaryOp};
use r_value::RValue;

#[derive(Clone, PartialEq, Debug)]
pub enum Expr {
    RValue(RValue),
    Unary {
        op: UnaryOp,
        rhs: Box<Expr>,
    },
    Binary {
        op: BinaryOp,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
}

impl Expr {
    /// New rvalue.
    pub fn rvalue(x: RValue) -> Expr {
        Expr::RValue(x).simplify()
    }

    /// New rvalue literal.
    pub fn lit(x: f64) -> Expr {
        Expr::RValue(RValue::Lit(x))
    }

    /// New unary expression (with literal value simplification).
    pub fn unary(op: UnaryOp, rhs: Expr) -> Self {
        let rhs = Box::new(rhs);
        Self::Unary { op, rhs }.simplify()
    }

    /// New binary expression (with literal value simplification).
    pub fn binary(op: BinaryOp, lhs: Expr, rhs: Expr) -> Self {
        let lhs = Box::new(lhs);
        let rhs = Box::new(rhs);
        Self::Binary { op, lhs, rhs }.simplify()
    }

    /// simplify an expression algebraicly or numerically via pattern matching.
    pub fn simplify(self) -> Expr {
        match self {
            // Calculate unary expresion of a literal
            Expr::Unary {
                op,
                rhs: box Expr::RValue(RValue::Lit(n)),
            } => Self::lit(op.operate(n)),

            // Calculate binary expressions of literals
            Expr::Binary {
                op,
                lhs: box Expr::RValue(RValue::Lit(l)),
                rhs: box Expr::RValue(RValue::Lit(r)),
            } => Self::lit(op.operate(l, r)),

            // Unpack r-value expression
            Expr::RValue(RValue::Expr(box e)) => e,

            // Otherwise...
            _ => self,
        }
    }
}

impl<'i> AstNode<'i, Rule, MypsParser, MypsParserError> for Expr {
    type Output = Self;

    // expr = _{ u_expr | b_expr | r_value }
    //     /* A unary expression */
    //     u_expr = { u_op ~ r_value }
    //     /* A binary expression */
    //     b_expr = { r_value ~ (b_op ~ r_value)+ }
    const RULE: Rule = Rule::expr;

    fn try_from_pair(pair: Pair<Rule>) -> MypsParserResult<Self> {
        match pair.as_rule() {
            Rule::u_expr => {
                let mut pairs = pair.into_inner();
                let op = pairs.next_pair()?.try_into_ast::<UnaryOp>()?;
                let rhs = pairs.next_pair()?.try_into_ast()?;
                Ok(Expr::unary(op, rhs))
            }
            // Binary expression using operator precedence climber
            Rule::b_expr => Ok(expr_climb(pair.into_inner())),
            Rule::r_value => Ok(Self::rvalue(pair.try_into_ast()?)),
            _ => return Err(MypsParserError::wrong_rule("an r-value expression", pair)),
        }
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Expr::*;

        match self {
            RValue(rv) => write!(f, "{}", rv),
            Unary { op, rhs } => write!(f, "({}{})", op, rhs),
            Binary { op, lhs, rhs } => write!(f, "({}{}{})", lhs, op, rhs),
        }
    }
}

// Operator precedence climber
lazy_static! {
    static ref CLIMBER: PrecClimber<Rule> = PrecClimber::new(vec![
        Operator::new(Rule::or, Assoc::Left),
        Operator::new(Rule::xor, Assoc::Left),
        Operator::new(Rule::and, Assoc::Left),
        Operator::new(Rule::add, Assoc::Left),
        Operator::new(Rule::sub, Assoc::Left),
        Operator::new(Rule::rem, Assoc::Left),
        Operator::new(Rule::div, Assoc::Left),
        Operator::new(Rule::mul, Assoc::Left),
    ]);
}

// Operator precedence climber infix helper
fn infix(lhs: Expr, op_pair: Pair<Rule>, rhs: Expr) -> Expr {
    let op = op_pair.into_ast::<BinaryOp>();
    Expr::binary(op, lhs, rhs)
}

// Operator precedence climber helper (for binary expressions)
pub fn expr_climb(pairs: Pairs<Rule>) -> Expr {
    CLIMBER.climb(pairs, Expr::from_pair, infix)
}
