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
//! - Implement devices and add the `db` self device alias.
//! * Implement `DeviceKind` and `Device` structs, along with RON parsing to populate
//!     stock Stationeers device kinds.
//! * Consider adding error kind for wrong number of arguments.
//!     Not needed when using `mips_parser` to construct the AST since the parser dinstinguishes
//!     the number of args from too few or too many,
//!     but absolutely needed for manual expression constructions.
//!
//! * Implement all the functions:
//!
//! - [x] Device IO
//!     - [x] `Bdns`
//!     - [x] `Bdnsal`
//!     - [x] `Bdse`
//!     - [x] `Bdseal`
//!     - [x] `Brdns`
//!     - [x] `Brdse`
//!     - [x] `L`
//!     - [x] `Lb`
//!     - [x] `Lr`
//!     - [x] `Ls`
//!     - [x] `S`
//!     - [x] `Sb`
//! - [x] Flow Control, Branches and Jumps
//!     - [x] `Bap`
//!     - [x] `Bapal`
//!     - [ ] `Bapz`
//!     - [ ] `Bapzal`
//!     - [x] `Beq`
//!     - [x] `Beqal`
//!     - [x] `Beqz`
//!     - [x] `Beqzal`
//!     - [x] `Bge`
//!     - [x] `Bgeal`
//!     - [x] `Bgez`
//!     - [x] `Bgezal`
//!     - [x] `Bgt`
//!     - [x] `Bgtal`
//!     - [x] `Bgtz`
//!     - [x] `Bgtzal`
//!     - [x] `Ble`
//!     - [x] `Bleal`
//!     - [x] `Blez`
//!     - [x] `Blezal`
//!     - [x] `Blt`
//!     - [x] `Bltal`
//!     - [x] `Bltz`
//!     - [x] `Bltzal`
//!     - [x] `Bna`
//!     - [x] `Bnaal`
//!     - [x] `Bnaz`
//!     - [x] `Bnazal`
//!     - [x] `Bne`
//!     - [x] `Bneal`
//!     - [x] `Bnez`
//!     - [x] `Bnezal`
//!     - [x] `Brap`
//!     - [ ] `Brapz`
//!     - [x] `Breq`
//!     - [x] `Breqz`
//!     - [x] `Brge`
//!     - [x] `Brgez`
//!     - [x] `Brgt`
//!     - [x] `Brgtz`
//!     - [x] `Brle`
//!     - [x] `Brlez`
//!     - [x] `Brlt`
//!     - [x] `Brltz`
//!     - [x] `Brna`
//!     - [x] `Brnaz`
//!     - [x] `Brne`
//!     - [x] `Brnez`
//!     - [x] `J`
//!     - [x] `Jal`
//!     - [x] `Jr`
//! - [ ] Variable selection
//!     - [ ] `Sap`
//!     - [ ] `Sapz`
//!     - [ ] `Sdns`
//!     - [ ] `Sdse`
//!     - [ ] `Select`
//!     - [x] `Seq`
//!     - [ ] `Seqz`
//!     - [ ] `Sge`
//!     - [ ] `Sgez`
//!     - [ ] `Sgt`
//!     - [ ] `Sgtz`
//!     - [ ] `Sle`
//!     - [ ] `Slez`
//!     - [ ] `Slt`
//!     - [ ] `Sltz`
//!     - [ ] `Sna`
//!     - [ ] `Snaz`
//!     - [ ] `Sne`
//!     - [ ] `Snez`
//! - [ ] Mathematical Operations
//!     - [ ] `Abs`
//!     - [ ] `Acos`
//!     - [x] `Add`
//!     - [ ] `Asin`
//!     - [ ] `Atan`
//!     - [ ] `Ceil`
//!     - [ ] `Cos`
//!     - [ ] `Div`
//!     - [ ] `Exp`
//!     - [ ] `Floor`
//!     - [ ] `Log`
//!     - [ ] `Max`
//!     - [ ] `Min`
//!     - [ ] `Mod`
//!     - [ ] `Mul`
//!     - [ ] `Rand`
//!     - [ ] `Round`
//!     - [ ] `Sin`
//!     - [ ] `Sqrt`
//!     - [ ] `Sub`
//!     - [ ] `Tan`
//!     - [ ] `Trunc`
//! - [x] Logic
//!     - [x] `And`
//!     - [x] `Nor`
//!     - [x] `Or`
//!     - [x] `Xor`
//! - [x] Stack
//!     - [x] `Peek`
//!     - [x] `Pop`
//!     - [x] `Push`
//! - [x] Misc
//!     - [x] `Alias`
//!     - [x] `Define`
//!     - [x] `Hcf`
//!     - [x] `Move`
//!     - [x] `Sleep`
//!     - [x] `Yield`
//! - [x] Label
//!     - [x] `Label`
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

pub mod state;
pub mod simulator;
pub mod device;

/// All-in-one module.
pub mod prelude {
    pub use crate::Line;
    pub use crate::state::{ICStateError, AliasKind, ICState};
    pub use crate::simulator::{ICSimulator, ICSimulatorError};
    pub use crate::device::{Device, DeviceKind, DeviceKinds};
}

// For documentation links;
#[allow(unused_imports)]
use prelude::*;

