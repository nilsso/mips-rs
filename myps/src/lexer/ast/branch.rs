use crate::superprelude::*;

///
///
/// Tuple variants `If` and `Elif` have an `usize` indexing the if-elif-else chain they are a
/// part of.
#[derive(Clone, Debug)]
pub enum Branch {
    Program,
    Loop,
    If(usize, Expr),
    Elif(usize, Expr),
    Else(usize),
    While(Expr),
    For(String, Expr, Expr, Option<Expr>),
    Def(String, Vec<String>),
}

impl Branch {
    pub fn is_if(&self) -> bool {
        match self {
            Self::If(..) | Self::Elif(..) => true,
            _ => false,
        }
    }
}

impl<'i> AstNode<'i, Rule, MypsParser, MypsLexerError> for Branch {
    type Output = Self;

    const RULE: Rule = Rule::branch;

    fn try_from_pair(pair: Pair<Rule>) -> MypsLexerResult<Self> {
        #[rustfmt::skip]
        match pair.as_rule() {
            Rule::branch       => pair.first_inner()?.try_into_ast(),
            Rule::branch_loop  => Ok(Branch::Loop),
            Rule::branch_if    => Ok(Branch::If(0, pair.first_inner()?.try_into_ast()?)),
            Rule::branch_elif  => Ok(Branch::Elif(0, pair.first_inner()?.try_into_ast()?)),
            Rule::branch_else  => Ok(Branch::Else(0)),
            Rule::branch_while => Ok(Branch::While(pair.first_inner()?.try_into_ast()?)),
            Rule::branch_for   => {
                let mut pairs = pair.into_inner();
                let i = pairs.next_pair()?.as_str().into();
                let s = pairs.next_pair()?.try_into_ast()?;
                let e = pairs.next_pair()?.try_into_ast()?;
                let step = pairs.next().map(Expr::try_from_pair).transpose()?;
                Ok(Branch::For(i, s, e, step))
            }
            Rule::branch_def   => {
                let mut pairs = pair.into_inner();
                let name = pairs.next_pair()?.as_str().into();
                let args = pairs.map(|pair| pair.as_str().into()).collect();
                Ok(Branch::Def(name, args))
            },
            _ => Err(MypsLexerError::wrong_rule("a branch", pair)),
        }
    }
}
