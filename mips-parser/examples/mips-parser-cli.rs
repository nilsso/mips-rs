#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_parens)]

use std::fs::File;
use std::io::{stdin, BufRead, BufReader};

use clap::{load_yaml, App, ArgMatches};
use pest::{error::Error as PestError, iterators::Pairs};

use mips_parser::prelude::*;

fn main() {
    if let Err(err) = cli() {
        println!("Error: {:?}", err);
    }
}

fn cli() -> Result<(), std::io::Error> {
    let yaml = load_yaml!("mips-parser-cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();

    let pretty = (matches.occurrences_of("pretty") > 0);
    let output = matches.value_of("output").unwrap();
    let file = matches.value_of("file");
    // let pretty = matches.value_of

    let as_peg = (output == "peg");
    let as_ast = (output == "ast");

    if let Some(path) = file {
        let file = std::fs::File::open(path)?;
        for line in std::io::BufReader::new(file).lines() {
            exec_line(&line?, as_peg, as_ast, pretty);
        }
    } else {
        let stdin = std::io::stdin();
        let mut buffer = String::new();
        while stdin.read_line(&mut buffer).map(|b| b > 0).unwrap_or(false) {
            let line = buffer.to_owned();
            buffer.clear();
            exec_line(&line, as_peg, as_ast, pretty);
        }
    }
    Ok(())
}

fn exec_line(line: &String, as_peg: bool, as_ast: bool, pretty: bool) {
    let pairs = MipsParser::parse(Rule::expr, &line);
    if let Err(e) = &pairs {
        println!("{:?}", e);
        return;
    }
    let pair_res = pairs
        .unwrap()
        .first_inner()
        .or(Err(MipsParserError::AstError(AstError::InsufficientPairs)));
    if let Err(e) = &pair_res {
        println!("{:?}", e);
        return;
    }
    let pair = pair_res.unwrap();
    if as_peg {
        if pretty {
            println!("{:#?}", pair);
        } else {
            println!("{:?}", pair);
        }
        return;
    }
    let ast_res = Expr::from_pair(pair).map_err(MipsParserError::AstError);
    if let Err(e) = &ast_res {
        println!("{:?}", e);
        return;
    }
    let ast = ast_res.unwrap();
    if as_ast {
        if pretty {
            println!("{:#?}", ast);
        } else {
            println!("{:?}", ast);
        }
        return;
    }
    println!("{}", ast);
}
