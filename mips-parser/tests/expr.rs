#![feature(custom_inner_attributes)]
#![rustfmt::skip::macros(mips_ast_test)]
use mips_parser::{ast::all_node_variants::*, prelude::*, Rule::*};
use util::mips_ast_test;

// Test blank expressions
mips_ast_test!(expr_blank, "\n",       expr, Expr, None);
mips_ast_test!(expr_comment, "# test\n", expr, Expr, None);

macro_rules! t {
    ($t:literal) => {
        ArgToken($t.into())
    };
}
macro_rules! a {
    ($a:literal) => {
        ArgAlias($a.into())
    };
}
macro_rules! r {
    ($arg:path, $inner:path, $i:literal, $j:literal) => {
        $arg($inner($i, $j))
    };
    ($arg:path, $inner:path, $a:literal) => {
        $arg($inner($a.into()))
    };
}
macro_rules! m {
    ($i:literal, $j:literal) => {
        r!(ArgMem, MemLit, $i, $j)
    };
    ($a:literal) => {
        r!(ArgMem, MemAlias, $a)
    };
}
macro_rules! v {
    ($n:literal) => {
        ArgVal(ValLit($n.into()))
    };
    ($i:literal, $j:literal) => {
        ArgVal(ValMem(MemLit($i, $j)))
    };
}
macro_rules! e {
    ($f:ident, $($arg:expr),*$(,)*) => { Some(Expr($f, vec![$($arg),*])) }
}

mips_ast_test!(expr_alias_x_r0, "alias x r0\n",  expr, Expr, e!(Alias, t!("x"),  m!(0, 0)));
mips_ast_test!(expr_alias_x_y,  "alias x y\n",   expr, Expr, e!(Alias, t!("x"),  a!("y")));
mips_ast_test!(expr_move_r1_7,  "move r1 7\n",   expr, Expr, e!(Move,  m!(1, 0), v!(7)));
mips_ast_test!(expr_move_r1_r2, "move r1 rr2\n", expr, Expr, e!(Move,  m!(1, 0), v!(2, 1)));
