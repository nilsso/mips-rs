mod types;
pub use types::{Int, Num, Dev, Var};

mod expr;
pub use expr::Expr;
pub use expr::operator::{UnaryOp, BinaryOp};
pub use expr::l_value::LValue;
pub use expr::r_value::{RValue, Mode};

mod statement;
pub use statement::Statement;

mod branch;
pub use branch::Branch;

mod line;
pub use line::Line;

mod program;
pub use program::Program;

use std::fmt;

pub trait DisplayMips {
    fn fmt_mips(&self, f: &mut fmt::Formatter) -> fmt::Result;
}
