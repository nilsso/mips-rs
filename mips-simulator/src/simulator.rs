//! IC10 simulator.
use mips_parser::prelude::*;

use crate::prelude::{ICState, ICStateError, Line};

#[derive(Debug)]
pub enum ICSimulatorError {
    StateError(ICStateError),
    LineError(usize),
}

#[derive(Debug)]
pub enum StepResult {
    Ok(usize),
    End(usize),
}

impl StepResult {
    pub fn index(&self) -> usize {
        match self {
            &StepResult::Ok(i) => i,
            &StepResult::End(i) => i,
        }
    }
}

pub type ICSimulatorResult = Result<StepResult, ICSimulatorError>;

#[derive(Clone, PartialEq, Debug)]
pub struct ICSimulator {
    pub state: ICState,
    pub lines: Vec<Line>,
}

impl ICSimulator {
    pub fn new(state: ICState, program: Program) -> Self {
        let mut lines = Vec::new();
        for (i, expr) in program.into_iter() {
            while lines.len() < i {
                lines.push(Line::Blank(i));
            }
            lines.push(Line::Expr(i, expr));
        }
        Self {
            state,
            lines,
        }
    }

    pub fn iter_lines(&self) -> impl Iterator<Item = &Line> {
        self.lines.iter()
    }

    pub fn next_line_index(&self) -> usize {
        self.state.next_line_index
    }

    pub fn get_line(&self, i: usize) -> Option<&Line> {
        self.lines.get(i)
    }

    pub fn next_line(&self) -> Option<&Line> {
        self.get_line(self.state.next_line_index)
    }

    pub fn step(&mut self) -> ICSimulatorResult {
        let i = self.state.next_line_index;
        if i >= self.lines.len() {
            return Err(ICSimulatorError::LineError(i));
        }

        let line = &self.lines[i];

        let jumped = self.state.try_exec_line(line).map_err(ICSimulatorError::StateError)?;
        if !jumped {
            self.state.next_line_index += 1;
        }

        let i = self.state.next_line_index;
        if i >= self.lines.len() {
            Ok(StepResult::End(i))
        } else {
            Ok(StepResult::Ok(i))
        }
    }
}
