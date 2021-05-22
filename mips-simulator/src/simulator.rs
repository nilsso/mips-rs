//! IC10 simulator.

use mips_parser::prelude::*;

use crate::state::ICState;
// use crate::ICError;
// use crate::Mips as Error;

#[derive(Clone, PartialEq, Debug)]
pub struct ICSimulator {
    pub state: ICState,
    pub program: Program,
    pub next_line: usize,
}

impl ICSimulator {
    pub fn new(state: ICState, program: Program) -> Self {
        Self {
            state,
            program,
            next_line: 0,
        }
    }
}
