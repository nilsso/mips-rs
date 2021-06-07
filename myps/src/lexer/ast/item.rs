use std::collections::HashSet;

use crate::superprelude::*;

#[derive(Debug)]
pub enum ItemInner {
    Stmt(Statement),
    Block(Block),
}

#[derive(Debug)]
pub struct Item {
    pub(crate) dependencies: HashSet<String>,
    pub(crate) comment: Option<String>,
    pub(crate) item_inner: ItemInner,
}

impl Item {
    pub fn new(
        item_inner: ItemInner,
        dependencies: HashSet<String>,
        comment: Option<String>,
    ) -> Self {
        Self {
            dependencies,
            comment,
            item_inner,
        }
    }
    pub fn block(
        block: Block,
        dependencies: HashSet<String>,
        comment: Option<String>
    ) -> Self {
        Self::new(ItemInner::Block(block), dependencies, comment)
    }

    pub fn statement(
        stmt: Statement,
        dependencies: HashSet<String>,
        comment: Option<String>,
    ) -> Self {
        Self::new(ItemInner::Stmt(stmt), dependencies, comment)
    }
}

// impl<'i> AstNode<'i, Rule, MypsParser, MypsLexerError> for Item {
//     type Output = Option<(Self, usize)>;

//     const RULE: Rule = Rule::line;

//     fn try_from_pair(pair: Pair<Rule>) -> MypsLexerResult<Self::Output> {

//         match pair.as_rule() {
//             Rule::line => {
//                 let mut indent = 0;
//                 let mut stmt = None;
//                 let mut comment = None;
//                 for item in pair.into_inner() {
//                     match item.as_rule() {
//                         Rule::indent => {
//                             indent += 1;
//                         },
//                         Rule::stmt => {
//                             stmt = Some(item.try_into_ast()?);
//                         },
//                         Rule::comment => {
//                             comment = Some(item.as_str().into());
//                         },
//                         Rule::empty => {
//                             return Ok(None);
//                         },
//                         _ => {
//                             return Err(MypsLexerError::wrong_rule(
//                                 "an indent, statement, or comment",
//                                 item,
//                             ))
//                         }
//                     }
//                 }
//                 let item = Item {
//                     dependencies: HashSet::new(),
//                     stmt: stmt.unwrap(),
//                     comment,
//                 };
//                 Ok(Some((item, indent)))
//             }
//             _ => Err(MypsLexerError::wrong_rule("a line", pair)),
//         }
//     }
// }
