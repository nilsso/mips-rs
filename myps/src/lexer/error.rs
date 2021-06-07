use std::io::Error as IOError;
use std::num::{ParseIntError, ParseFloatError};

use crate::superprelude::*;

type PegError = pest::error::Error<Rule>;

#[derive(Debug)]
pub enum MypsLexerError {
    // External errors
    PegError(PegError),
    AstError(AstError),
    IOError(IOError),
    ParseIntError(ParseIntError),
    ParseFloatError(ParseFloatError),
    // Lexer errors
    WrongRule(String),
    UndefinedAlias(String),
    WrongAlias(String),

    ExpectedIndent(String),
    WrongIndent(String),
    BranchStatement(&'static str),

    Dummy,
}

pub type MypsLexerResult<T> = Result<T, MypsLexerError>;

impl_from_error!(MypsLexerError, PegError, AstError, IOError, ParseIntError, ParseFloatError);

impl MypsLexerError {
    pub fn wrong_rule(expected: &'static str, found: Pair<Rule>) -> Self {
        Self::WrongRule(format!("Expected {} pair, found {:?}", expected, found))
    }

    pub fn undefined_alias(key: &String) -> Self {
        Self::UndefinedAlias(format!("Alias {} is undefined", key))
    }

    pub fn wrong_alias(expected: &'static str, found: &Alias) -> Self {
        Self::WrongAlias(format!("Expected {} alias, found {:?}", expected, found))
    }

    pub fn expected_indent(expected: usize) -> Self {
        Self::ExpectedIndent(format!("Expected indent of {} or more", expected))
    }

    pub fn wrong_indent(expected: usize, found: usize) -> Self {
        Self::WrongIndent(format!("Expected indent of {}, found {}", expected, found))
    }

    pub fn branch_statement() -> Self {
        Self::BranchStatement("Items can't be constructed from branch statements")
    }
}
