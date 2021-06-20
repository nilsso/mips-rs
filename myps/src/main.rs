#![feature(bool_to_option)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_assignments)]

use std::io::Read;

use myps::superprelude::*;

fn print_inteference_graph(translator: &Translator) {
    print!("    ");
    for j in 0..translator.units.len() {
        if j % 10 == 0 {
            print!("{:0<2}", j);
        } else {
            print!("  ");
        }
    }
    println!();
    print!("     ");
    for j in 0..translator.units.len() {
        print!("{:<2}", j % 10);
    }
    println!();
    for i in 0..translator.var_next_id {
        let (s, e) = translator.var_lifetimes[i].clone();
        print!("r{:<2} ", i);
        for j in 0..translator.units.len() {
            if s <= j && j <= e {
                if j == s && j == e {
                    print!("{:>2}", j);
                } else if j == s {
                    print!("{:>2}", j);
                } else if j == e {
                    print!("{:->2}", j);
                } else {
                    print!("--");
                }
            } else {
                print!(" |");
            }
        }
        println!();
    }
}

fn main() {
    // SOURCE
    // let source = "x = d(i).Setting";
    // let peg = MypsParser::parse(Rule::stmt, &source)?;
    // println!("{:#?}", peg);

    let args = std::env::args().collect::<Vec<String>>();
    let path = &args[1];
    let source = std::fs::read_to_string(path).unwrap();
    // let source = "push(123)";

    // for line in source.split("\n") {
    //     println!("# {}", line);
    // }
    // println!("# ==========================");

    // PARSER TEST
    let rule = Rule::program;
    // let rule = Rule::stmt;
    // let rule = Rule::expr_line;
    // let rule = Rule::rv_expr_line;
    // let rule = Rule::stmt_assign_value_line;
    let peg = MypsParser::parse(rule, &source).unwrap();
    // println!("{:#?}", peg);

    // LEXER TEST
    let program_pair = peg.only_inner().unwrap();
    let (program_item, functions) = lex_program_pair(program_pair).unwrap();

    // // TRANSLATOR TEST
    let conf_path = "translator.ron";
    let conf_string = std::fs::read_to_string(conf_path).unwrap();
    let conf = ron::from_str(&conf_string).unwrap();

    let mut translator = Translator::translate(conf, program_item, functions).unwrap();
    // println!("{:#?}", translator);
    // println!("# ==========================");

    for (i, unit) in translator.units.iter().enumerate() {
        // println!("{:?}", unit);
        println!("{}: {}", i, unit);
        // println!("{}", unit);
    }
    println!("# ==========================");
    // for (i, lifetime) in translator.var_lifetimes.iter().enumerate() {
    //     println!("{} : {:?}", i, lifetime);
    // }
    // println!("# ==========================");
    print_inteference_graph(&translator);
    println!("# ==========================");
    translator.optimize_registers();
    for (i, unit) in translator.units.iter().enumerate() {
        // println!("{:?}", unit);
        println!("{}: {}", i, unit);
        // println!("{}", unit);
    }
}
