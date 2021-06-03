//! IC10 simulator.
use mips_parser::prelude::*;

use crate::state::{ExecResult, ICState, ICStateError};
use crate::{MEM_SIZE, DEV_SIZE, STACK_SIZE, Line};

#[derive(Debug)]
pub enum ICSimulatorError {
    StateError(ICStateError),
    LineError(usize),
}

#[derive(Copy, Clone, Debug)]
pub enum SimStatus {
    Running(usize),
    Finished(usize),
}

impl SimStatus {
    pub fn index(&self) -> usize {
        match self {
            &SimStatus::Running(i) => i,
            &SimStatus::Finished(i) => i,
        }
    }
}

pub type ICSimulatorResult = Result<SimStatus, ICSimulatorError>;

#[derive(Clone, Debug)]
pub struct ICSimulator<const MS: usize, const DS: usize, const SS: usize> {
    pub state: ICState<MS, DS, SS>,
    pub lines: Vec<Line>,
}

// Alias for a simulator of the default Stationeers IC state.
pub type ICSimulatorDefault = ICSimulator<MEM_SIZE, DEV_SIZE, STACK_SIZE>;

impl<const MS: usize, const DS: usize, const SS: usize>
    ICSimulator<MS, DS, SS>
{
    /// Construct new IC simulator.
    pub fn new(state: ICState<MS, DS, SS>, program: Program) -> Self {
        let lines = Self::program_to_lines(program);
        Self { state, lines }
    }

    /// Load a new state.
    pub fn load_state(&mut self, state: ICState<MS, DS, SS>) {
        self.state = state;
    }

    /// Load a new program.
    pub fn load_program(&mut self, program: Program) {
        self.lines = Self::program_to_lines(program);
    }

    /// Helper to convert a program AST node to lines.
    pub fn program_to_lines(program: Program) -> Vec<Line> {
        let mut lines = Vec::new();
        for (i, expr) in program.into_iter() {
            while lines.len() < i {
                lines.push(Line::Blank(i));
            }
            lines.push(Line::Expr(i, expr));
        }
        lines
    }

    /// Iterator over the program lines.
    pub fn iter_lines(&self) -> impl Iterator<Item = &Line> {
        self.lines.iter()
    }

    /// Get index of next line.
    pub fn next_line_index(&self) -> usize {
        self.state.next_line_index
    }

    /// Get next line.
    pub fn next_line(&self) -> Option<&Line> {
        self.lines.get(self.state.next_line_index)
    }

    /// Has the simulator run out of program lines.
    pub fn is_finished(&self) -> bool {
        self.state.next_line_index >= self.lines.len()
    }

    /// Get the status of this simulator.
    pub fn status(&self) -> SimStatus {
        let i = self.state.next_line_index;
        if !self.is_finished() {
            SimStatus::Running(i)
        } else {
            SimStatus::Finished(i)
        }
    }

    /// Step once through the program.
    pub fn step(&mut self) -> ICSimulatorResult {
        let i = self.state.next_line_index;
        if self.is_finished() {
            return Err(ICSimulatorError::LineError(i));
        }
        let line = &self.lines[i];
        let exec_res = self
            .state
            .exec_line(line)
            .map_err(ICSimulatorError::StateError)?;
        match exec_res {
            ExecResult::Normal(jumped) => {
                if !jumped {
                    self.state.next_line_index += 1;
                }
            }
            ExecResult::Sleep(_) => {}
            ExecResult::Yield => {}
        }
        Ok(self.status())
    }

    /// Step n times through the program.
    pub fn step_n(&mut self, n: usize) -> ICSimulatorResult {
        for _ in 0..n {
            self.step()?;
        }
        Ok(self.status())
    }

    /// Run the simulator until it is finished.
    pub fn run_until_finished(&mut self) -> ICSimulatorResult {
        while !self.is_finished() {
            self.step()?;
        }
        Ok(self.status())
    }
}
