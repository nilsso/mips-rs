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

#[derive(Debug)]
pub struct Lexer {
    pub(crate) alias_table: HashMap<String, RValueReturn>,
    pub(crate) functions: HashMap<String, Function>,
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

        let mut alias_table = HashMap::new();
        let functions = Function::default_functions();
        let mut called_functions = HashSet::<String>::new();

        let mut blocks = vec![(Block::program(), None)];
        let mut indent_stack = vec![0_usize];
        let mut curr_indent = 0_usize;
        let mut expect_indent = false;
        let mut if_elif_else_index = -1_isize;

        let mut def_within = false;
        let mut def_had_return = false;

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

        fn compile_rvalue(
            rvalue: &RValue,
            called_functions: &mut HashSet<String>,
        ) -> MypsLexerResult<()> {
            match rvalue {
                RValue::Expr(box e) => {
                    // compile_expr(e, called_functions)?;
                    compile_expr(e, called_functions).unwrap();
                }
                RValue::Func(FunctionCall { name, .. }) => {
                    called_functions.insert(name.clone());
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
                if def_had_return && indent >= curr_indent {
                    panic!("Expected function block to end");
                }
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
                            def_within = true;
                        }
                    }
                    let block = Block::new(branch);
                    blocks.push((block, comment_opt));
                    expect_indent = true;
                }
                Rule::stmt => {
                    // let stmt = item_pair.try_into_ast()?;
                    let stmt = item_pair.try_into_ast().unwrap();
                    match &stmt {
                        // Statement::AssignAlias(a_var, dev) => {
                        //     let alias = Alias::Dev(dev.to_owned());
                        //     alias_table.insert(a_var.clone(), alias);
                        // }
                        Statement::AssignValue(lvalues, rvalues) => {
                            for rvalue in rvalues.iter() {
                                compile_rvalue(rvalue, &mut called_functions).unwrap();
                            }
                        }
                        Statement::FunctionCall(name, _) => {
                            called_functions.insert(name.clone());
                        }
                        Statement::Return(returns) => {
                            if !def_within {
                                panic!("Cannot return outside a function");
                            }
                            for rv_rtn in returns.iter() {
                                compile_rvalue(rv_rtn, &mut called_functions);
                            }
                            def_had_return = true;
                        }
                    }
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
        lexer.analyze_item(program_item).unwrap();

        // Ok((program_item, lexer))
        Err(MypsLexerError::Dummy)
    }

    fn analyze_def(
        &mut self,
        items: Vec<Item>,
        name: String,
        arg_labels: Vec<String>,
        comment: Option<String>,
    ) -> MypsLexerResult<()> {
        let mut alias_table = HashMap::new();

        // Check that the function is undefined
        if self.functions.contains_key(&name) {
            panic!("Cannot redefine functions");
        }

        // Construct an alias table of the argument labels,
        // checking that argument labels are unique
        for arg_label in arg_labels.iter() {
            if alias_table.contains_key(arg_label) {
                panic!("Duplicate argument '{}' in user function definition", arg_label);
            } else {
                alias_table.insert(arg_label.clone(), RValueReturn::Var(arg_label.clone()));
            }
        }

        // Construct a lexer dedicated to analyzing the items of this new functions
        let def_lexer = Lexer {
            alias_table,
            functions: self.functions.clone(),
            called_functions: self.called_functions.clone(),
        };

        // Construct a block to analyze
        let fn_item = Item {
            item_inner: ItemInner::Block(Block {
                branch: Branch::Program,
                items,
            }),
            comment,
        };

        // Analyze the item
        let item = def_lexer.analyze_item(fn_item)?.unwrap();

        // Unpack the items
        let Item {
            item_inner: ItemInner::Block(Block { items, .. }),
            ..
        } = item;

        // Count the number of returns
        let return_item = items.last().unwrap();
        let Item {
            item_inner: ItemInner::Stmt(Statement::Return(returns)),
            ..
        } = return_item;
        let num_returns = returns.len();

        // Insert this new user function
        let function = Function::new_user(arg_labels, num_returns, items);
        self.functions.insert(name, function);

        Ok(())
    }

