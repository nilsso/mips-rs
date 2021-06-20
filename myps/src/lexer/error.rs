use std::io::Error as IOError;
use std::num::{ParseFloatError, ParseIntError};
use std::{fmt, fmt::Display};

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

    FailedConversion(String),

    UndefinedAlias(String),
    WrongNumArgs(String),
    UndefinedFunction(String),
    RedefinedFunction(String),
    NestedFunction(String),

    StmtError(String),

    WrongReturn(String),

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

impl MypsLexerError {
    pub fn wrong_rule(expected: &'static str, found: Pair<Rule>) -> Self {
        Self::WrongRule(format!("Expected {} pair, found {:?}", expected, found))
    }

    pub fn undefined_alias<K: std::fmt::Debug>(k: K) -> Self {
        Self::UndefinedAlias(format!("Alias {:?} is undefined", k))
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
        Self::WrongNumArgs(format!(
            "{} function expects {} arg, found {}",
            kind, expected, found
        ))
    }

    pub fn undefined_function(name: &String) -> Self {
        Self::UndefinedFunction(format!("Function {} is undefined", name))
    }

    pub fn redefined_function(name: &String) -> Self {
        Self::RedefinedFunction(format!("Function {} cannot be redefined", name))
    }

    pub fn nested_function(name: &String) -> Self {
        Self::NestedFunction(format!(
            "Function {} cannot be defined within another blocks",
            name
        ))
    }

    pub fn stmt_error(stmt_string: String, err: MypsLexerError) -> Self {
        Self::StmtError(format!(
            "Encountered an error translating the following statement:
```
{}
```
With the error: {:?}",
            stmt_string, err
        ))
    }

    pub fn failed_conversion<T: std::fmt::Debug>(expected: &'static str, found: T) -> Self {
        Self::FailedConversion(format!("Expected {}, found {:?}", expected, found))
    }
}

impl Display for MypsLexerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MypsLexerError::PegError(err) => {
                write!(f, "Parser error: {:?}", err)
            }
            MypsLexerError::AstError(err) => {
                write!(f, "AST base error: {:?}", err)
            }
            MypsLexerError::IOError(err) => {
                write!(f, "IO error: {:?}", err)
            }
            MypsLexerError::ParseIntError(err) => {
                write!(f, "ParseInt error: {:?}", err)
            }
            MypsLexerError::ParseFloatError(err) => {
                write!(f, "ParseFloat error: {:?}", err)
            }
            // Lexer errors
            MypsLexerError::MisplacedElif(s)
            | MypsLexerError::MisplacedElse(s)
            | MypsLexerError::BranchStatement(s) => {
                write!(f, "{}", s)
            },

            MypsLexerError::WrongRule(s)
            | MypsLexerError::ExpectedIndent(s)
            | MypsLexerError::WrongIndent(s)
            | MypsLexerError::FailedConversion(s)
            | MypsLexerError::UndefinedAlias(s)
            | MypsLexerError::WrongNumArgs(s)
            | MypsLexerError::UndefinedFunction(s)
            | MypsLexerError::RedefinedFunction(s)
            | MypsLexerError::NestedFunction(s)
            | MypsLexerError::StmtError(s)
            | MypsLexerError::WrongReturn(s) => {
                write!(f, "{}", s)
            }

            MypsLexerError::Dummy => {
                write!(f, "Dummy error")
            }
        }
    }
}
