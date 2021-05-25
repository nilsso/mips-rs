//! Stationeers MIPS abstract syntax tree (AST) definition.
use std::convert::AsRef;
use std::fs::read_to_string;
use std::num::{ParseFloatError, ParseIntError};
use std::path::PathBuf;
use std::str::FromStr;

use pest::{
    iterators::{Pair, Pairs},
    Parser,
};

use crate::{MipsParser, MipsParserError, Rule};

/// MIPS AST node error type.
#[derive(Debug)]
pub enum AstError {
    /// Failure to construct a program node.
    Program,
    /// Failure to construct an expression node.
    Expr(String),
    /// Failture to construct an function node.
    Func(String),
    /// Failure to construct an argument node.
    Arg(String),
    /// Failure to construct a memory node.
    Mem(String),
    /// Failure to construct a device node.
    Dev(String),
    /// Failure to construct a value node.
    Val(String),
    /// Failure in parsing an integer.
    ParseInt(std::num::ParseIntError),
    /// Failure in parsing an float.
    ParseFloat(std::num::ParseFloatError),
    /// Not enough pairs error (raised by [`FirstInner::first_inner`]).
    InsufficientPairs,
    /// Wrong argument kind.
    WrongArg(String),
}

/// Shortcut type for AST error results.
pub type AstResult<T> = Result<T, AstError>;

/// Trait for constructing AST nodes from pairs, strings and files.
pub trait Node
where
    Self: Sized,
{
    /// Output type for node `try_from` constructors
    /// (generally `Self` or `Option<Self>`).
    type Output = Self;

    /// Rule for [`Self::try_from_str`](#method.try_from_str).
    const RULE: Rule;

    /// Try to construct this node kind from a PEG pair
    /// (expected to be of at least the [`Self::RULE`](#associatedconstant.RULE) rule).
    fn try_from_pair(pair: Pair<Rule>) -> AstResult<Self::Output>;

    /// Try to construct this node kind from a string
    /// (expected to match at least the [`Self::RULE`](#associatedconstant.RULE) rule).
    fn try_from_str<S: AsRef<str>>(source: &S) -> Result<Self::Output, MipsParserError> {
        let peg =
            MipsParser::parse(Self::RULE, source.as_ref()).map_err(MipsParserError::ParserError)?;
        peg.first_inner()
            .and_then(Self::try_from_pair)
            .map_err(MipsParserError::AstError)
    }

    /// Try to construct this node kind from a file.
    fn try_from_file<P: Into<PathBuf>>(path: P) -> Result<Self::Output, MipsParserError> {
        let input = read_to_string(path.into()).map_err(|e| MipsParserError::IOError(e))?;
        Self::try_from_str(&input)
    }
}

// All nodes
pub mod nodes;

/// Helper module for exposing all node kind variants.
pub mod all_node_variants {
    pub use crate::ast::nodes::{Arg::*, Dev::*, Func::*, Mem::*, Val::*};
}

/// Helper try to convert a pair into a int.
fn pair_to_int<Int>(pair: Pair<Rule>) -> AstResult<Int>
where
    Int: FromStr<Err = ParseIntError>,
{
    pair.as_str().parse().map_err(AstError::ParseInt)
}

/// Helper try to convert a pair into a float.
fn pair_to_float<Float>(pair: Pair<Rule>) -> AstResult<Float>
where
    Float: FromStr<Err = ParseFloatError>,
{
    pair.as_str().parse().map_err(AstError::ParseFloat)
}

/// Helper trait to get first inner pair from a Pest Pair or Pairs.
pub trait FirstInner {
    type Item;

    /// Try to get first inner pair.
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
