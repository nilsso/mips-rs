use std::{fmt, fmt::Display};

use crate::ast::Statement;
use crate::ast_includes::*;

#[derive(Debug)]
pub struct Line {
    pub(crate) indent: usize,
    pub(crate) stmt: Statement,
    pub(crate) comment: Option<String>,
}

impl<'i> AstNode<'i, Rule, MypsParser, MypsParserError> for Line {
    type Output = Option<Self>;

    const RULE: Rule = Rule::line;

    fn try_from_pair(pair: Pair<Rule>) -> MypsParserResult<Option<Self>> {
        fn line_to_indent_item(
            pair: Pair<Rule>,
        ) -> MypsParserResult<Option<(usize, Pair<Rule>, Option<String>)>> {
            match pair.as_rule() {
                Rule::line => {
                    let mut indent = 0;
                    let mut stmt = None;
                    let mut comment = None;
                    for item in pair.into_inner() {
                        match item.as_rule() {
                            Rule::indent => indent += 1,
                            Rule::empty => return Ok(None),
                            Rule::stmt => stmt = Some(item),
                            Rule::comment => comment = Some(item.as_str().into()),
                            // Rule::stmt => return Ok(Some((indent, item))),
                            _ => {
                                return Err(MypsParserError::wrong_rule(
                                    "an indent or statement",
                                    item,
                                ))
                            }
                        }
                    }
                    Ok(Some((indent, stmt.unwrap(), comment)))
                }
                _ => Err(MypsParserError::wrong_rule("a (indented) statement", pair)),
            }
        }

        // #[rustfmt::skip]
        match pair.as_rule() {
            Rule::line => {
                if let Some((indent, stmt_pair, comment)) = line_to_indent_item(pair)? {
                    let stmt = stmt_pair.first_inner()?.try_into_ast()?;
                    Ok(Some(Self {
                        indent,
                        stmt,
                        comment,
                    }))
                } else {
                    Ok(None)
                }
            }
            Rule::EOI => Ok(None),
            _ => Err(MypsParserError::wrong_rule("a line", pair)),
        }
    }
}

impl Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for _ in 0..self.indent {
            write!(f, "    ")?;
        }
        write!(f, "{}", self.stmt)
    }
}

// #[derive(Debug)]
// pub enum Item {
//     Branch(Branch),
//     Statement(Statement),
// }

// pub fn build_branches(pairs: Pairs<Rule>) -> MypsParserResult<Branch> {
//     let mut branches = vec![Branch::new(BranchKind::Program)];
//     let mut indent_stack = vec![0_usize];
//     let mut expect_indent = false;

//     for (item_indent, item) in pairs.filter_map(line_into_item) {
//         // Handle indentation
//         let curr_indent = *indent_stack.last().unwrap();
//         if expect_indent {
//             // Expecting increase in indent because previous line was a branch marker
//             if item_indent <= curr_indent {
//                 panic!("Expected indent");
//             } else {
//                 expect_indent = false;
//                 // Push this new indent level
//                 indent_stack.push(item_indent);
//             }
//         } else {
//             if item_indent < curr_indent {
//                 // Drop in indent level means the end of a branch
//                 while item_indent < *indent_stack.last().unwrap() {
//                     // Pop off stack levels until the item level is at or above the stack level
//                     let branch = branches.pop().unwrap();
//                     let head = branches.last_mut().unwrap();
//                     head.push(Item::Branch(branch));
//                     indent_stack.pop();
//                 }
//                 if item_indent != *indent_stack.last().unwrap() {
//                     // If now the item and stack levels are different,
//                     // this item level was never on the stack before
//                     panic!("Incorrect indent");
//                 }
//             }
//         }
//         // Handle item
//         let rule = item.as_rule();
//         match rule {
//             Rule::branch => {
//                 // Push a new branch and expect the next line to be indented
//                 let branch_kind = item.try_into_ast()?;
//                 branches.push(Branch::new(branch_kind));
//                 expect_indent = true;
//             },
//             Rule::assign_alias | Rule::assign_value => {
//                 let stmt = item.try_into_ast()?;
//                 let head = branches.last_mut().unwrap();
//                 head.push(Item::Statement(stmt));
//             }
//             _ => return Err(MypsParserError::wrong_rule("a statement or branch", item)),
//         }
//     }
//     // Reduce whats left into branches of items
//     while branches.len() > 1 {
//         let branch = branches.pop().unwrap();
//         let head = branches.last_mut().unwrap();
//         head.push(Item::Branch(branch));
//     }
//     Ok(branches.pop().unwrap())
// }
