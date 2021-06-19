#![allow(unused_variables)]
use std::collections::{HashMap, HashSet};

use crate::superprelude::*;

// mod alias;
pub mod ast;
mod error;
// pub mod lex;

// pub use alias::{Alias, AliasTable};
pub use error::{MypsLexerError, MypsLexerResult};

fn parse_line_pair(
    line_pair: Pair<Rule>,
) -> MypsLexerResult<Option<(usize, Pair<Rule>, Option<String>)>> {
    match line_pair.as_rule() {
        Rule::line => {
            let pairs = line_pair.into_inner();
            let mut indent = 0;
            let mut item_pair_opt = None;
            let mut comment_opt = None;
            for pair in pairs {
                match pair.as_rule() {
                    Rule::indent => {
                        indent += 1;
                    }
                    Rule::item => {
                        item_pair_opt = Some(pair.only_inner()?);
                    }
                    Rule::comment => {
                        comment_opt = Some(pair.as_str().into());
                    }
                    Rule::empty => {
                        return Ok(None);
                    }
                    _ => {
                        let err = MypsLexerError::wrong_rule(
                            "an indent, item (branch or statement), or comment",
                            pair,
                        );
                        unreachable!("{:?}", err);
                        // return Err(err);
                    }
                }
            }
            if let Some(item_pair) = item_pair_opt {
                Ok(Some((indent, item_pair, comment_opt)))
            } else {
                let err: MypsLexerError = AstError::NotEnoughPairs.into();
                unreachable!("{:?}", err);
                // err
            }
        }
        Rule::EOI => Ok(None),
        _ => {
            let err = MypsLexerError::wrong_rule("a line", line_pair);
            unreachable!("{:?}", err);
            // err
        }
    }
}

// #[derive(Clone, Debug)]
// pub enum Alias {
//     Dev(Dev),
//     // TODO: Add functions
//     Int(i64),
//     Num(f64),
//     Var,
// }

