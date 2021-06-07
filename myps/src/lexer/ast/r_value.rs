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

#[derive(Clone, PartialEq, Debug)]
pub enum RValue {
    Num(Num),
    NetParam(Int, Mode, String),
    DevParam(Dev, String),
    Expr(Box<Expr>),
}

impl<'i> AstNode<'i, Rule, MypsParser, MypsLexerError> for RValue {
    type Output = Self;

    // r_value = { num_lit | net_param | dev_param | "(" ~ expr ~ ")" | var }
    //     net_param = ${ int ~ "." ~ batch_mode ~ "." ~ param }
    //     dev_param = ${ dev ~ "." ~ param }
    const RULE: Rule = Rule::r_value;

    fn try_from_pair(pair: Pair<Rule>) -> MypsLexerResult<Self> {
        match pair.as_rule() {
            Rule::r_value => pair.first_inner()?.try_into_ast(),
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
            Rule::expr | Rule::u_expr | Rule::b_expr | Rule::t_expr => {
                Ok(Self::Expr(Box::new(pair.try_into_ast()?)))
            }
            Rule::num_lit | Rule::var => Ok(Self::Num(pair.try_into_ast()?)),
            _ => return Err(MypsLexerError::wrong_rule("an r-value", pair)),
        }
    }
}
