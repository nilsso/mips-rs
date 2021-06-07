#![allow(unused_variables)]

use std::collections::HashSet;

use crate::superprelude::*;

pub fn lex(pairs: Pairs) -> MypsLexerResult<(Item, AliasTable)> {
    let mut alias_table = AliasTable::new();
    let mut blocks = vec![Item::new_block(Block::program(), None)];
    let mut indent_stack = vec![0_usize];
    let mut expect_indent = false;

    // for Line { indent, stmt, comment } in program_ast.lines.into_iter() {
    for pair in pairs {
        // Handle indentation
        let curr_indent = *indent_stack.last().unwrap();
        if expect_indent {
            // Expecting increase in indent because previous line was a branch marker
            if indent <= curr_indent {
                panic!("Expected indent");
            } else {
                // Push this new indent level
                indent_stack.push(indent);
                expect_indent = false;
            }
        } else {
            if indent < curr_indent {
                // Drop in indent level means the end of a branch
                while indent < curr_indent {
                    // Pop off stack levels until the item level is at or above the stack level
                    let block = blocks.pop().unwrap();
                    let head = blocks.last_mut().unwrap();
                    head.push_item(block);
                    indent_stack.pop();
                }
                if indent != curr_indent {
                    // If now the item and stack levels are different,
                    // this item level was never on the stack before
                    panic!("Incorrect indent");
                }
            }
        }
        // Handle item
        // TODO: Validate and reduce
        match stmt {
            AstStatement::Branch(branch) => {
                // TODO: Analyze expressions HERE

                // Push a new branch and expect the next line to be indented
                blocks.push(Item::new_branch(branch, comment));
                expect_indent = true;
            }
            AstStatement::AssignAlias(a, d) => {
                let mut dependencies = HashSet::new();
                let alias = match &d {
                    Dev::Lit(b, i) => Alias::Dev(DevAlias::Lit(*b, *i)),
                    Dev::Batch(hash) => Alias::Dev(DevAlias::Batch(*hash)),
                    Dev::Var(k) => {
                        dependencies.insert(k.clone());
                        Alias::Var
                    },
                };
                alias_table.insert(a.clone(), alias);
                let head = blocks.last_mut().unwrap();
                head.push_item(Item::new_assign_alias(a, d, dependencies, comment));
            },
            AstStatement::AssignValue(l, r) => {
                let dependencies = analyze_rvalue(&r, &alias_table)?;
                match &l {
                    LValue::Var(k) => {
                        if let RValue::Num(Num::Lit(n)) = &r {
                            alias_table.insert(k.clone(), Alias::Num(*n));
                        } else {
                            alias_table.insert(k.clone(), Alias::Var);
                        }
                    },
                    _ => unreachable!("{:?}", l),
                }
                let head = blocks.last_mut().unwrap();
                head.push_item(Item::new_assign_value(l, r, dependencies, comment));
            }
            // _ => {
            //     unreachable!()
            //     // Push item to head block.
            //     // let head = blocks.last_mut().unwrap();
            //     // head.push(Item::try_from_stmt(stmt)?);
            // }
        }
    }
    // Reduce whats left into branches of items
    while blocks.len() > 1 {
        let block = blocks.pop().unwrap();
        let head = blocks.last_mut().unwrap();
        for k in block.dependencies.iter() {
            head.dependencies.insert(k.clone());
        }
        head.push_item(block);
    }
    Ok((blocks.pop().unwrap(), alias_table))
}

/// Validate an r-value and give its variable dependencies.
///
/// If an r-value is a var variant, validates whether those aliases exist, and replaces them if
/// a constant value has been calculated previously.
pub fn analyze_rvalue(
    rvalue: &RValue,
    alias_table: &AliasTable,
) -> MypsLexerResult<HashSet<String>> {
    // Internal helper
    let mut dependencies = HashSet::new();
    let rvalue = analyze_rvalue_helper(rvalue, alias_table, &mut dependencies)?;
    Ok(dependencies)
}

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
