use mips_parser::prelude::*;
use mips_simulator::prelude::*;

macro_rules! run {
    ($s:literal, $state:ident) => {
        let p = MipsParser::parse(Rule::expr, &$s).unwrap().first_inner();
        if let Err(e) = &p {
            println!("{:?}", e);
        }
        let p = p.unwrap();
        let a = Expr::from_pair(p);
        if let Err(e) = &a {
            println!("{:?}", e);
        }
        let a = a.unwrap();
        println!("{} -> {:?}", $s, $state.exec_expr(&a));
    }
}

fn main() {
    let mut state = ICState::default()
        // .with_mem(14, 15)
        // .with_mem(15, 16)
        // .with_alias("x", AliasKind::MemId(3))
        // .with_alias("y", AliasKind::DevId(4))
        ;

    run!("alias x r2", state);
    run!("move x 6", state);

    println!("{:#?}", state);
}
