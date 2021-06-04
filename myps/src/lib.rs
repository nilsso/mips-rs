#![feature(trait_alias)]
#![feature(box_patterns)]
// #![allow(unused_imports)]
use std::io::Error as IOError;
use std::num::{ParseIntError, ParseFloatError};
use std::collections::HashMap;

use pest_derive::Parser;
use pest::iterators::Pair;

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
    Dummy,
}

impl MypsParserError {
    pub fn wrong_rule(expected: &'static str, found: Pair<Rule>) -> Self {
        Self::WrongRule(format!("Expected {} pair, found {:?}", expected, found))
    }

    pub fn wrong_alias(expected: &'static str, found: &Alias) -> Self {
        Self::WrongAlias(format!("Expected {} alias, found {:?}", expected, found))
    }

    pub fn undefined_alias(key: &String) -> Self {
        Self::UndefinedAlias(format!("Alias {} is undefined", key))
    }
}

impl_from_error!(MypsParserError, PegError, AstError, IOError, ParseIntError, ParseFloatError);

pub type MypsParserResult<T> = Result<T, MypsParserError>;

#[derive(Debug)]
pub enum Alias {
    DevLit(usize, usize),
    Int(i64),
    Num(f64),
    Var,
}

pub struct AliasTable(HashMap<String, Alias>);

// TODO:
// - Validate that aliases in expressions are previously defined
// - Replace aliases that are defined to be values
impl AliasTable {
    pub fn lookup(&self, k: &String) -> MypsParserResult<&Alias> {
        self.get(k).ok_or(MypsParserError::undefined_alias(k))
    }

    pub fn lookup_device(&self, k: &String) -> MypsParserResult<(usize, usize)> {
        let a = self.lookup(k)?;
        if let Alias::DevLit(a, b) = a {
            Ok((*a, *b))
        } else {
            Err(MypsParserError::wrong_alias("a device", &a))
        }
    }

    pub fn lookup_int(&self, k: &String) -> MypsParserResult<Option<i64>> {
        let a = self.lookup(k)?;
        match a {
            Alias::Int(n) => Ok(Some(*n)),
            Alias::Var => Ok(None),
            _ => Err(MypsParserError::wrong_alias("an int", &a))
        }
    }

    pub fn lookup_num(&self, k: &String) -> MypsParserResult<Option<f64>> {
        let a = self.lookup(k)?;
        match a {
            Alias::Num(n) => Ok(Some(*n)),
            Alias::Var => Ok(None),
            _ => Err(MypsParserError::wrong_alias("an num", &a))
        }
    }

    fn get(&self, k: &String) -> Option<&Alias> {
        self.0.get(k)
    }
}

pub mod nodes;

pub mod prelude {
    pub use crate::{Alias, AliasTable, MypsParser, MypsParserResult, MypsParserError, Rule};
    pub use crate::nodes::{Num, Int, Dev, Expr, UnaryOp, BinaryOp, LValue, RValue, Statement};
    pub use util::traits::{IntoAst, FirstInner, NextPair, AstNode};
    pub use pest::{
        iterators::{Pair, Pairs},
        Parser,
    };
}

pub(crate) mod ast_includes {
    pub use crate::{Alias, AliasTable, MypsParser, MypsParserError, MypsParserResult, Rule};
    pub use util::traits::{IntoAst, FirstInner, NextPair, AstNode};
    pub use pest::iterators::{Pair, Pairs};
}

