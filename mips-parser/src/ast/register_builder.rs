//! Register builder utility.
//!
//! Begins with a `Register::Literal(i)` literal register and successive calls with `mem` and `dev`
//! wrap the literal register with a `Register::Memory` and a `Register::Device` respectively.
//!
//! Note that a register chain can only have a device register as its last wrapping.
//! For example, these work:
//! * `RegisterBuilder(0).dev()` ↔ "d0"
//! * `RegisterBuilder(1).mem().dev()` ↔ "dr1"
//! * `RegisterBuilder(2).mem().mem().dev()` ↔ "drr2"
//!
//! While these do not (these panic):
//! * `RegisterBuilder(3).dev().mem()` ↔ "rd3"
//! * `RegisterBuilder(4).mem().dev().mem().dev()` ↔ "drdr4"

use super::register::Register;

/// Register builder utility.
pub struct RegisterBuilder {
    head: Box<Register>,
}

impl RegisterBuilder {
    /// New register builder.
    pub fn new(i: usize) -> Self {
        Self {
            head: Box::new(Register::RegLit(i)),
        }
    }

    /// Wrap with a memory register.
    pub fn mem(mut self) -> Self {
        if let Register::RegDev(_) = *self.head {
            panic!();
        }
        self.head = Box::new(Register::RegMem(self.head));
        self
    }

    /// Wrap with a device register.
    pub fn dev(mut self) -> Self {
        if let Register::RegDev(_) = *self.head {
            panic!();
        }
        self.head = Box::new(Register::RegDev(self.head));
        self
    }

    /// Return the constructed `Register`.
    pub fn build(self) -> Register {
        *self.head
    }
}

