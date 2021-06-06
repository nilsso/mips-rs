use crate::ast::{Dev, LValue, RValue};
use crate::lexer::Block;

#[derive(Debug)]
pub enum Statement {
    Block(Block),
    AssignAlias(String, Dev),
    AssignValue(LValue, RValue),
}
