#![allow(unused_variables)]
use std::collections::HashSet;

use crate::superprelude::*;

mod alias;
pub mod ast;
mod error;
// pub mod lex;

pub use alias::{Alias, AliasTable, DevAlias};
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
                        return Err(MypsLexerError::wrong_rule(
                            "an indent, item (branch or statement), or comment",
                            pair,
                        ));
                    }
                }
            }
            if let Some(item_pair) = item_pair_opt {
                Ok(Some((indent, item_pair, comment_opt)))
            } else {
                Err(AstError::InsufficientPairs).map_err(MypsLexerError::AstError)
            }
        }
        Rule::EOI => Ok(None),
        _ => Err(MypsLexerError::wrong_rule("a line", line_pair))
    }
}

/// Convert MYPS parser output pairs to an abstract syntax tree.
pub fn lex(mut pairs: Pairs<Rule>) -> MypsLexerResult<(Item, AliasTable)> {
    let program_pair = pairs.next().unwrap();
    if pairs.next().is_some() {
        panic!();
    }

    // Parse and collect line pairs into (indent, item_pair, comment) tuples
    let items = program_pair
        .into_inner()
        .map(parse_line_pair)
        .collect::<MypsLexerResult<Vec<Option<(usize, Pair<Rule>, Option<String>)>>>>()?
        .into_iter()
        .flatten()
        .collect::<Vec<(usize, Pair<Rule>, Option<String>)>>();

    let mut alias_table = AliasTable::new();
    let mut blocks = vec![(
        Block::program(),
        HashSet::<String>::new(),
        Option::<String>::None,
    )];
    let mut indent_stack = vec![0_usize];
    let mut curr_indent = 0_usize;
    let mut expect_indent = false;

    // for Line { indent, stmt, comment } in program_ast.lines.into_iter() {
    for (indent, item_pair, comment_opt) in items.into_iter() {
        // Handle indentation
        if expect_indent {
            // Expecting increase in indent because previous line was a branch marker
            if indent <= curr_indent {
                return Err(MypsLexerError::expected_indent(curr_indent + 1));
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
                    let (block, block_deps, comment_opt) = blocks.pop().unwrap();
                    let (head, head_deps, _) = blocks.last_mut().unwrap();
                    for dep in block_deps.iter() {
                        head_deps.insert(dep.clone());
                    }
                    head.items.push(Item::block(block, block_deps, comment_opt));
                    indent_stack.pop();
                }
                if indent != *indent_stack.last().unwrap() {
                    // If now the item and stack levels are different,
                    // this item level was never on the stack before
                    return Err(MypsLexerError::wrong_indent(curr_indent, indent));
                }
            }
        }
        // Handle item
        match item_pair.as_rule() {
            Rule::branch => {
                let branch = item_pair.try_into_ast()?;
                let mut dependencies = HashSet::new();
                match &branch {
                    Branch::If(expr) | Branch::Elif(expr) | Branch::While(expr) => {
                        analyze_expr_helper(&expr, &alias_table, &mut dependencies)?;
                    }
                    Branch::For(_, s, e) => {
                        if let Int::Var(s_var) = s {
                            dependencies.insert(s_var.clone());
                        }
                        if let Int::Var(e_var) = e {
                            dependencies.insert(e_var.clone());
                        }
                    }
                    Branch::Def(name) => {
                        // TODO: Add function to alias_table
                    }
                    _ => {}
                }
                let block = Block::new(branch);
                blocks.push((block, dependencies, comment_opt));
            }
            Rule::stmt => {
                let stmt = item_pair.try_into_ast()?;
                let mut dependencies = HashSet::new();
                match &stmt {
                    Statement::AssignAlias(a_var, dev) => {
                        let alias = Alias::Dev(match dev {
                            Dev::Lit(b, i) => DevAlias::Lit(*b, *i),
                            Dev::Indexed(id) => DevAlias::Indexed(id.clone()),
                            Dev::Batch(hash) => DevAlias::Batch(*hash),
                            Dev::Var(var) => {
                                dependencies.insert(var.clone());
                                DevAlias::Var(var.clone())
                            }
                        });
                        alias_table.insert(a_var.clone(), alias);
                    }
                    Statement::AssignValue(l_val, r_val) => {
                        analyze_rvalue_helper(&r_val, &alias_table, &mut dependencies)?;
                        if let LValue::Var(var) = l_val {
                            alias_table.insert(var.clone(), Alias::Var);
                        }
                    }
                    Statement::FunctionCall(name) => {
                    },
                }
                let (head, head_deps, _) = blocks.last_mut().unwrap();
                head.items
                    .push(Item::statement(stmt, dependencies, comment_opt));
            }
            _ => {
                return Err(MypsLexerError::wrong_rule(
                    "a block or statement",
                    item_pair,
                ));
            }
        }
        /*
        match item.stmt {
            Statement::Branch(branch) => {
                // TODO: Analyze expressions HERE

                // Push a new branch and expect the next line to be indented
                blocks.push(item);
                expect_indent = true;
            }
            Statement::AssignAlias(a, d) => {
                let mut dependencies = HashSet::new();
                let alias = match &d {
                    Dev::Lit(b, i) => Alias::Dev(DevAlias::Lit(*b, *i)),
                    Dev::Batch(hash) => Alias::Dev(DevAlias::Batch(*hash)),
                    Dev::Var(k) => {
                        dependencies.insert(k.clone());
                        Alias::Var
                    }
                };
                alias_table.insert(a.clone(), alias);
                let head = blocks.last_mut().unwrap();
                head.push_item(item);
            }
            Statement::AssignValue(l, r) => {
                let dependencies = analyze_rvalue(&r, &alias_table)?;
                match &l {
                    LValue::Var(k) => {
                        if let RValue::Num(Num::Lit(n)) = &r {
                            alias_table.insert(k.clone(), Alias::Num(*n));
                        } else {
                            alias_table.insert(k.clone(), Alias::Var);
                        }
                    }
                    _ => unreachable!("{:?}", l),
                }
                let head = blocks.last_mut().unwrap();
                head.push_item(item);
            }
            _ => {
                unreachable!()
                // Push item to head block.
                // let head = blocks.last_mut().unwrap();
                // head.push(Item::try_from_stmt(stmt)?);
            }
        }
        */
    }
    // Reduce whats left into branches of items
    while blocks.len() > 1 {
        let (block, block_deps, comment_opt) = blocks.pop().unwrap();
        let (head, head_deps, _) = blocks.last_mut().unwrap();
        for dep in block_deps.iter() {
            head_deps.insert(dep.clone());
        }
        head.items.push(Item::block(block, block_deps, comment_opt));
    }
    let (head, head_deps, comment_opt) = blocks.pop().unwrap();
    Ok((Item::block(head, head_deps, comment_opt), alias_table))
}

