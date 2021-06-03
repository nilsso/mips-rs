//! IC10 state and simulator component of `mips-rs`.
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
//! ### TODO
//! - Check how the game handles domain errors on math functions
//! - Work on constant values (e.g. loading device kinds -> their hashes)
#![feature(bool_to_option)]
#![feature(result_cloned)]
#![feature(result_flattening)]
#![feature(int_error_matching)]

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

pub mod device;
pub mod simulator;
pub mod state;
pub mod test_utils;
pub mod watcher;

use device::Device;
use state::{AliasKind, ICState};

pub const MEM_SIZE: usize = 18;
pub const DEV_SIZE: usize = 6;
pub const STACK_SIZE: usize = 512;

impl Default for ICState<MEM_SIZE, DEV_SIZE, STACK_SIZE> {
    /// New Stationeers default IC state (without the self device set).
    fn default() -> Self {
        Self::new()
            .with_alias("sp", AliasKind::MemId(MEM_SIZE - 2))
            .with_alias("ra", AliasKind::MemId(MEM_SIZE - 1))
            .with_alias("db", AliasKind::DevSelf)
            .with_dev_self(Device::circuit_housing())
    }
}

/// All-in-one module.
pub mod prelude {
    pub use crate::device::{Device, DeviceKind, DeviceKinds};
    pub use crate::simulator::{ICSimulator, ICSimulatorError, ICSimulatorDefault};
    pub use crate::state::{AliasKind, DevId, ICState, ICStateError};
    pub use crate::{Line, DEV_SIZE, MEM_SIZE, STACK_SIZE};
    pub use ron::de::from_reader;
    pub use std::fs::File;
}
