#![feature(custom_inner_attributes)]
#![rustfmt::skip::macros(mips_ast_test)]
use mips_parser::{ast::all_node_variants::*, prelude::*, Rule::*};
use util::mips_ast_test;

mips_ast_test!(dev_lit_d0,   "d0",   dev, Dev, DevLit(0, 0));
mips_ast_test!(dev_lit_dr2,  "dr2",  dev, Dev, DevLit(2, 1));
mips_ast_test!(dev_lit_drr4, "drr4", dev, Dev, DevLit(4, 2));
mips_ast_test!(dev_alias_y,  "y",    dev, Dev, DevAlias("y".into()));
