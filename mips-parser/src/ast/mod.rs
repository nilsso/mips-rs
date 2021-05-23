//! Stationeers MIPS abstract syntax tree (AST) definition.

use pest::iterators::{Pair, Pairs};

use crate::Rule;

pub mod nodes;

#[derive(Debug)]
pub enum AstError {
    Program,
    Expr(String),
    Func(String),
    Arg(String),
    Mem(String),
    Dev(String),
    Val(String),
    ParseInt(std::num::ParseIntError),
    ParseFloat(std::num::ParseFloatError),
    InsufficientPairs,
}

pub type AstResult<T> = Result<T, AstError>;

pub mod all_node_variants {
    pub use crate::ast::nodes::{
        Arg::*,
        Dev::*,
        Func::*,
        Mem::*,
        Val::*,
    };
}

use std::str::FromStr;
use std::num::ParseIntError;
use std::num::ParseFloatError;

// Helper try to convert a pair into a int.
fn pair_to_int<Int>(pair: Pair<Rule>) -> AstResult<Int>
where
    Int: FromStr<Err = ParseIntError>,
{
    pair.as_str().parse().map_err(AstError::ParseInt)
}

// Helper try to convert a pair into a float.
fn pair_to_float<Float>(pair: Pair<Rule>) -> AstResult<Float>
where
    Float: FromStr<Err = ParseFloatError>,
{
    pair.as_str().parse().map_err(AstError::ParseFloat)
}

pub trait FirstInner {
    type Item;

    fn first_inner(self) -> AstResult<Self::Item>;
}

impl<'i> FirstInner for Pair<'i, Rule> {
    type Item = Self;

    fn first_inner(self) -> AstResult<Self> {
        self.into_inner().next().ok_or(AstError::InsufficientPairs)
    }
}

impl<'i> FirstInner for Pairs<'i, Rule> {
    type Item = Pair<'i, Rule>;

    fn first_inner(mut self) -> AstResult<Self::Item> {
        self.next().ok_or(AstError::InsufficientPairs)
    }
}

// TODO: Not sure if I need a custom error type here. Parser might do well enough.
// use thiserror::Error;

// #[derive(Debug)]
// pub enum ASTError {
//     // Program
//     // Expr
//     // Function
//     // Arg
//     // Memory
//     // Device
//     // Value
// }

// #[derive(Clone, PartialEq, Debug, Error)]
// pub enum StateError {
//     #[error("Cannot find value a from register of type {0}")]
//     InvalidRegister(Register),
//     #[error("Invalid alias {0}")]
//     InvalidAlias(String),
//     #[error("Invalid memory index {0}")]
//     InvalidMemoryIndex(usize),
// }

