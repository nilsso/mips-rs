use std::collections::HashSet;

use crate::superprelude::*;

#[derive(Clone, PartialEq, Debug)]
pub enum Num {
    Lit(f64),
    Var(String),
}

impl Num {
    pub fn analyze(&self, aliases: &mut HashSet<String>) -> MypsLexerResult<()> {
        match self {
            Num::Var(k) => {
                aliases
                    .contains(k)
                    .then_some(())
                    .ok_or(MypsLexerError::undefined_alias(k))
            },
            _ => Ok(()),
        }
    }
}

impl<'i> AstNode<'i, Rule, MypsParser, MypsLexerError> for Num {
    type Output = Self;

    // num = { num_lit | var }
    const RULE: Rule = Rule::num;

    fn try_from_pair(pair: Pair<Rule>) -> MypsLexerResult<Self> {
        match pair.as_rule() {
            Rule::num => pair.only_inner()?.try_into_ast(),
            Rule::int => pair.only_inner()?.try_into_ast(),
            Rule::num_lit => Ok(Self::Lit(pair.as_str().parse()?)),
            Rule::int_lit => Ok(Self::Lit(pair.as_str().parse()?)),
            Rule::var => Ok(Self::Var(pair.as_str().into())),
            _ => Err(MypsLexerError::wrong_rule("a number or variable", pair)),
        }
    }
}

impl Display for Num {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Lit(n) => write!(f, "{}", n),
            Self::Var(v) => write!(f, "{}", v),
        }
    }
}
