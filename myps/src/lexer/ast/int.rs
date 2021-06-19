use crate::superprelude::*;

#[derive(Clone, PartialEq, Debug)]
pub enum Int {
    Lit(i64),
    Var(String),
}

impl<'i> AstNode<'i, Rule, MypsParser, MypsLexerError> for Int {
    type Output = Self;

    // int = { int_lit | var }
    const RULE: Rule = Rule::int;

    fn try_from_pair(pair: Pair<Rule>) -> MypsLexerResult<Self> {
        match pair.as_rule() {
            Rule::int => pair.only_inner()?.try_into_ast(),
            Rule::int_lit => Ok(Self::Lit(pair.as_str().parse()?)),
            Rule::var => Ok(Self::Var(pair.as_str().into())),
            _ => Err(MypsLexerError::wrong_rule("an integer or variable", pair)),
        }
    }
}

impl Display for Int {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Lit(n) => write!(f, "{}", n),
            Self::Var(var) => write!(f, "{}", var),
        }
    }
}
