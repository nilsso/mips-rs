use std::collections::HashSet;

use crate::superprelude::*;

/// Device node.
#[derive(Clone, PartialEq, Debug)]
pub enum Dev {
    Lit(Box<RValue>),
    Net(Box<RValue>),
    Var(String),
    DB,
}

impl Dev {
    pub fn analyze(&self, aliases: &mut HashSet<String>) -> MypsLexerResult<()> {
        match self {
            Dev::Var(k) => {
                aliases
                    .contains(k)
                    .then_some(())
                    .ok_or(MypsLexerError::undefined_alias(k))
            },
            _ => Ok(()),
        }
    }
}

impl<'i> AstNode<'i, Rule, MypsParser, MypsLexerError> for Dev {
    type Output = Self;

    // dev = _{ dev_batch | dev_lit | var }
    //     dev_batch = ${ int ~ ".all" }
    //     dev_lit   = ${ "d" ~ "r"* ~ !".all" ~ int_lit }
    const RULE: Rule = Rule::dev;

    fn try_from_pair(pair: Pair<Rule>) -> MypsLexerResult<Self> {
        match pair.as_rule() {
            Rule::dev_lit => pair.only_inner()?.try_into_ast(),
            Rule::dev_self => Ok(Dev::DB),
            Rule::dev_base => {
                let rv = pair.only_inner()?.try_into_ast()?;
                Ok(Dev::Lit(Box::new(rv)))
            }
            Rule::dev_net => {
                let rv = pair.only_inner()?.try_into_ast()?;
                Ok(Dev::Net(Box::new(rv)))
            }
            Rule::int => {
                let rv = pair.try_into_ast()?;
                Ok(Self::Net(Box::new(rv)))
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
