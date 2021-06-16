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
                        item_pair_opt = Some(pair.first_inner()?);
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
                let err: MypsLexerError = AstError::InsufficientPairs.into();
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

#[derive(Clone, Debug)]
pub struct Lexer {
    pub(crate) alias_table: HashSet<String>,
    pub(crate) functions: HashMap<String, Block>,
    pub(crate) called_functions: HashSet<String>,
}

impl Lexer {
    fn compile_program_item(program_pair: Pair<Rule>) -> MypsLexerResult<(Item, Self)> {
        // Parse and collect line pairs into (indent, item_pair, comment) tuples
        let items = program_pair
            .into_inner()
            .map(parse_line_pair)
            // .collect::<MypsLexerResult<Vec<Option<(usize, Pair<Rule>, Option<String>)>>>>()?
            .collect::<MypsLexerResult<Vec<Option<(usize, Pair<Rule>, Option<String>)>>>>()
            .unwrap()
            .into_iter()
            .flatten()
            .collect::<Vec<(usize, Pair<Rule>, Option<String>)>>();

        let mut alias_table = HashSet::new();
        let functions = HashMap::new();
        let mut called_functions = HashSet::<String>::new();

        let mut blocks = vec![(Block::program(), None)];
        let mut indent_stack = vec![0_usize];
        let mut curr_indent = 0_usize;
        let mut expect_indent = false;
        let mut if_elif_else_index = -1_isize;

        fn head_items<'a>(blocks: &'a Vec<(Block, Option<String>)>) -> &'a Vec<Item> {
            &blocks.last().unwrap().0.items
        }

        fn had_if_or_elif(blocks: &Vec<(Block, Option<String>)>) -> bool {
            head_items(blocks)
                .last()
                .map(|item| item.is_if())
                .unwrap_or(false)
        }

        fn get_if_or_elif_index(blocks: &Vec<(Block, Option<String>)>) -> usize {
            head_items(blocks)
                .iter()
                .filter_map(Item::if_elif_else_index)
                .next()
                .unwrap()
        }

        fn compile_r_value(
            r_value: &RValue,
            called_functions: &mut HashSet<String>,
        ) -> MypsLexerResult<()> {
            match r_value {
                RValue::Expr(box e) => {
                    // compile_expr(e, called_functions)?;
                    compile_expr(e, called_functions).unwrap();
                }
                _ => {}
            }
            Ok(())
        }

        fn compile_expr(
            expr: &Expr,
            called_functions: &mut HashSet<String>,
        ) -> MypsLexerResult<()> {
            match expr {
                Expr::Unary { rhs: box rhs, .. } => {
                    // compile_expr(rhs, called_functions)?;
                    compile_expr(rhs, called_functions).unwrap();
                }
                Expr::Binary {
                    lhs: box lhs,
                    rhs: box rhs,
                    ..
                } => {
                    // compile_expr(lhs, called_functions)?;
                    compile_expr(lhs, called_functions).unwrap();
                    // compile_expr(rhs, called_functions)?;
                    compile_expr(rhs, called_functions).unwrap();
                }
                Expr::Ternary {
                    box cond,
                    box if_t,
                    box if_f,
                } => {
                    // compile_expr(cond, called_functions)?;
                    compile_expr(cond, called_functions).unwrap();
                    // compile_expr(if_t, called_functions)?;
                    compile_expr(if_t, called_functions).unwrap();
                    // compile_expr(if_f, called_functions)?;
                    compile_expr(if_f, called_functions).unwrap();
                }
                _ => {}
            }
            Ok(())
        }

        // for Line { indent, stmt, comment } in program_ast.lines.into_iter() {
        for (indent, item_pair, comment_opt) in items.into_iter() {
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
                        // Pop off stack levels until the item level is at or above the stack level
                        let (block, comment_opt) = blocks.pop().unwrap();
                        let (head, _) = blocks.last_mut().unwrap();
                        head.items.push(Item::block(block, comment_opt));
                        indent_stack.pop();
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
                        Branch::Program | Branch::Loop => {}
                        Branch::If(id, cond) => {
                            // compile_expr(expr, &mut self.called_functions)?;
                            compile_expr(cond, &mut called_functions).unwrap();
                            if_elif_else_index += 1;
                            *id = if_elif_else_index as usize;
                        }
                        Branch::Elif(id, cond) => {
                            if !had_if_or_elif(&blocks) {
                                let err = MypsLexerError::misplaced_elif();
                                panic!("{:?}", err);
                                // return Err(err);
                            }
                            // compile_expr(expr, &mut self.called_functions)?;
                            compile_expr(cond, &mut called_functions).unwrap();
                            *id = get_if_or_elif_index(&blocks);
                        }
                        Branch::Else(id) => {
                            if !had_if_or_elif(&blocks) {
                                let err = MypsLexerError::misplaced_else();
                                panic!("{:?}", err);
                                // return Err(err);
                            }
                            *id = get_if_or_elif_index(&blocks);
                        }
                        Branch::While(cond) => {
                            // compile_expr(expr, &mut self.called_functions)?;
                            compile_expr(cond, &mut called_functions).unwrap();
                        }
                        Branch::For(name, s, e, step_opt) => {
                            // compile_expr(s_expr, &mut self.called_functions)?;
                            compile_expr(s, &mut called_functions).unwrap();
                            // compile_expr(e_expr, &mut self.called_functions)?;
                            compile_expr(e, &mut called_functions).unwrap();
                            if let Some(step) = &step_opt {
                                // compile_expr(step_expr, &mut self.called_functions)?;
                                compile_expr(step, &mut called_functions).unwrap();
                            }
                        }
                        Branch::Def(..) => {
                            if !indent_stack.is_empty() {
                                panic!("Cannot define nested functions");
                            }
                            // unreachable!();
                        }
                    }
                    let block = Block::new(branch);
                    blocks.push((block, comment_opt));
                    expect_indent = true;
                }
                Rule::stmt => {
                    let stmt = item_pair.try_into_ast()?;
                    let (head, _) = blocks.last_mut().unwrap();
                    head.items.push(Item::statement(stmt, comment_opt));
                }
                _ => {
                    let err = MypsLexerError::wrong_rule("a block or statement", item_pair);
                    unreachable!("{:?}", err);
                    // return Err(err);
                }
            }
        }

        while blocks.len() > 1 {
            // Pop off stack levels until the item level is at or above the stack level
            let (block, comment_opt) = blocks.pop().unwrap();
            let (head, _) = blocks.last_mut().unwrap();
            head.items.push(Item::block(block, comment_opt));
            indent_stack.pop();
        }

        let (program_block, comment) = blocks.pop().unwrap();
        let program_item = Item::block(program_block, comment);
        let lexer = Lexer {
            alias_table,
            functions,
            called_functions,
        };

        Ok((program_item, lexer))
    }

    /// Convert MYPS parser output pairs to an abstract syntax tree.
    pub fn lex(mut pairs: Pairs<Rule>) -> MypsLexerResult<(Item, Self)> {
        let program_pair = pairs.next().unwrap();
        if pairs.next().is_some() {
            panic!();
        }

        // lexer.compile_blocks_and_funcs(program_pair)?;
        let (program_item, mut lexer) = Lexer::compile_program_item(program_pair).unwrap();

        // Validate that called functions are defined
        for func_name in lexer.called_functions.iter() {
            // match func_name.as_str() {
            //     "pow" if !lexer.functions.contains_key(func_name) => {
            //         lexer.functions.insert(func_name.clone(), Function::pow());
            //     },
            //     _ => {},
            // }
            if !lexer.functions.contains_key(func_name) {
                let err = MypsLexerError::func_undefined(func_name);
                panic!("{:?}", err);
                // return Err(err);
            }
        }

        // lexer.analyze_called_functions(&program_item)?;
        let program_item = lexer.analyze_item(program_item)?.unwrap();

        Ok((program_item, lexer))
    }

    // fn get_alias(&self, k: &String) -> MypsLexerResult<RValueReturn> {
    //     self.alias_table
    //         .get(k)
    //         .cloned()
    //         .ok_or(MypsLexerError::undefined_alias(k))
    // }

    fn analyze_item(&mut self, item: Item) -> MypsLexerResult<Option<Item>> {
        let Item {
            item_inner,
            comment,
        } = item;

        match item_inner {
            ItemInner::Block(block) => {
                let Block { branch, items } = block;

                match branch {
                    Branch::Def(name, arg_names) => {
                        unreachable!();
                        // self.analyze_def(items, name, arg_names, comment).and(Ok(None))
                    }
                    _ => {
                        let items = items
                            .into_iter()
                            .map(|item| self.analyze_item(item))
                            .collect::<MypsLexerResult<Vec<Option<Item>>>>()
                            .unwrap()
                            .into_iter()
                            .flatten()
                            .collect();

                        Ok(Some(Item {
                            item_inner: ItemInner::Block(Block { branch, items }),
                            comment,
                        }))
                    }
                }
            }
            ItemInner::Stmt(stmt) => {
                match &stmt {
                    Statement::AssignValue(l_values, r_values) => {
                        for r_value in r_values.iter() {
                            self.analyze_r_value(r_value)?;
                        }
                        for l_value in l_values.iter() {
                            self.analyze_l_value(l_value)?;
                            if let LValue::Var(k, ..) = l_value {
                                self.alias_table.insert(k.clone());
                            }
                        }
                    }
                    Statement::FunctionCall(name, args) => {
                        unreachable!("{:?}", stmt);
                    }
                };
                Ok(Some(Item::statement(stmt, comment)))
            }
        }
    }

    fn analyze_l_value(&mut self, l_value: &LValue) -> MypsLexerResult<()> {
        if let LValue::Param(dev, ..) = l_value {
            self.analyze_dev(dev)
        } else {
            Ok(())
        }
        // match l_value {
        //     LValue::Param(dev, _) => self.analyze_dev(dev),
        //     LValue::Var(k, _) => self.analyze_var(k),
        // }
    }

    // Returns the Ok of number of r_values returned by this r_value
    fn analyze_r_value(&mut self, r_value: &RValue) -> MypsLexerResult<()> {
        match r_value {
            RValue::Num(num) => {
                self.analyze_num(num)?;
            }
            RValue::Dev(dev) => {
                self.analyze_dev(dev)?;
            }
            RValue::DevParam(dev, ..) | RValue::NetParam(dev, ..) | RValue::DevSlot(dev, ..) => {
                self.analyze_dev(dev)?;
            }
            RValue::Expr(box e) => {
                self.analyze_expr(e)?;
            }
            RValue::Func(func, args) => {}
            RValue::Var(k) => {
                self.analyze_var(k)?;
            }
        }
        Ok(())
    }

    fn analyze_var(&self, k: &String) -> MypsLexerResult<()> {
        self.alias_table
            .contains(k)
            .then_some(())
            .ok_or(MypsLexerError::undefined_alias(k))
    }

    fn analyze_num(&self, num: &Num) -> MypsLexerResult<()> {
        match num {
            Num::Var(k) => self.analyze_var(k),
            _ => Ok(()),
        }
    }

    fn analyze_dev(&self, dev: &Dev) -> MypsLexerResult<()> {
        match dev {
            Dev::Var(k) => self.analyze_var(k),
            _ => Ok(()),
        }
    }

    // Reduce, validate and analyze an expression helper.
    fn analyze_expr(&mut self, expr: &Expr) -> MypsLexerResult<()> {
        match expr {
            Expr::Unary { op, box rhs } => {
                // analyze_expr(rhs, alias_table, dependencies)?;
                self.analyze_expr(rhs).unwrap();
            }
            Expr::Binary {
                op,
                box lhs,
                box rhs,
            } => {
                // analyze_expr(lhs, alias_table, dependencies)?;
                self.analyze_expr(lhs).unwrap();
                // analyze_expr(rhs, alias_table, dependencies)?;
                self.analyze_expr(rhs).unwrap();
            }
            Expr::Ternary {
                box cond,
                box if_t,
                box if_f,
            } => {
                // analyze_expr(cond, alias_table, dependencies)?;
                self.analyze_expr(cond).unwrap();
                // analyze_expr(if_t, alias_table, dependencies)?;
                self.analyze_expr(if_t).unwrap();
                // analyze_expr(if_f, alias_table, dependencies)?;
                self.analyze_expr(if_f).unwrap();
            }
            Expr::RValue(rv) => {
                // analyze_r_value(rv, alias_table, dependencies)?;
                self.analyze_r_value(rv).unwrap();
            }
        }
        Ok(())
    }
}
