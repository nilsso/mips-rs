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
}

impl<'i> AstNode<'i, Rule, MypsParser, MypsLexerError> for LValue {
    type Output = Self;

    // l_value = _{ l_param | var }
    //     l_param = ${ dev ~ "." ~ param }
    const RULE: Rule = Rule::l_value;

    fn try_from_pair(pair: Pair<Rule>) -> MypsLexerResult<Self> {
        match pair.as_rule() {
            // Rule::l_value => pair.first_inner()?.try_into_ast(),
            Rule::l_value => pair.first_inner().unwrap().try_into_ast(),
            Rule::l_param => {
                let mut inner_pairs = pair.into_inner();
                let dev = inner_pairs.next_pair()?.try_into_ast()?;
                let param = inner_pairs.next_pair()?.as_str().into();
                Ok(Self::Param(dev, param))
            }
            Rule::var_asn => {
                let mut inner_pairs = pair.into_inner();
                // let fix_or_name = inner_pairs.next_pair()?;
                let fix_or_name = inner_pairs.next_pair().unwrap();
                let (name, fix) = match fix_or_name.as_rule() {
                    Rule::fix => {
                        // let name = inner_pairs.next_pair()?;
                        let name = inner_pairs.next_pair().unwrap();
                        (name.as_str().into(), true)
                    },
                    Rule::var => {
                        (fix_or_name.as_str().into(), false)
                    }
                    _ => unreachable!("{:?}", fix_or_name),
                };
                Ok(Self::Var(name, fix))
            },
            _ => return Err(MypsLexerError::wrong_rule("an l-value", pair)),
        }
    }
}