/// Validate an r-value and give its variable dependencies.
///
/// If an r-value is a var variant, validates whether those aliases exist, and replaces them if
/// a constant value has been calculated previously.
// pub fn analyze_rvalue(
//     rvalue: &RValue,
//     alias_table: &AliasTable,
// ) -> MypsLexerResult<HashSet<String>> {
//     // Internal helper
//     let mut dependencies = HashSet::new();
//     let rvalue = analyze_rvalue_helper(rvalue, alias_table, &mut dependencies)?;
//     Ok(dependencies)
// }

fn analyze_rvalue_helper(
    rvalue: &RValue,
    alias_table: &AliasTable,
    dependencies: &mut HashSet<String>,
) -> MypsLexerResult<()> {
    match rvalue {
        RValue::Num(num) => {
            if let Num::Var(k) = num {
                alias_table.validate_num(k)?;
                dependencies.insert(k.clone());
            }
        }
        RValue::NetParam(hash, ..) => {
            if let Int::Var(k) = hash {
                alias_table.validate_num(k)?;
                dependencies.insert(k.clone());
            }
        }
        RValue::DevParam(dev, ..) => {
            if let Dev::Var(k) = dev {
                alias_table.validate_dev(k)?;
                dependencies.insert(k.clone());
            }
        }
        RValue::Expr(box Expr::RValue(rv)) => analyze_rvalue_helper(rv, alias_table, dependencies)?,
        RValue::Expr(box e) => {
            analyze_expr_helper(e, alias_table, dependencies)?;
        }
    }
    Ok(())
}

// Reduce, validate and analyze an expression helper.
fn analyze_expr_helper(
    expr: &Expr,
    alias_table: &AliasTable,
    dependencies: &mut HashSet<String>,
) -> MypsLexerResult<()> {
    match expr {
        Expr::Unary { op, box rhs } => {
            analyze_expr_helper(rhs, alias_table, dependencies)?;
        }
        Expr::Binary {
            op,
            box lhs,
            box rhs,
        } => {
            analyze_expr_helper(lhs, alias_table, dependencies)?;
            analyze_expr_helper(rhs, alias_table, dependencies)?;
        }
        Expr::Ternary {
            box cond,
            box if_t,
            box if_f,
        } => {
            analyze_expr_helper(cond, alias_table, dependencies)?;
            analyze_expr_helper(if_t, alias_table, dependencies)?;
            analyze_expr_helper(if_f, alias_table, dependencies)?;
        }
        Expr::RValue(rv) => {
            analyze_rvalue_helper(rv, alias_table, dependencies)?;
        }
    }
    Ok(())
}