    fn get_alias(&self, k: &String) -> MypsLexerResult<RValueReturn> {
        self.alias_table
            .get(k)
            .cloned()
            .ok_or(MypsLexerError::undefined_alias(k))
    }

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
                        self.analyze_def(items, name, arg_names, comment).and(Ok(None))
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
            ItemInner::Stmt(stmt) => match stmt {
                // Statement::AssignAlias(alias, dev) => {}
                Statement::AssignValue(lvalues, rvalues) => {
                    let rvalue_kinds = rvalues
                        .into_iter()
                        .map(|rvalue| self.analyze_rvalue(rvalue))
                        .collect::<MypsLexerResult<Vec<Vec<RValueReturn>>>>()
                        .unwrap()
                        .into_iter()
                        .flat_map(|rvalue_kinds| rvalue_kinds.into_iter())
                        .collect::<Vec<RValueReturn>>();

                    let n_lvalues = lvalues.len();
                    let n_rvalues = rvalue_kinds.len();
                    if n_lvalues != n_rvalues {
                        panic!(
                            "Mismatched number of lvalues and rvalues ({} != {})",
                            n_lvalues, n_rvalues
                        );
                    }
                    unreachable!();
                }
                Statement::FunctionCall(name, args) => {
                    unreachable!("{:?}", stmt);
                }
                Statement::Return(..) => {
                    unreachable!("{:?}", stmt);
                }
            },
        }
    }

    // Returns the Ok of number of rvalues returned by this rvalue
    fn analyze_rvalue(&mut self, rvalue: RValue) -> MypsLexerResult<Vec<RValueReturn>> {
        match rvalue {
            RValue::Num(num) => {
                let rtn = match num {
                    Num::Var(k) => {
                        let alias = self.alias_table.get(&k).unwrap();
                        if let RValueReturn::Num(..) = alias {
                            alias.clone()
                        } else {
                            panic!();
                        }
                    }
                    _ => RValueReturn::Num(num),
                };
                Ok(vec![rtn])
            }
            RValue::Dev(dev) => {
                let rtn = match dev {
                    Dev::Var(k) => {
                        let alias = self.alias_table.get(&k).unwrap();
                        if let RValueReturn::Dev(..) = alias {
                            alias.clone()
                        } else {
                            panic!();
                        }
                    }
                    _ => RValueReturn::Dev(dev),
                };
                Ok(vec![rtn])
            }
            RValue::DevParam(dev, ..) | RValue::NetParam(dev, ..) | RValue::DevSlot(dev, ..) => {
                if let Dev::Var(k) = dev {
                    let alias = self.alias_table.get(k);
                    if !matches!(alias, Some(ReturnKind::Dev)) {
                        panic!();
                    }
                }
                Ok(vec![ReturnKind::Num])
            }
            // RValue::Expr(box e) => {
            //     self.analyze_expr(e)
            // }
            RValue::Func(func_call) => self.analyze_function_call(func_call),
            // RValue::Var(k) => {
            //     let return_kind = self.alias_table[k];
            //     Ok(vec![return_kind])
            // },
            _ => unreachable!(),
        }
    }

    // Reduce, validate and analyze an expression helper.
    fn analyze_expr(&mut self, expr: &Expr) -> MypsLexerResult<Vec<ReturnKind>> {
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
                // analyze_rvalue(rv, alias_table, dependencies)?;
                self.analyze_rvalue(rv).unwrap();
            }
        }
        Ok(())
    }

    fn analyze_function_call(
        &mut self,
        func_call: &FunctionCall,
    ) -> MypsLexerResult<Vec<RValueReturn>> {
        let FunctionCall { name, args } = func_call;

        let function = self.functions.get(name).unwrap();
        let (num_args_expected, return_kinds) = match function {
            Function::Builtin(FunctionBuiltin {
                num_args,
                num_returns
            }) => (*num_args, return_kinds),
            Function::User(FunctionUser {
                arg_labels,
                num_returns,
                ..
            }) => (arg_labels.len(), return_kinds),
        };

        if num_args_expected != args.len() {
            let err = MypsLexerError::func_wrong_num_args(name, num_args_expected, args.len());
            panic!("{:?}", err);
            // return Err(err);
        }

        // // Sum up the argument depths
        // let mut depth = 0;
        // for arg in args.iter() {
        //     match arg {
        //         Arg::Dev(..) => {
        //             depth += 1;
        //         },
        //         Arg::RValue(rvalue) => {
        //             depth += self.analyze_rvalue(rvalue).unwrap();
        //         },
        //     }
        // }

        Ok(return_kinds.clone())
    }

    // fn analyze_called_functions(&mut self, item: &Item) -> MypsLexerResult<()> {
    // fn test_rvalue(rvalue: &RValue, functions: &Functions) -> MypsLexerResult<()> {
    //     match rvalue {
    //         // RValue::Num(Num),
    //         // RValue::DevParam(Dev, String),
    //         // RValue::NetParam(Dev, Mode, String),
    //         // RValue::DevSlot(Dev, Int, String),
    //         RValue::Expr(box expr) => test_expr(expr, functions)?,
    //         RValue::Func(FunctionCall { name, args }) => {
    //             let func = functions
    //                 .get(name)
    //                 .ok_or(MypsLexerError::func_undefined(name))?;
    //             let arg_kinds = match func {
    //                 Function::Builtin(FunctionBuiltin { arg_kinds, ..}) => arg_kinds,
    //                 Function::User(FunctionUser { arg_kinds, .. }) => arg_kinds,
    //             };
    //             for (expected, found) in arg_kinds.iter().zip(args.iter()) {
    //                 let found = match found {
    //                     Arg::RValue(..) => ArgKind::RValue,
    //                     Arg::Dev(..) => ArgKind::Dev,
    //                 };
    //                 if &found != expected {
    //                     return Err(MypsLexerError::func_wrong_args(name, arg_kinds, args));
    //                 }
    //             }
    //         }
    //         _ => {}
    //     }
    //     Ok(())
    // }

    // fn test_expr(expr: &Expr, functions: &Functions) -> MypsLexerResult<()> {
    //     match expr {
    //         Expr::RValue(rvalue) => test_rvalue(rvalue, functions)?,
    //         Expr::Unary { rhs: box rhs, .. } => test_expr(rhs, functions)?,
    //         Expr::Binary {
    //             lhs: box lhs,
    //             rhs: box rhs,
    //             ..
    //         } => {
    //             test_expr(lhs, functions)?;
    //             test_expr(rhs, functions)?;
    //         }
    //         Expr::Ternary {
    //             cond: box cond,
    //             if_t: box if_t,
    //             if_f: box if_f,
    //         } => {
    //             test_expr(cond, functions)?;
    //             test_expr(if_t, functions)?;
    //             test_expr(if_f, functions)?;
    //         }
    //     }
    //     Ok(())
    // }

    // match &item.item_inner {
    //     ItemInner::Block(block) => {
    //         for item in block.items.iter() {
    //             self.analyze_called_functions(item)?;
    //         }
    //     }
    //     ItemInner::Stmt(stmt) => {
    //         match stmt {
    //             Statement::AssignAlias(..) => {}
    //             Statement::AssignValue(_, rvalue) => {
    //                 // test_rvalue(rvalue, &self.functions)?
    //             }
    //             Statement::FunctionCall(..) => {
    //                 // TODO
    //                 unreachable!();
    //             }
    //         }
    //     }
    // }
    // Ok(())
    // }
}
