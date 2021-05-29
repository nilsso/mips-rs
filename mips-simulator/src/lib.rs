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
#![feature(bool_to_option)]
#![feature(result_cloned)]
#![feature(result_flattening)]
#![feature(map_into_keys_values)]
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

use device::DeviceKinds;
use state::{AliasKind, ICState};

pub const MEM_SIZE: usize = 18;
pub const DEV_SIZE: usize = 6;
pub const STACK_SIZE: usize = 512;
pub const HOUSING: &'static str = "CircuitHousing";

/// The default Stationeers
pub type ICStateDefault<'dk> = ICState<'dk, MEM_SIZE, DEV_SIZE, STACK_SIZE>;

impl<'dk> Default for ICState<'dk, 18, 6, 512> {
    /// New Stationeers default IC state (without the self device set).
    fn default() -> Self {
        Self::new()
            .with_alias("sp", AliasKind::MemId(16))
            .with_alias("ra", AliasKind::MemId(17))
            .with_alias("db", AliasKind::DevSelf)
    }
}

/// New Stationeers default IC state with self device `"CircuitHousing"`.
///
/// Panics if `dev_kinds` did not contain a `"CircuitHousing"` key.
pub fn stationeers_ic(dev_kinds: &DeviceKinds) -> ICState<18, 6, 512> {
    let housing = dev_kinds[HOUSING].make();
    ICState::default().with_dev_self(housing)
}

/// All-in-one module.
pub mod prelude {
    pub use crate::device::{Device, DeviceKind, DeviceKinds};
    pub use crate::simulator::{ICSimulator, ICSimulatorError};
    pub use crate::state::{AliasKind, DevId, ICState, ICStateError};
    pub use crate::{stationeers_ic, ICStateDefault, Line};
}
