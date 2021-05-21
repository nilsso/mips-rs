use mips_parser::prelude::*;
use mips_state::*;

macro_rules! run {
    ($s:literal, $state:ident) => {
        let p = MipsParser::parse(Rule::expr, &$s).unwrap().inner();
        let a = Expr::from_pair(p);
        println!("{} -> {:?}", $s, $state.exec_expr(&a));
    }
}

fn main() {
    let mut state = MipsState::default()
        // .with_mem(14, 15)
        // .with_mem(15, 16)
        // .with_alias("x", AliasKind::MemId(3))
        // .with_alias("y", AliasKind::DevId(4))
        ;

    run!("alias x r2", state);
    run!("move x 6", state);

    println!("{:#?}", state);
}
