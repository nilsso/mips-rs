use mips_parser::prelude::*;

fn main() -> Result<(), ()> {
    let mut state = MipsState::default();
    for (i, x) in state.mem.iter_mut().enumerate() {
        *x = (i + 1) as f32;
    }

    // let unparsed_file = std::fs::read_to_string("test.mips").expect("Unable to read file");
    // let pairs = MipsParser::parse(Rule::file, &unparsed_file).expect("");
    // let program = build_ast(pairs.into_iter().next().unwrap());
    // state.validate(&program)?;

    state.aliases.insert("x".into(), Alias::Mem(5));

    let s = "rr5";
    println!("{}", s);
    let r = MipsParser::parse(Rule::reg, s).expect("").next().unwrap();
    let r = Node::new(r);
    println!("{:#?}", r);
    let r = r.as_register().unwrap();
    println!("{:#?}", r);
    match state.value(&r) {
        Ok(x) => println!("{}", x),
        Err(e) => println!("{}", e),
    };

    Ok(())
}
