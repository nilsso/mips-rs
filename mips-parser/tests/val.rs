#![feature(custom_inner_attributes)]
#![rustfmt::skip::macros(mips_ast_test)]
use mips_parser::{ast::all_node_variants::*, prelude::*, Rule::*};
use util::mips_ast_test;

mips_ast_test!(val_lit_37,                 "37",         val, Val, ValLit(37.0));
mips_ast_test!(val_lit_128_46,             "-128.46",    val, Val, ValLit(-128.46));
mips_ast_test!(val_lit_3_14e5,             "3.14e5",     val, Val, ValLit(314000.0));
mips_ast_test!(val_lit_neg_801_75_e_neg_7, "-801.75e-7", val, Val, ValLit(-0.000080175));
mips_ast_test!(val_mem_r0,                 "r0",         val, Val, ValMem(MemLit(0, 0)));
mips_ast_test!(val_mem_rr1,                "rr1",        val, Val, ValMem(MemLit(1, 1)));
mips_ast_test!(val_mem_rrr5,               "rrr5",       val, Val, ValMem(MemLit(5, 2)));
mips_ast_test!(val_mem_x,                  "x",          val, Val, ValMem(MemAlias("x".into())));

