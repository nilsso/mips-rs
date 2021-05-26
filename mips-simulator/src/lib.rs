//! Integrated Circuit (IC10) simulator.
//!
//! A new simulator state can be constructed via [`ICState::default`] for one with
//! (per how IC10 chips are in Stationeers):
//!
//! * 18 memory registers,
//! * 6 device registers, and
//! * aliases `sp` and `ra` for memory registers 16 and 17 respectively.
//!
//! They can also be constructed manually via [`ICState::new`] by providing `mem_size` the number
//! of memory registers and `dev_size` the number of device registers.
//! A few helper-builder methods exist for setting the state memory register values
//! ([`with_mem`][`ICState::with_mem`]), state devices ([`with_dev`][`ICState::with_dev`]) and
//! aliases ([`with_alias`][`ICState::with_alias`]) at the call site.
//!
//! Note that internally the `sp` and `ra` *registers* are always `mem_size-2` and `mem_size-1`
//! respectively; that is, internal functions which modify `sp` will always modify the
//! `mem_size-1`-th register and likewise the `mem_size-2`-th register for `ra`,
//! regardless of how the `sp` and `ra` *aliases* are set.
//!
//! ## TODO
//!
//! - Implement devices and add the `db` self device alias.
#![feature(bool_to_option)]
#![feature(result_cloned)]
#![feature(result_flattening)]
#![feature(map_into_keys_values)]

use std::{fmt, fmt::Display};

use mips_parser::prelude::Expr;

/// Type for either an expression, or for representing a blank line.
#[derive(Clone, PartialEq, Debug)]
pub enum Line {
    Expr(usize, Expr),
    Blank(usize),
}

impl Display for Line {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Line::Expr(i, expr) => write!(fmt, r#"({}) "{}""#, i, expr),
            Line::Blank(i) => write!(fmt, r#"({}) (blank)"#, i),
        }
    }
}

pub mod state;
pub mod simulator;
pub mod device;

/// All-in-one module.
pub mod prelude {
    pub use crate::Line;
    pub use crate::state::{ICStateError, AliasKind, ICState};
    pub use crate::simulator::{ICSimulator, ICSimulatorError};
}

// For documentation links;
#[allow(unused_imports)]
use prelude::*;

