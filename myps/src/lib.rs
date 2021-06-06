#![feature(trait_alias)]
#![feature(box_patterns)]
#![feature(stmt_expr_attributes)]
// #![allow(unused_imports)]
use std::io::Error as IOError;
use std::num::{ParseFloatError, ParseIntError};

use pest::iterators::Pair;
use pest_derive::Parser;

use util::impl_from_error;
use util::traits::AstError;

#[derive(Parser)]
#[grammar = "myps.pest"]
pub struct MypsParser;

type PegError = pest::error::Error<Rule>;

#[derive(Debug)]
pub enum MypsParserError {
    PegError(PegError),
    AstError(AstError),
    IOError(IOError),
    ParseIntError(ParseIntError),
    ParseFloatError(ParseFloatError),
    WrongRule(String),
    WrongAlias(String),
    UndefinedAlias(String),
}

impl MypsParserError {
    pub fn wrong_rule(expected: &'static str, found: Pair<Rule>) -> Self {
        Self::WrongRule(format!("Expected {} pair, found {:?}", expected, found))
    }
}

impl_from_error!(
    MypsParserError,
    PegError,
    AstError,
    IOError,
    ParseIntError,
    ParseFloatError
);

pub type MypsParserResult<T> = Result<T, MypsParserError>;

pub mod analyzer;
pub mod ast;
pub mod lexer;
pub mod translator;

pub mod prelude {
    pub use crate::lexer::lex::parse_and_lex;
    pub use crate::lexer::MypsLexerResult;
    // pub use crate::translator::parse_and_translate;
    // pub use crate::{Rule, MypsParser};
    // pub use crate::ast::{
    //     Program, BinaryOp, Branch, Dev, Expr, Int, LValue, Num, RValue, Statement, UnaryOp,
    // };
    // pub use crate::{MypsParser, MypsParserError, MypsParserResult, Rule};
    pub use pest::iterators::{Pair, Pairs};
    pub use pest::Parser;
    pub use util::traits::AstNode;
}

pub(crate) mod ast_includes {
    pub use std::{fmt, fmt::Display};
    pub use crate::ast::DisplayMips;
    pub use crate::{MypsParser, MypsParserError, MypsParserResult, Rule};
    pub use pest::iterators::{Pair, Pairs};
    pub use util::traits::{AstNode, FirstInner, IntoAst, NextPair};
}
