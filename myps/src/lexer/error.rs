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
    ExpectedIndent(String),
    WrongIndent(String),
    MisplacedElif(&'static str),
    MisplacedElse(&'static str),
    BranchStatement(&'static str),

    UndefinedAlias(String),
    WrongAlias(String),
    WrongNumArgs(String),
    UndefinedFunction(String),
    RedefinedFunction(String),
    NestedFunction(String),

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

    pub fn wrong_num_args(kind: &'static str, expected: usize, found: usize) -> Self {
        Self::WrongNumArgs(format!("{} function expects {} arg, found {}", kind, expected, found))
    }

    pub fn undefined_function(name: &String) -> Self {
        Self::UndefinedFunction(format!("Function {} is undefined", name))
    }

    pub fn redefined_function(name: &String) -> Self {
        Self::RedefinedFunction(format!("Function {} cannot be redefined", name))
    }

    pub fn nested_function(name: &String) -> Self {
        Self::NestedFunction(format!("Function {} cannot be defined within another blocks", name))
    }
}
