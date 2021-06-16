use std::io::Error as IOError;
use std::num::{ParseFloatError, ParseIntError};

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
    MisplacedElif(&'static str),
    MisplacedElse(&'static str),
    BranchStatement(&'static str),

    FuncUndefined(String),
    FuncWrongNumArgs(String),

    Dummy,
}

pub type MypsLexerResult<T> = Result<T, MypsLexerError>;

impl_from_error!(
    MypsLexerError,
    PegError,
    AstError,
    IOError,
    ParseIntError,
    ParseFloatError
);

use std::fmt::Debug;

impl MypsLexerError {
    pub fn wrong_rule(expected: &'static str, found: Pair<Rule>) -> Self {
        Self::WrongRule(format!("Expected {} pair, found {:?}", expected, found))
    }

    pub fn undefined_alias<K: Debug>(k: K) -> Self {
        Self::UndefinedAlias(format!("Alias {:?} is undefined", k))
    }

    pub fn wrong_alias<A: Debug>(expected: &'static str, found: A) -> Self {
        Self::WrongAlias(format!("Expected {} alias, found {:?}", expected, found))
    }

    pub fn expected_indent(expected: usize) -> Self {
        Self::ExpectedIndent(format!("Expected indent of {} or more", expected))
    }

    pub fn wrong_indent(expected: usize, found: usize) -> Self {
        Self::WrongIndent(format!("Expected indent of {}, found {}", expected, found))
    }

    pub fn misplaced_elif() -> Self {
        Self::MisplacedElif("Elif blocks must follow an if or elif block")
    }

    pub fn misplaced_else() -> Self {
        Self::MisplacedElse("Else blocks must follow an if or elif block")
    }

    pub fn branch_statement() -> Self {
        Self::BranchStatement("Items can't be constructed from branch statements")
    }

    pub fn func_undefined(name: &String) -> Self {
        Self::FuncUndefined(format!("Function {} is undefined", name))
    }

    pub fn func_wrong_num_args(name: &String, expected: usize, found: usize) -> Self {
        Self::FuncWrongNumArgs(format!(
            "Function {} expected {} arguments, found {}",
            name, expected, found
        ))
    }
}
