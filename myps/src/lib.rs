#![feature(trait_alias)]
#![feature(box_patterns)]
#![feature(stmt_expr_attributes)]
#![feature(bool_to_option)]
#![allow(unused_imports)]
use std::io::Error as IOError;
use std::num::{ParseFloatError, ParseIntError};

use pest::iterators::Pair;
use pest_derive::Parser;

use util::impl_from_error;
use util::traits::AstError;

#[derive(Parser)]
#[grammar = "myps.pest"]
pub struct MypsParser;

pub mod lexer;
pub mod translator;

// pub mod prelude {
//     pub use crate::lexer::lex::parse_and_lex;
//     pub use crate::lexer::MypsLexerResult;
//     // pub use crate::translator::parse_and_translate;
//     // pub use crate::{Rule, MypsParser};
//     // pub use crate::ast::{
//     //     Program, BinaryOp, Branch, Dev, Expr, Int, LValue, Num, RValue, Statement, UnaryOp,
//     // };
//     // pub use crate::{MypsParser, MypsParserError, MypsParserResult, Rule};
//     pub use pest::iterators::{Pair, Pairs};
//     pub use pest::Parser;
//     pub use util::traits::{AstNode, FirstInner, NextPair, IntoAst};
// }

pub mod superprelude {
    pub use std::{fmt, fmt::Display};

    pub use pest::iterators::{Pair, Pairs};
    pub use pest::Parser;

    pub use util::traits::*;

    pub use crate::*;
    pub use crate::lexer::*;
    pub use crate::lexer::ast::*;
    pub use crate::translator::*;
}
