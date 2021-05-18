#![feature(bool_to_option)]
// #![allow(unused_variables)]
// #![allow(unused_mut)]
//! A Rust based parser for [Stationeers][stationeers] [MIPS][mips] code using [Pest][pest],
//! and state simulator for MIPS executing IC10 chips.
//!
//! [stationeers]: https://store.steampowered.com/app/544550/Stationeers/
//! [mips]: https://stationeers-wiki.com/MIPS
//! [pest]: https://pest.rs/

use pest_derive::Parser;

/// Stationeers MIPS language parser.
#[derive(Parser)]
#[grammar = "mips.pest"]
pub struct MipsParser;

pub mod ast;
pub mod state;

pub mod prelude {
    pub use pest::Parser;
    pub use crate::{
        {Rule, MipsParser},
        ast::{Node, Register, Value},
        state::{Alias, StateError, MipsState, MipsStateBuilder},
    };
}
