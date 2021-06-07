#![feature(bool_to_option)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_assignments)]

use std::io::Read;

use myps::superprelude::*;

fn main() -> MypsLexerResult<()> {
    // let source = "x = d(i).Setting";
    // let peg = MypsParser::parse(Rule::stmt, &source)?;
    // println!("{:#?}", peg);

    let path = "./test.myps";
    // let path = "./test2.myps";
    let source = std::fs::read_to_string(path).unwrap();
    // PARSER TEST
    let peg = MypsParser::parse(Rule::program, &source)?;
    // println!("{:#?}", peg);
    // LEXER TEST
    let (program, alias_table) = lex(peg)?;
    // println!("{:#?}", program);
    // TRANSLATOR TEST
    // let translator = Translator::parse_and_translate(&source)?;
    let mut translator = Translator::new();
    translator.translate_item(program);
    // println!("{:#?}", translator);
    for unit in translator.units.iter() {
        println!("{}", unit);
    }

    Ok(())
}
