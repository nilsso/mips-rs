#![feature(bool_to_option)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_assignments)]
use std::{fmt, fmt::Display};

use myps::prelude::*;

// use pest::iterators::FlatPairs;

// pub struct Block {
// };

fn line_into_item(pair: Pair<Rule>) -> Option<(usize, Pair<Rule>)> {
    match pair.as_rule() {
        Rule::EOI => None,
        Rule::line => {
            let mut indent = 0;
            for item in pair.into_inner() {
                match item.as_rule() {
                    Rule::empty => return None,
                    Rule::indent => indent += 1,
                    _ => return Some((indent, item)),
                }
            }
            unreachable!();
        },
        _ => unreachable!(format!("{:?}", pair)),
    }
}

#[derive(Debug)]
pub enum BlockKind {
    Program,
    Loop,
}

impl<'i> AstNode<'i, Rule, MypsParser, MypsParserError> for BlockKind {
    type Output = Self;

    const RULE: Rule = Rule::branch;

    fn try_from_pair(pair: Pair<Rule>) -> MypsParserResult<Self> {
        match pair.as_rule() {
            _ => Ok(Self::Loop),
        }
    }
}

impl Display for BlockKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use BlockKind::*;

        match self {
            Program => write!(f, "(program)"),
            Loop => write!(f, "loop:"),
        }
    }
}

#[derive(Debug)]
pub struct Block(pub BlockKind, pub Vec<Item>);

impl Block {
    pub fn new(block_kind: BlockKind) -> Self {
        Self(block_kind, Vec::new())
    }

    pub fn push(&mut self, item: Item) {
        self.1.push(item);
    }
}

use std::iter::once;

fn print_block(block: &Block, indent: usize) {
    for item in block.1.iter() {
        match item {
            Item::Branch(block) => {
                if !matches!(block.0, BlockKind::Program) {
                    for _ in 0..indent {
                        print!("    ");
                    }
                    println!("{}", block.0);
                }
                print_block(block, indent + 1);
            },
            Item::Statement(stmt) => {
                for _ in 0..(indent) {
                    print!("    ");
                }
                println!("{}", stmt);
            },
        }
    }
}

// impl Display for Block {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         let item_strs = self.1.iter().map(Item::to_string).collect::<Vec<String>>();
//         println!("({})", item_strs.len());
//         match self.0 {
//             BlockKind::Program => {
//                 for item_str in item_strs {
//                     f.pad(": 4>")?;
//                     f.write("{}", item_str)?;
//                     // writeln!(f, "{}", item_str)?;
//                 }
//             },
//             _ => {
//                 write!(f, "{}", self.0)?;
//                 for item_str in item_strs {
//                     write!(f, "    {}", item_str)?;
//                 }
//             }
//         }
//         Ok(())
//     }
// }

// impl<'i> AstNode<'i, Rule, MypsParser, MypsParserError> for Block {
//     type Output = Self;

//     const RULE: Rule = Rule::stmt;

//     #[allow(unused_variables)]
//     fn try_from_pair(pair: Pair<Rule>) -> MypsParserResult<Self> {
//         Ok(match pair.as_rule() {
//             Rule::assign_r_value => {
//             },
//             Rule::assign_l_value => {
//             },
//             _ => Err(MypsParserError::new_wrong_rule("Stmt", pair)),
//         })
//         // todo!()
//         // match pair.as_rule() {
//         //     Rule::token => Ok(Token(pair.as_str().into())),
//         //     _ => Err(MypsParserError::new_wrong_rule("token", pair)),
//         // }
//     }
// }

#[derive(Debug)]
pub enum Item {
    Branch(Block),
    Statement(Statement),
}

pub fn build_blocks(pairs: Pairs<Rule>) -> MypsParserResult<Block> {
    let mut blocks = vec![Block::new(BlockKind::Program)];
    let mut indent_stack = vec![0_usize];
    let mut expect_indent = false;

    for (item_indent, item) in pairs.filter_map(line_into_item) {
        // Handle indentation
        let curr_indent = *indent_stack.last().unwrap();
        if expect_indent {
            // Expecting increase in indent because previous line was a block marker
            if item_indent <= curr_indent {
                panic!("Expected indent");
            } else {
                expect_indent = false;
                // Push this new indent level
                indent_stack.push(item_indent);
            }
        } else {
            if item_indent < curr_indent {
                // Drop in indent level means the end of a block
                while item_indent < *indent_stack.last().unwrap() {
                    // Pop off stack levels until the item level is at or above the stack level
                    let block = blocks.pop().unwrap();
                    let head = blocks.last_mut().unwrap();
                    head.push(Item::Branch(block));
                    indent_stack.pop();
                }
                if item_indent != *indent_stack.last().unwrap() {
                    // If now the item and stack levels are different,
                    // this item level was never on the stack before
                    panic!("Incorrect indent");
                }
            }
        }
        // Handle item
        let rule = item.as_rule();
        match rule {
            Rule::branch => {
                // Push a new block and expect the next line to be indented
                let block_kind = item.try_into_ast()?;
                blocks.push(Block::new(block_kind));
                expect_indent = true;
            },
            Rule::assign_alias | Rule::assign_value => {
                let stmt = item.try_into_ast()?;
                let head = blocks.last_mut().unwrap();
                head.push(Item::Statement(stmt));
            }
            _ => return Err(MypsParserError::wrong_rule("a statement or branch", item)),
        }
    }
    // Reduce whats left into blocks of items
    while blocks.len() > 1 {
        let block = blocks.pop().unwrap();
        let head = blocks.last_mut().unwrap();
        head.push(Item::Branch(block));
    }
    Ok(blocks.pop().unwrap())
}

fn main() {
    use pest::Parser;
    use pest::iterators::FlatPairs;

    use myps::{MypsParser, Rule};
    // use myps::nodes::{expr_climb, Expr};
    use util::traits::{FirstInner, AstNode};

    // use std::io::Read;
    // use std::env;
    // let args = env::args().collect::<Vec<String>>();
    // let source = args[1].as_str();
    // let source = "2 / (1 / x)";
    // let mut buffer = String::new();
    // let mut stdin = std::io::stdin(); // We get `Stdin` here.
    // stdin.read_to_string(&mut buffer).unwrap();
    // let source = buffer.as_str();
    // let source = "1 + (x + 2) / y";
    // let res = MypsParser::parse(Rule::expr_line, &source);
    // println!("{:#?}", res);
    // let pair = res.unwrap().first_inner().unwrap();
    // let expr = Expr::try_from_pair(pair).unwrap();
    // println!("{:#?}", expr);
    // println!("{}", expr);
    // println!("{:#?}", Expr::try_from_pair(pair));

    let source = std::fs::read_to_string("./test.myps").unwrap();
    let res = MypsParser::parse(Rule::program, &source);
    if let Ok(peg) = res {
        let program = peg.first_inner().unwrap();
        let block = build_blocks(program.into_inner()).unwrap();
        println!("{:#?}", &block);
        print_block(&block, 0);
    } else {
        println!("ERROR: {:#?}", res);
    }
}
