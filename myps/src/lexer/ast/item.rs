use std::collections::HashSet;

use crate::superprelude::*;

#[derive(Clone, Debug)]
pub enum ItemInner {
    Stmt(Statement),
    Block(Block),
}

impl ItemInner {
    pub fn is_if(&self) -> bool {
        match self {
            Self::Block(block) if block.branch.is_if() => true,
            _ => false,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Item {
    pub(crate) comment: Option<String>,
    pub(crate) item_inner: ItemInner,
}

impl Item {
    pub fn new(
        item_inner: ItemInner,
        comment: Option<String>,
    ) -> Self {
        Self {
            comment,
            item_inner,
        }
    }

    pub fn is_if(&self) -> bool {
        self.item_inner.is_if()
    }

    pub fn if_elif_else_index(&self) -> Option<usize> {
        match self.item_inner {
            ItemInner::Block(Block { branch: Branch::If(id, ..), .. }) => Some(id),
            ItemInner::Block(Block { branch: Branch::Elif(id, ..), .. }) => Some(id),
            ItemInner::Block(Block { branch: Branch::Else(id, ..), .. }) => Some(id),
            _ => None,
        }
    }

    pub fn block(
        block: Block,
        comment: Option<String>
    ) -> Self {
        Self::new(ItemInner::Block(block), comment)
    }

    pub fn statement(
        stmt: Statement,
        comment: Option<String>,
    ) -> Self {
        Self::new(ItemInner::Stmt(stmt), comment)
    }

    pub fn analyze(&self, aliases: &mut HashSet<String>, called_functions: &mut HashSet<String>) -> MypsLexerResult<()> {
        let Item { item_inner, .. } = self;

        match item_inner {
            ItemInner::Block(block) => block.analyze(aliases, called_functions),
            ItemInner::Stmt(stmt) => stmt.analyze(aliases, called_functions),
        }
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
//                     stmt: stmt.unwrap(),
//                     comment,
//                 };
//                 Ok(Some((item, indent)))
//             }
//             _ => Err(MypsLexerError::wrong_rule("a line", pair)),
//         }
//     }
// }
