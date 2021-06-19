use std::collections::HashSet;

use crate::superprelude::*;

#[derive(Clone, PartialEq, Debug)]
pub enum LValue {
    Param(Dev, String),
    Var(String, bool),
}

impl LValue {
    pub fn as_var(&self) -> Option<&String> {
        match self {
            Self::Var(k, _) => Some(k),
            _ => None,
        }
    }

    pub fn as_rvalue(&self) -> RValue {
        println!("l_value::as_rvalue {:?}", self);
        match self {
            Self::Param(dev, param) => {
                if matches!(dev, Dev::Net(..)) {
                    unreachable!("{:?}", dev);
                } else {
                    RValue::DevParam(dev.to_owned(), param.to_owned())
                }
            },
            Self::Var(k, _) => RValue::Num(Num::Var(k.to_owned())),
        }
    }

    pub fn analyze(&self, aliases: &mut HashSet<String>) -> MypsLexerResult<()> {
        match self {
            Self::Param(dev, _) => {
                dev.analyze(aliases)
            },
            Self::Var(k, _) => {
                aliases.insert(k.to_owned());
                Ok(())
            },
        }
    }
}

impl<'i> AstNode<'i, Rule, MypsParser, MypsLexerError> for LValue {
    type Output = Self;

    const RULE: Rule = Rule::lv;

    fn try_from_pair(pair: Pair<Rule>) -> MypsLexerResult<Self> {
        match pair.as_rule() {
            Rule::lv => pair.only_inner()?.try_into_ast(),
            Rule::lv_param => {
                let mut inner_pairs = pair.into_inner();
                let dev = inner_pairs.next_pair()?.try_into_ast()?;
                let param = inner_pairs.final_pair()?.as_str().into();
                Ok(Self::Param(dev, param))
            }
            Rule::var_fix => {
                let name = pair.only_inner()?.as_str().into();

                Ok(Self::Var(name, true))
            },
            Rule::var => {
                let name = pair.as_str().into();
                Ok(Self::Var(name, false))
            },
            _ => return Err(MypsLexerError::wrong_rule("an l-value", pair)),
        }
    }
}
