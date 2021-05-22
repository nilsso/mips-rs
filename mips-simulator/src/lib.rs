//! Integrated Circuit (IC10) simulator.
//!
//! A new simulator state can be constructed via [`ICState::default`] for one with
//! (per how IC10 chips are in Stationeers):
//!
//! * 18 memory registers,
//! * 6 device registers, and
//! * aliases `sp` and `ra` for memory registers 16 and 17 respectively.
//!
//! They can also be constructed manually via [`ICState::new`] by providing the memory and device
//! register sizes.
//! A few helper-builder methods exist for setting the state memory register values
//! ([`with_mem`][`ICState::with_mem`]), state devices ([`with_dev`][`ICState::with_dev`]) and
//! aliases ([`with_alias`][`ICState::with_alias`]) at the call site.
//!
//! ## TODO
//!
//! - Implement devices and add the `db` self device alias.
#![feature(bool_to_option)]

mod exec;

pub mod state;
pub mod simulator;
pub mod device;

/// State simulator error type.
#[derive(Debug)]
pub enum ICError {
    AliasUnset,
    AliasWrongKind,
    ArgWrongKind,
    OutOfBounds,
}

/// Alias for `Result<T, ICError>`.
pub type ICResult<T> = std::result::Result<T, ICError>;

/// All-in-one module.
pub mod prelude {
    pub use crate::{ICError, ICResult};
    pub use crate::state::{AliasKind, ICState};
    pub use crate::simulator::ICSimulator;
}

// For documentation links;
#[allow(unused_imports)]
use prelude::*;

