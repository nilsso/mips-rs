use std::collections::HashSet;

use crate::superprelude::*;

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
    Def(String),
    Function(String),
}

impl Branch {
    pub fn is_if(&self) -> bool {
        match self {
            Self::If(..) | Self::Elif(..) => true,
            _ => false,
        }
    }

    pub fn analyze(&self, aliases: &mut HashSet<String>) -> MypsLexerResult<()> {
        match self {
            Branch::Program
            | Branch::Loop
            | Branch::Else(_)
            | Branch::Def(_)
            | Branch::Function(_) => Ok(()),
            Branch::If(_, expr) => expr.analyze(aliases),
            Branch::Elif(_, expr) => expr.analyze(aliases),
            Branch::While(expr) => expr.analyze(aliases),
            Branch::For(_, s_expr, e_expr, _) => {
                s_expr.analyze(aliases)?;
                e_expr.analyze(aliases)
            }
        }
    }
}

impl<'i> AstNode<'i, Rule, MypsParser, MypsLexerError> for Branch {
    type Output = Self;

    const RULE: Rule = Rule::branch;

    fn try_from_pair(pair: Pair<Rule>) -> MypsLexerResult<Self> {
        #[rustfmt::skip]
        match pair.as_rule() {
            Rule::branch  => pair.only_inner()?.try_into_ast(),
            Rule::b_loop  => Ok(Branch::Loop),
            Rule::b_if    => Ok(Branch::If(0, pair.only_inner()?.try_into_ast()?)),
            Rule::b_elif  => Ok(Branch::Elif(0, pair.only_inner()?.try_into_ast()?)),
            Rule::b_else  => Ok(Branch::Else(0)),
            Rule::b_while => Ok(Branch::While(pair.only_inner()?.try_into_ast()?)),
            Rule::b_for   => {
                let mut pairs = pair.into_inner();
                let i = pairs.next_pair()?.as_str().into();
                let s = pairs.next_pair()?.try_into_ast()?;
                let e = pairs.next_pair()?.try_into_ast()?;
                let step = pairs.next().map(Expr::try_from_pair).transpose()?;
                pairs.done()?;
                Ok(Branch::For(i, s, e, step))
            }
            Rule::b_def   => {
                let name = pair.only_inner()?.as_str().into();
                Ok(Branch::Def(name))
            },
            _ => Err(MypsLexerError::wrong_rule("a branch", pair)),
        }
    }
}
