use crate::superprelude::*;

/// Device node.
#[derive(Clone, PartialEq, Debug)]
pub enum Dev {
    Lit(Box<RValue>),
    Net(Box<RValue>),
    Var(String),
    DB,
}

impl<'i> AstNode<'i, Rule, MypsParser, MypsLexerError> for Dev {
    type Output = Self;

    // dev = _{ dev_batch | dev_lit | var }
    //     dev_batch = ${ int ~ ".all" }
    //     dev_lit   = ${ "d" ~ "r"* ~ !".all" ~ int_lit }
    const RULE: Rule = Rule::dev;

    fn try_from_pair(pair: Pair<Rule>) -> MypsLexerResult<Self> {
        match pair.as_rule() {
            Rule::dev_lit => pair.first_inner()?.try_into_ast(),
            Rule::dev_self => Ok(Dev::DB),
            Rule::dev_base => {
                let rvalue = pair.first_inner()?.try_into_ast()?;
                Ok(Dev::Lit(Box::new(rvalue)))
            }
            Rule::dev_net => {
                let rvalue = pair.first_inner()?.try_into_ast()?;
                Ok(Dev::Net(Box::new(rvalue)))
            }
            Rule::var => Ok(Self::Var(pair.as_str().into())),
            _ => return Err(MypsLexerError::wrong_rule("a device", pair)),
        }
    }
}

impl Display for Dev {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Lit(box RValue::Num(i)) => {
                write!(f, "d{}", i)
            },
            Self::Var(var) => {
                write!(f, "{}", var)
            },
            Self::DB => write!(f, "db"),
            _ => unreachable!("{:?}", self),
        }
    }
}
