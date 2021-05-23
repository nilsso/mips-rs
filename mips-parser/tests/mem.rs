#![feature(custom_inner_attributes)]
#![rustfmt::skip::macros(mips_ast_test)]
use mips_parser::{ast::all_node_variants::*, prelude::*, Rule::*};
use util::mips_ast_test;

mips_ast_test!(mem_lit_r0,   "r0",   mem, Mem, MemLit(0, 0));
mips_ast_test!(mem_lit_rr1,  "rr1",  mem, Mem, MemLit(1, 1));
mips_ast_test!(mem_lit_rrr5, "rrr5", mem, Mem, MemLit(5, 2));
mips_ast_test!(mem_alias_x,  "x",    mem, Mem, MemAlias("x".into()));
