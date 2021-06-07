// #![allow(unused_imports)]
use lazy_static::lazy_static;
use pest::prec_climber::{Assoc, Operator, PrecClimber};

use crate::superprelude::*;

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
    Ternary {
        cond: Box<Expr>,
        if_t: Box<Expr>,
        if_f: Box<Expr>,
    },
}

impl Expr {
    /// New rvalue.
    pub fn rvalue(x: RValue) -> Expr {
        Expr::RValue(x)
    }

    /// New rvalue literal.
    pub fn lit(x: f64) -> Expr {
        Expr::RValue(RValue::Num(Num::Lit(x)))
    }

    /// New unary expression (with literal value simplification).
    pub fn unary(op: UnaryOp, rhs: Expr) -> Self {
        let rhs = Box::new(rhs);
        Self::Unary { op, rhs }
    }

    /// New binary expression (with literal value simplification).
    pub fn binary(op: BinaryOp, lhs: Expr, rhs: Expr) -> Self {
        let lhs = Box::new(lhs);
        let rhs = Box::new(rhs);
        Self::Binary { op, lhs, rhs }
    }

    pub fn ternary(cond: Expr, if_t: Expr, if_f: Expr) -> Self {
        let cond = Box::new(cond);
        let if_t = Box::new(if_t);
        let if_f = Box::new(if_f);
        Self::Ternary { cond, if_t, if_f }
    }

    /// simplify an expression algebraicly or numerically via pattern matching.
    ///
    /// TODO: Remove from here, implement in the lexer
    pub fn simplify(self) -> Expr {
        match self {
            // Calculate unary expresion of a literal
            Expr::Unary {
                op,
                rhs: box Expr::RValue(RValue::Num(Num::Lit(n))),
            } => Self::lit(op.operate(n)),

            // Calculate binary expressions of literals
            Expr::Binary {
                op,
                lhs: box Expr::RValue(RValue::Num(Num::Lit(l))),
                rhs: box Expr::RValue(RValue::Num(Num::Lit(r))),
            } => Self::lit(op.operate(l, r)),

            // Calculate ternary expression of literal
            Expr::Ternary {
                cond: box Expr::RValue(RValue::Num(Num::Lit(c))),
                if_t: box Expr::RValue(RValue::Num(Num::Lit(t))),
                if_f: box Expr::RValue(RValue::Num(Num::Lit(f))),
            } => Self::lit(if c != 0.0 { t } else { f }),

            // Unpack r-value expression
            Expr::RValue(RValue::Expr(box e)) => e,

            // Otherwise...
            _ => self,
        }
    }
}

impl<'i> AstNode<'i, Rule, MypsParser, MypsLexerError> for Expr {
    type Output = Self;

    // expr = _{ u_expr | b_expr | r_value }
    //     /* A unary expression */
    //     u_expr = { u_op ~ r_value }
    //     /* A binary expression */
    //     b_expr = { r_value ~ (b_op ~ r_value)+ }
    const RULE: Rule = Rule::expr;

    fn try_from_pair(pair: Pair<Rule>) -> MypsLexerResult<Self> {
        match pair.as_rule() {
            Rule::u_expr => {
                let mut pairs = pair.into_inner();
                let op = pairs.next_pair()?.try_into_ast::<UnaryOp>()?;
                let rhs = pairs.next_pair()?.try_into_ast()?;
                Ok(Expr::unary(op, rhs))
            }
            Rule::b_expr => Ok(expr_climb(pair.into_inner())),
            Rule::t_expr => {
                let mut pairs = pair.into_inner();
                let cond = pairs.next_pair()?.try_into_ast()?;
                let if_t = pairs.next_pair()?.try_into_ast()?;
                let if_f = pairs.next_pair()?.try_into_ast()?;
                Ok(Expr::ternary(cond, if_t, if_f))
            }
            Rule::r_value => Ok(Self::rvalue(pair.try_into_ast()?)),
            _ => return Err(MypsLexerError::wrong_rule("an r-value expression", pair)),
        }
    }
}

// Operator precedence climber
lazy_static! {
    static ref CLIMBER: PrecClimber<Rule> = PrecClimber::new(vec![
        // Logical
        Operator::new(Rule::or, Assoc::Left),
        Operator::new(Rule::xor, Assoc::Left),
        Operator::new(Rule::and, Assoc::Left),
        // Relational
        Operator::new(Rule::eq, Assoc::Left),
        Operator::new(Rule::ge, Assoc::Left),
        Operator::new(Rule::gt, Assoc::Left),
        Operator::new(Rule::le, Assoc::Left),
        Operator::new(Rule::lt, Assoc::Left),
        Operator::new(Rule::ne, Assoc::Left),
        // Numerical
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
