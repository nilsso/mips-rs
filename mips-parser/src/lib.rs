//! Parser for [Stationeers][stationeers] [MIPS][mips] code using [Pest][pest].
//!
//! The [MipsParser] can be used standalone to validate Stationeers MIPS source code,
//! but the [Pairs][pairs] returned can be used to construct an [abstract syntax tree (AST)][ast]
//! as specified in the [`ast`] sub-module.
//!
//! For an implementation using the AST, see the [simulator component]
//! of this project.
//!
//! [stationeers]: https://store.steampowered.com/app/544550/Stationeers/
//! [mips]: https://stationeers-wiki.com/MIPS
//! [pest]: https://pest.rs/
//! [pairs]: https://docs.rs/pest/2.1.3/pest/iterators/struct.Pairs.html
//! [ast]: https://en.wikipedia.org/wiki/Abstract_syntax_tree
//! [simulator component]: ../mips_simulator/index.html
#![feature(bool_to_option)]
#![feature(associated_type_defaults)]

use pest_derive::Parser;

/// Stationeers MIPS language parser.
#[derive(Parser)]
#[grammar = "mips.pest"]
pub struct MipsParser;

pub mod ast;

/// MIPS parser error type.
#[derive(Debug)]
pub enum MipsParserError {
    IOError(std::io::Error),
    ParserError(pest::error::Error<Rule>),
    AstError(ast::AstError),
}

/// All-in-one module.
pub mod prelude {
    pub use crate::ast::nodes::{Arg, Dev, Expr, Func, Mem, Program, Val};
    pub use crate::ast::{Node, AstError, FirstInner};
    pub use crate::{MipsParser, MipsParserError, Rule};
    pub use pest::{iterators::Pair, Parser};
}
