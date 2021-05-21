#![allow(unused_imports)]
//! State simulator for [Stationeers][stationeers] [MIPS][mips] executing
//! [Integrated Circuit (IC10)][ic10] programmable circuit-boards.
//!
//! [stationeers]: https://store.steampowered.com/app/544550/Stationeers/
//! [mips]: https://stationeers-wiki.com/MIPS
//! [ic10]: https://stationeers-wiki.com/Integrated_Circuit_(IC10)

pub mod builder;
pub mod device;
pub mod state;

pub use builder::MipsStateBuilder;
pub use device::{Device, DeviceKind};
pub use state::{Alias, MipsState, StateError};

/// Common exports for using MIPS state.
pub mod prelude {
    pub use crate::MipsStateBuilder;
    pub use crate::{Alias, MipsState, StateError};
    pub use crate::{Device, DeviceKind};
}
