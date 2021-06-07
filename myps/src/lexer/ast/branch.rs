use crate::superprelude::*;

#[derive(Debug)]
pub enum Branch {
    Program,
    Loop,
    If(Expr),
    Elif(Expr),
    Else,
    While(Expr),
    For(String, Int, Int),
    Def(String),
}

impl Branch {
    pub fn def(&self) -> Option<&String> {
        if let Branch::Def(d) = self {
            Some(d)
        } else {
            None
        }
    }

    pub fn expr(&self) -> Option<&Expr> {
        match self {
            Branch::If(e) | Branch::Elif(e) | Branch::While(e) => Some(e),
            _ => None,
        }
    }

    pub fn mut_expr(&mut self) -> Option<&mut Expr> {
        match self {
            Branch::If(e) | Branch::Elif(e) | Branch::While(e) => Some(e),
            _ => None,
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
            Rule::branch_if    => Ok(Branch::If(pair.first_inner()?.try_into_ast()?)),
            Rule::branch_elif  => Ok(Branch::Elif(pair.first_inner()?.try_into_ast()?)),
            Rule::branch_else  => Ok(Branch::Else),
            Rule::branch_while => Ok(Branch::While(pair.first_inner()?.try_into_ast()?)),
            Rule::branch_for   => {
                let mut pairs = pair.into_inner();
                let name = pairs.next_pair()?.as_str().into();
                let s = pairs.next_pair()?.try_into_ast()?;
                let e = pairs.next_pair()?.try_into_ast()?;
                Ok(Branch::For(name, s, e))
            }
            Rule::branch_def   => Ok(Branch::Def(pair.first_inner()?.as_str().into())),
            _ => Err(MypsLexerError::wrong_rule("a branch", pair)),
        }
    }
}
