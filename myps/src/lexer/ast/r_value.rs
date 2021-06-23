use std::collections::HashSet;

use crate::superprelude::*;

#[derive(Clone, PartialEq, Debug)]
pub enum Mode {
    Avg,
    Sum,
    Min,
    Max,
}

impl<'i> AstNode<'i, Rule, MypsParser, MypsLexerError> for Mode {
    type Output = Self;

    const RULE: Rule = Rule::batch_mode;

    fn try_from_pair(pair: Pair<Rule>) -> MypsLexerResult<Self> {
        match pair.as_rule() {
            Rule::batch_avg => Ok(Mode::Avg),
            Rule::batch_sum => Ok(Mode::Sum),
            Rule::batch_min => Ok(Mode::Min),
            Rule::batch_max => Ok(Mode::Max),
            _ => Err(MypsLexerError::wrong_rule("a batch mode", pair)),
        }
    }
}

impl Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Mode::*;

        match self {
            Avg => write!(f, "0"),
            Sum => write!(f, "1"),
            Min => write!(f, "2"),
            Max => write!(f, "3"),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum RVFunc {
    // Nullary
    Peek,
    Pop,
    Rand,
    // Unary
    Abs,
    Acos,
    Asin,
    Atan,
    Ceil,
    Cos,
    Exp,
    Floor,
    Ln,
    Round,
    Sin,
    Sqrt,
    Tan,
    Trunc,
    // Binary
    Max,
    Min,
}

impl<'i> AstNode<'i, Rule, MypsParser, MypsLexerError> for RVFunc {
    type Output = (Self, Vec<RValue>);

    const RULE: Rule = Rule::rv_func;

    fn try_from_pair(pair: Pair<Rule>) -> MypsLexerResult<Self::Output> {
        let mut pairs = pair.into_inner();
        let name_pair = pairs.next_pair()?;
        let r_values = pairs.map(RValue::try_from_pair).collect::<MypsLexerResult<Vec<RValue>>>()?;
        let n_args = r_values.len();

        let rv_func = match name_pair.as_rule() {
            Rule::rv_func_nullary => {
                if n_args == 0 {
                    let name_pair = name_pair.only_inner()?;
                    #[rustfmt::skip]
                    match name_pair.as_rule() {
                        Rule::f_peek => RVFunc::Peek,
                        Rule::f_pop  => RVFunc::Pop,
                        _ => return Err(MypsLexerError::wrong_rule("r-value nullary function name", name_pair)),
                    }
                } else {
                    return Err(MypsLexerError::wrong_num_args("Nullary", 0, n_args));
                }
            }
            Rule::rv_func_unary => {
                if n_args == 1 {
                    let name_pair = name_pair.only_inner()?;
                    #[rustfmt::skip]
                    match name_pair.as_rule() {
                        Rule::f_abs   => RVFunc::Abs,
                        Rule::f_acos  => RVFunc::Acos,
                        Rule::f_asin  => RVFunc::Asin,
                        Rule::f_atan  => RVFunc::Atan,
                        Rule::f_ceil  => RVFunc::Ceil,
                        Rule::f_cos   => RVFunc::Cos,
                        Rule::f_exp   => RVFunc::Exp,
                        Rule::f_floor => RVFunc::Floor,
                        Rule::f_ln    => RVFunc::Ln,
                        Rule::f_rand  => RVFunc::Rand,
                        Rule::f_round => RVFunc::Round,
                        Rule::f_sin   => RVFunc::Sin,
                        Rule::f_sqrt  => RVFunc::Sqrt,
                        Rule::f_tan   => RVFunc::Tan,
                        Rule::f_trunc => RVFunc::Trunc,
                        _ => return Err(MypsLexerError::wrong_rule("r-value unary function name", name_pair)),
                    }
                } else {
                    return Err(MypsLexerError::wrong_num_args("Unary", 1, n_args));
                }
            }
            Rule::rv_func_binary => {
                if n_args == 2 {
                    let name_pair = name_pair.only_inner()?;
                    #[rustfmt::skip]
                    match name_pair.as_rule() {
                        Rule::f_max => RVFunc::Max,
                        Rule::f_min => RVFunc::Min,
                        _ => return Err(MypsLexerError::wrong_rule("r-value binary function name", name_pair)),
                    }
                } else {
                    return Err(MypsLexerError::wrong_num_args("Binary", 2, n_args));
                }
            }
            _ => {
                return Err(MypsLexerError::wrong_rule(
                    "r-value function name",
                    name_pair,
                ))
            }
        };

        Ok((rv_func, r_values))
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum RValue {
    Num(Num),
    Dev(Dev),
    DevParam(Dev, String),
    NetParam(Dev, Mode, String),
    DevSlot(Dev, Int, String),
    Expr(Box<Expr>),
    Func(RVFunc, Vec<RValue>),
    Var(String),
}

impl RValue {
    pub fn analyze(&self, aliases: &mut HashSet<String>) -> MypsLexerResult<()> {
        match self {
            RValue::Num(num) => num.analyze(aliases),
            RValue::Dev(dev)
            | RValue::DevParam(dev, ..)
            | RValue::NetParam(dev, ..)
            | RValue::DevSlot(dev, ..) => dev.analyze(aliases),
            RValue::Expr(box expr) => expr.analyze(aliases),
            RValue::Func(func, r_values) => {
                for r_value in r_values.iter() {
                    r_value.analyze(aliases)?;
                }
                Ok(())
            }
            RValue::Var(k) => aliases
                .contains(k)
                .then_some(())
                .ok_or(MypsLexerError::undefined_alias(k)),
        }
    }
}

impl<'i> AstNode<'i, Rule, MypsParser, MypsLexerError> for RValue {
    type Output = Self;

    const RULE: Rule = Rule::rv;

    fn try_from_pair(pair: Pair<Rule>) -> MypsLexerResult<Self> {
        match pair.as_rule() {
            Rule::rv => pair.only_inner()?.try_into_ast(),
            Rule::dev_lit => Ok(Self::Dev(pair.try_into_ast()?)),
            Rule::rv_net_param => {
                let mut pairs = pair.into_inner();
                let hash = pairs.next_pair()?.try_into_ast()?;
                let mode = pairs.next_pair()?.try_into_ast()?;
                let param = pairs.final_pair()?.as_str().into();
                Ok(Self::NetParam(hash, mode, param))
            }
            Rule::rv_dev_param => {
                let mut pairs = pair.into_inner();
                let dev = pairs.next_pair()?.try_into_ast()?;
                let param = pairs.final_pair()?.as_str().into();
                Ok(Self::DevParam(dev, param))
            }
            Rule::rv_dev_slot => {
                let mut pairs = pair.into_inner();
                let dev = pairs.next_pair()?.try_into_ast()?;
                let slot = pairs.next_pair()?.try_into_ast()?;
                let param = pairs.final_pair()?.as_str().into();
                Ok(Self::DevSlot(dev, slot, param))
            }
            Rule::expr_unary | Rule::expr_binary | Rule::expr_ternary => {
                Ok(Self::Expr(Box::new(pair.try_into_ast()?)))
            }
            Rule::rv_func => {
                let (rv_func, r_values) = RVFunc::try_from_pair(pair)?;
                Ok(Self::Func(rv_func, r_values))
            }
            Rule::int | Rule::int_lit => Ok(Self::Num(pair.try_into_ast()?)),
            Rule::num | Rule::num_lit => Ok(Self::Num(pair.try_into_ast()?)),
            Rule::var => Ok(Self::Var(pair.as_str().into())),
            _ => return Err(MypsLexerError::wrong_rule("an r-value", pair)),
        }
    }
}
