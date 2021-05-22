//! Stationeers MIPS abstract syntax tree (AST) definition.

pub mod nodes;

pub mod all_node_variants {
    pub use crate::ast::nodes::{
        Arg::*,
        Dev::*,
        Func::*,
        Mem::*,
        Val::*,
    };
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

