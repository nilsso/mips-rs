#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_parens)]

use clap::{load_yaml, App, ArgMatches};

use pest::{error::Error as PestError, iterators::Pairs};

use mips_parser::prelude::*;

fn cli() -> Result<(), MipsParserError> {
    let yaml = load_yaml!("mips-parser-cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();

    let output = matches.value_of("output").unwrap();

    // println!("{:?}", matches);

    let as_peg = (output == "peg");
    let as_ast = (output == "ast");

    let stdin = std::io::stdin();
    let mut buffer = String::new();
    while stdin.read_line(&mut buffer).map(|b| b > 0).unwrap_or(false) {
        let input = buffer.trim_end().to_owned();
        buffer.clear();

        let pairs = MipsParser::parse(Rule::expr, &input);
        if let Err(e) = &pairs {
            println!("{:?}", e);
            continue;
        }
        let pair = pairs
            .unwrap()
            .first_inner()
            .or(Err(MipsParserError::AstError(AstError::InsufficientPairs)))?;
        if as_peg {
            println!("{:?}", pair);
            continue;
        }
        let ast_res = Expr::from_pair(pair).map_err(MipsParserError::AstError);
        if let Err(e) = &ast_res {
            println!("{:?}", e);
            continue;
        }
        let ast = ast_res.unwrap();
        if as_ast {
            println!("{:?}", ast);
            continue;
        }
        println!("{}", ast);
    }

    // let mut pairs: Vec<Result<Pairs<Rule>, PestError<Rule>>> = Vec::new();

    Ok(())
}

fn main() {
    if let Err(err) = cli() {
        println!("Error: {:?}", err);
    }
    // use std::fs::read_to_string;
    // let path = "./example-scripts/solar.mips";
    // let source = read_to_string(path).unwrap();
    // let source = "s drr2 Parameter rr3";
    // println!("{}", source);
    // let pairs = MipsParser::parse(Rule::program, &source);
    // let pair = pairs.unwrap().next().unwrap();
    // let ast = Program::from_pair(program_pair);
    // let pair = MipsParser::parse(Rule::func, &source).unwrap().inner();
    // let ast = Expr::from_pair(pair);
    // println!("------------------");
    // for expr in ast.iter() {
    //     println!("{:?}", expr);
    // }
    // println!("{:?}", ast);
    // println!("------------------");
    // println!("{}", ast);
}
