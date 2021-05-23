#![feature(custom_inner_attributes)]
#![rustfmt::skip::macros(mips_ast_test)]
use mips_parser::{ast::all_node_variants::*, prelude::*, Rule::*};
use util::mips_ast_test;

mips_ast_test!(arg_reg_rrr8,             "rrr8",     reg, Arg, ArgMem(MemLit(8, 2)));
mips_ast_test!(arg_reg_drrr15,           "drrr15",   reg, Arg, ArgDev(DevLit(15, 3)));
mips_ast_test!(arg_mem_x,                "x",        mem, Arg, ArgMem(MemAlias("x".into())));
mips_ast_test!(arg_dev_y,                "y",        dev, Arg, ArgDev(DevAlias("y".into())));
mips_ast_test!(arg_alias_z,              "z",        reg, Arg, ArgAlias("z".into()));
mips_ast_test!(arg_val_neg_95_2_e_neg_3, "-95.2e-3", val, Arg, ArgVal(ValLit(-0.0952)));
mips_ast_test!(arg_token_hello,          "hello",    tkn, Arg, ArgToken("hello".into()));
