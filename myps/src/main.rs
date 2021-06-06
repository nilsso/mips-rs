#![feature(bool_to_option)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_assignments)]
use std::io::Read;

use myps::prelude::*;
use myps::ast::{BinaryOp, Expr, Num, RValue, LValue};
use myps::lexer::Statement;
use myps::translator::Translator;

fn main() -> MypsLexerResult<()> {
    // let source = "if 1";
    // let mut stdin = std::io::stdin(); // We get `Stdin` here.
    // let mut buffer = String::new();
    // stdin.read_to_string(&mut buffer).unwrap();
    // let source = buffer.as_str();
    let source = std::fs::read_to_string("./test.myps").unwrap();
    // parse_and_translate(&source)?;

    let translator = Translator::parse_and_translate(&source)?;
    // println!("{:#?}", translator.units);
    for unit in translator.units.iter() {
        println!("{}", unit);
    }
    println!("{:#?}", translator.var_lookup);

    Ok(())
}
