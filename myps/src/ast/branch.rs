use std::{fmt, fmt::Display};

use crate::ast::Expr;
use crate::ast_includes::*;

#[derive(Debug)]
pub enum Branch {
    Program,
    Loop,
    Def(String),
    If(Expr),
    Elif(Expr),
    Else,
    While(Expr),
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

impl<'i> AstNode<'i, Rule, MypsParser, MypsParserError> for Branch {
    type Output = Self;

    const RULE: Rule = Rule::branch;

    fn try_from_pair(pair: Pair<Rule>) -> MypsParserResult<Self> {
        #[rustfmt::skip]
        match pair.as_rule() {
            Rule::branch       => pair.first_inner()?.try_into_ast(),
            Rule::branch_loop  => Ok(Branch::Loop),
            Rule::branch_def   => Ok(Branch::Def(pair.first_inner()?.as_str().into())),
            Rule::branch_if    => Ok(Branch::If(pair.first_inner()?.try_into_ast()?)),
            Rule::branch_elif  => Ok(Branch::Elif(pair.first_inner()?.try_into_ast()?)),
            Rule::branch_else  => Ok(Branch::Else),
            Rule::branch_while => Ok(Branch::While(pair.first_inner()?.try_into_ast()?)),
            _ => Err(MypsParserError::wrong_rule("a branch", pair)),
        }
    }
}

impl Display for Branch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Branch::*;

        match self {
            Program => write!(f, "(program)"),
            Loop => write!(f, "loop:"),
            Def(s) => write!(f, "def {}:", s),
            If(e) => write!(f, "if {}:", e),
            Elif(e) => write!(f, "elif {}:", e),
            Else => write!(f, "else:"),
            While(e) => write!(f, "while {}:", e),
        }
    }
}
