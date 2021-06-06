use std::io::Error as IOError;

use crate::lexer::Alias;
use crate::{MypsParserError, Rule};
use util::{impl_from_error, traits::AstError};

type PegError = pest::error::Error<Rule>;

#[derive(Debug)]
pub enum MypsLexerError {
    PegError(PegError),
    AstError(AstError),
    IOError(IOError),
    MypsParserError(MypsParserError),

    ExpectedIndent(String),
    WrongIndent(String),
    BranchStatement(&'static str),
    UndefinedAlias(String),
    WrongAlias(String),

    Dummy,
}

impl_from_error!(MypsLexerError, PegError, AstError, IOError, MypsParserError,);

impl MypsLexerError {
    // pub fn expected_indent(expected: usize, found: usize) -> Self {
    // Self::
    // }

    pub fn branch_statement() -> Self {
        Self::BranchStatement("Items can't be constructed from branch statements")
    }

    pub fn undefined_alias(key: &String) -> Self {
        Self::UndefinedAlias(format!("Alias {} is undefined", key))
    }

    pub fn wrong_alias(expected: &'static str, found: &Alias) -> Self {
        Self::WrongAlias(format!("Expected {} alias, found {:?}", expected, found))
    }
}

pub type MypsLexerResult<T> = Result<T, MypsLexerError>;