fn compile_program_item(
    program_pair: Pair<Rule>,
) -> MypsLexerResult<(Item, HashMap<String, (Block, Option<String>)>)> {
    // Parse and collect line pairs into (indent, item_pair, comment) tuples
    let lines = program_pair
        .into_inner()
        .map(parse_line_pair)
        // .collect::<MypsLexerResult<Vec<Option<(usize, Pair<Rule>, Option<String>)>>>>()?
        .collect::<MypsLexerResult<Vec<Option<(usize, Pair<Rule>, Option<String>)>>>>()
        .unwrap()
        .into_iter()
        .flatten()
        .collect::<Vec<(usize, Pair<Rule>, Option<String>)>>();

    let mut functions = HashMap::new();
    let mut block_items = vec![(Vec::new(), None)];
    let mut branches = vec![Branch::Program];

    let mut indent_stack = vec![0_usize];
    let mut curr_indent = 0_usize;
    let mut expect_indent = false;
    let mut if_elif_else_index = -1_isize;

    fn head_items<'a>(block_items: &'a Vec<(Vec<Item>, Option<String>)>) -> &'a Vec<Item> {
        &block_items.last().unwrap().0
    }

    fn had_if_or_elif(block_items: &Vec<(Vec<Item>, Option<String>)>) -> bool {
        head_items(block_items)
            .last()
            .map(|item| item.is_if())
            .unwrap_or(false)
    }

    fn get_if_or_elif_index(block_items: &Vec<(Vec<Item>, Option<String>)>) -> usize {
        head_items(block_items)
            .iter()
            .filter_map(Item::if_elif_else_index)
            .next()
            .unwrap()
    }

    fn compile_expr(expr: &Expr) -> MypsLexerResult<()> {
        match expr {
            Expr::Unary { rhs: box rhs, .. } => {
                compile_expr(rhs).unwrap();
            }
            Expr::Binary {
                lhs: box lhs,
                rhs: box rhs,
                ..
            } => {
                compile_expr(lhs).unwrap();
                compile_expr(rhs).unwrap();
            }
            Expr::Ternary {
                box cond,
                box if_t,
                box if_f,
            } => {
                compile_expr(cond).unwrap();
                compile_expr(if_t).unwrap();
                compile_expr(if_f).unwrap();
            }
            _ => {}
        }
        Ok(())
    }

    fn nest_next_block(
        block_items: &mut Vec<(Vec<Item>, Option<String>)>,
        branches: &mut Vec<Branch>,
        functions: &mut HashMap<String, (Block, Option<String>)>,
        indent_stack: &mut Vec<usize>,
    ) -> MypsLexerResult<()> {
        let (items, comment) = block_items.pop().unwrap();
        let branch = branches.pop().unwrap();
        indent_stack.pop();

        if let Branch::Def(name) = branch {
            if functions.contains_key(&name) {
                return Err(MypsLexerError::redefined_function(&name));
            } else if indent_stack.len() > 1 {
                return Err(MypsLexerError::nested_function(&name));
            } else {
                let function_block = Block {
                    branch: Branch::Function(name.to_owned()),
                    items,
                };
                functions.insert(name, (function_block, comment));
            }
        } else {
            let (head_items, _) = block_items.last_mut().unwrap();
            let block = Block { branch, items };
            head_items.push(Item::block(block, comment));
        }
        Ok(())
    }

    for (indent, item_pair, comment_opt) in lines.into_iter() {
        // Handle indentation
        if expect_indent {
            // Expecting increase in indent because previous line was a branch marker
            if indent <= curr_indent {
                let err = MypsLexerError::expected_indent(curr_indent + 1);
                unreachable!("{:?}", err);
                // return Err(err);
            } else {
                // Push this new indent level
                indent_stack.push(indent);
                curr_indent = indent;
                expect_indent = false;
            }
        } else {
            if indent < curr_indent {
                // Drop in indent level means the end of a branch
                while indent < *indent_stack.last().unwrap() {
                    nest_next_block(
                        &mut block_items,
                        &mut branches,
                        &mut functions,
                        &mut indent_stack,
                    )?;
                }
                curr_indent = *indent_stack.last().unwrap();
                if indent != curr_indent {
                    // If now the item and stack levels are different,
                    // this item level was never on the stack before
                    let err = MypsLexerError::wrong_indent(curr_indent, indent);
                    unreachable!("{:?}", err);
                    // return Err(err);
                } else {
                }
            }
        }
        // Handle item
        match item_pair.as_rule() {
            Rule::branch => {
                // let mut branch = item_pair.try_into_ast()?;
                let mut branch = item_pair.try_into_ast().unwrap();

                match &mut branch {
                    Branch::Program | Branch::Loop | Branch::Def(..) => {}
                    Branch::If(id, cond) => {
                        compile_expr(cond).unwrap();
                        if_elif_else_index += 1;
                        *id = if_elif_else_index as usize;
                    }
                    Branch::Elif(id, cond) => {
                        if !had_if_or_elif(&block_items) {
                            let err = MypsLexerError::misplaced_elif();
                            panic!("{:?}", err);
                            // return Err(err);
                        }
                        compile_expr(cond).unwrap();
                        *id = get_if_or_elif_index(&block_items);
                    }
                    Branch::Else(id) => {
                        if !had_if_or_elif(&block_items) {
                            let err = MypsLexerError::misplaced_else();
                            panic!("{:?}", err);
                            // return Err(err);
                        }
                        *id = get_if_or_elif_index(&block_items);
                    }
                    Branch::While(cond) => {
                        compile_expr(cond).unwrap();
                    }
                    Branch::For(name, s, e, step_opt) => {
                        compile_expr(s).unwrap();
                        compile_expr(e).unwrap();
                        if let Some(step) = &step_opt {
                            compile_expr(step).unwrap();
                        }
                    }
                    Branch::Function(name) => unreachable!("{:?}", name),
                }
                block_items.push((Vec::new(), comment_opt));
                branches.push(branch);
                expect_indent = true;
            }
            Rule::stmt => {
                let stmt = item_pair.try_into_ast()?;
                let (head_items, _) = block_items.last_mut().unwrap();
                head_items.push(Item::statement(stmt, comment_opt));
            }
            _ => {
                let err = MypsLexerError::wrong_rule("a block or statement", item_pair);
                unreachable!("{:?}", err);
                // return Err(err);
            }
        }
    }

    while block_items.len() > 1 {
        nest_next_block(
            &mut block_items,
            &mut branches,
            &mut functions,
            &mut indent_stack,
        )?;
    }

    let (program_items, comment) = block_items.pop().unwrap();
    let program_branch = branches.pop().unwrap();
    let program_block = Block::new(program_branch, program_items);
    let program_item = Item::block(program_block, comment);

    Ok((program_item, functions))
}

/// Convert MYPS parser output pairs to an abstract syntax tree.
pub fn lex_program_pair(
    program_pair: Pair<Rule>,
) -> MypsLexerResult<(Item, HashMap<String, (Block, Option<String>)>)> {
    let (program_item, functions) = compile_program_item(program_pair).unwrap();

    let mut aliases = HashSet::new();
    let mut called_functions = HashSet::new();

    // Analyze the body of the program (excluding user functions)
    program_item.analyze(&mut aliases, &mut called_functions)?;

    // Pass to analyze user defined functions
    for name in called_functions.clone() {
        let (function, _) = functions
            .get(&name)
            .ok_or(MypsLexerError::UndefinedFunction(name.to_owned()))?;

        function.analyze(&mut aliases, &mut called_functions)?;
    }

    // Pass again to make sure that user functions didn't call any undefined functions
    for name in called_functions {
        let function = functions
            .get(&name)
            .ok_or(MypsLexerError::UndefinedFunction(name.to_owned()))?;
    }

    Ok((program_item, functions))
}
