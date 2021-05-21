//! Parser for [Stationeers][stationeers] [MIPS][mips] code using [Pest][pest].
//!
//! The [MipsParser] can be used standalone to validate Stationeers MIPS source code,
//! but the [Pairs][pairs] returned can be used to construct an [abstract syntax tree (AST)][ast]
//! as specified in the [`ast`] sub-module.
//!
//! [stationeers]: https://store.steampowered.com/app/544550/Stationeers/
//! [mips]: https://stationeers-wiki.com/MIPS
//! [pest]: https://pest.rs/
//! [pairs]: https://docs.rs/pest/2.1.3/pest/iterators/struct.Pairs.html
//! [ast]: https://en.wikipedia.org/wiki/Abstract_syntax_tree
#![feature(bool_to_option)]
#![feature(stmt_expr_attributes)]

use std::path::PathBuf;

use pest::iterators::{Pair, Pairs};
use pest_derive::Parser;

/// Stationeers MIPS language parser.
#[derive(Parser)]
#[grammar = "mips.pest"]
pub struct MipsParser;

// Abstract syntax tree (AST) module (all the nodes)
pub mod ast;

#[derive(Debug)]
pub enum MipsParserError {
    IOError(std::io::Error),
    ParserError(pest::error::Error<Rule>),
}

/// Everything in one use statement.
pub mod prelude {
    pub use crate::ast::nodes::{Arg, Device, Expr, Function, Memory, Program, Value};
    pub use crate::{build_ast_from_path, build_ast_from_str};
    pub use crate::{InnerUnchecked, MipsParser, MipsParserError, Rule};
    pub use pest::Parser;
}

use ast::nodes::Program;
pub use pest::Parser;

pub fn build_ast_from_str(source: &str) -> Result<Program, MipsParserError> {
    let mut pairs =
        MipsParser::parse(Rule::program, &source).map_err(|e| MipsParserError::ParserError(e))?;
    let program_pair = pairs.next().unwrap();
    let program = Program::new(program_pair);
    Ok(program)
}

pub fn build_ast_from_path<P: Into<PathBuf>>(path: P) -> Result<Program, MipsParserError> {
    use std::fs::read_to_string;

    let source = read_to_string(path.into()).map_err(|e| MipsParserError::IOError(e))?;
    build_ast_from_str(&source)
}

pub trait InnerUnchecked {
    type Output;

    fn inner(self) -> Self::Output;
}

impl<'i> InnerUnchecked for Pair<'i, Rule> {
    type Output = Self;

    fn inner(self) -> Self {
        self.into_inner().next().unwrap()
    }
}

impl<'i> InnerUnchecked for Pairs<'i, Rule> {
    type Output = Pair<'i, Rule>;

    fn inner(mut self) -> Pair<'i, Rule> {
        self.next().unwrap()
    }
}
