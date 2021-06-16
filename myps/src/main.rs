#![feature(bool_to_option)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_assignments)]

use std::io::Read;

use myps::superprelude::*;

// fn print_inteference_graph(translator: &Translator) {
//     print!("      ");
//     for j in 0..translator.units.len() + 1 {
//         if j % 10 == 0 {
//             print!("{:0<2}", j);
//         } else {
//             print!("  ");
//         }
//     }
//     println!();
//     print!("       ");
//     for j in 0..translator.units.len() + 1 {
//         print!("{:<2}", j % 10);
//     }
//     println!();
//     for i in 0..translator.var_next_id {
//         let (s, e) = translator.var_lifetimes[i].clone();
//         print!("r{:<2} | ", i);
//         for j in 0..translator.units.len() + 1 {
//             if s <= j && j <= e {
//                 if j == s && j == e {
//                     print!("()");
//                 } else if j == s {
//                     print!("(*");
//                 } else if j == e {
//                     print!("*)");
//                 } else {
//                     print!("--");
//                 }
//             } else {
//                 print!(" |");
//             }
//         }
//         println!();
//     }
// }

fn main() {
    // SOURCE
    // let source = "x = d(i).Setting";
    // let peg = MypsParser::parse(Rule::stmt, &source)?;
    // println!("{:#?}", peg);

    let args = std::env::args().collect::<Vec<String>>();
    let path = &args[1];
    // let path = "./test-scripts/sum-evens.myps";
    // let path = "./test-scripts/fib.myps";
    // let path = "./test-scripts/test.myps";
    let source = std::fs::read_to_string(path).unwrap();
    // let source = "yield()";
    // for line in source.split("\n") {
    //     println!("# {}", line);
    // }
    // println!("# ==========================");

    // PARSER TEST
    let rule = Rule::program;
    // let rule = Rule::func;
    let peg = MypsParser::parse(rule, &source);
    // println!("{:#?}", peg);

    // LEXER TEST
    let (program_item, lexer) = Lexer::lex(peg.unwrap()).unwrap();
    println!("{:#?}", program_item);

    // TRANSLATOR TEST
    // let mut translator = Translator::translate(program_item);
    // println!("===");
    // for (i, unit) in translator.units.iter().enumerate() {
    //     // println!("{:?}", unit);
    //     println!("{}: {}", i, unit);
    // }

    // for (i, unit) in translator.units.iter().enumerate() {
    // //     // println!("{:?}", unit);
    // //     // println!("{}: {}", i, unit);
    //     println!("{}", unit);
    // }
    // println!("# ==========================");
    // print_inteference_graph(&translator);
    // println!("# ==========================");
    // translator.optimize_registers();
    // for (i, unit) in translator.units.iter().enumerate() {
    //     // println!("{:?}", unit);
    //     // println!("{}: {}", i, unit);
    //     println!("{}", unit);
    // }
}
