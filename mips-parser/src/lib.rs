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

// use pest::iterators::{Pair, Pairs};
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
    AstError(ast::AstError),
}

/// All-in-one module.
pub mod prelude {
    pub use crate::ast::{FirstInner, AstError};
    pub use crate::ast::nodes::{Arg, Dev, Expr, Mem, Program, Val, Func};
    pub use crate::{build_ast_from_path, build_ast_from_str};
    pub use crate::{MipsParser, MipsParserError, Rule};
    pub use pest::{Parser, iterators::Pair};
}

use ast::nodes::Program;
pub use pest::Parser;

pub fn build_ast_from_str(source: &str) -> Result<Program, MipsParserError> {
    let mut pairs =
        MipsParser::parse(Rule::program, &source).map_err(MipsParserError::ParserError)?;
    let program_pair = pairs.next().unwrap();
    let program = Program::from_pair(program_pair).map_err(MipsParserError::AstError)?;
    Ok(program)
}

pub fn build_ast_from_path<P: Into<PathBuf>>(path: P) -> Result<Program, MipsParserError> {
    use std::fs::read_to_string;

    let source = read_to_string(path.into()).map_err(|e| MipsParserError::IOError(e))?;
    build_ast_from_str(&source)
}

