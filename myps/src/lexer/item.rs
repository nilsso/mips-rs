use std::collections::HashSet;

use crate::ast::{Dev, LValue, RValue, Branch};
use crate::lexer::{Block, Statement};

#[derive(Debug)]
pub struct Item {
    pub(crate) dependencies: HashSet<String>,
    pub(crate) stmt: Statement,
}

impl Item {
    pub fn new(stmt: Statement, dependencies: HashSet<String>) -> Self {
        Self { stmt, dependencies }
    }

    pub fn decompose_block(self) -> (Block, HashSet<String>) {
        if let Item { stmt: Statement::Block(block), dependencies } = self {
            (block, dependencies)
        } else {
            unreachable!()
        }
    }

    pub fn push_item(&mut self, item: Item) {
        if let Item { stmt: Statement::Block(block), dependencies } = self {
            for k in item.dependencies.iter() {
                dependencies.insert(k.clone());
            }
            block.items.push(item);
        } else {
            unreachable!()
        }
    }

    pub fn new_branch(branch: Branch) -> Self {
        Self::new_block(Block::new(branch))
    }

    pub fn new_block(block: Block) -> Self {
        Self {
            stmt: Statement::Block(block),
            dependencies: HashSet::new(),
        }
    }

    pub fn new_assign_alias(k: String, dev: Dev, dependencies: HashSet<String>) -> Self {
        Self {
            stmt: Statement::AssignAlias(k, dev),
            dependencies,
        }
    }

    pub fn new_assign_value(l: LValue, r: RValue, dependencies: HashSet<String>) -> Self {
        Self {
            stmt: Statement::AssignValue(l, r),
            dependencies,
        }
    }
}

