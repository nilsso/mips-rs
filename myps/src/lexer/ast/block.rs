use std::collections::HashSet;

use crate::superprelude::*;

#[derive(Clone, Debug)]
pub struct Block {
    pub(crate) branch: Branch,
    pub(crate) items: Vec<Item>,
}

impl Block {
    pub fn new(branch: Branch, items: Vec<Item>) -> Self {
        Self { branch, items }
    }

    pub fn program() -> Self {
        Self::new(Branch::Program, Vec::new())
    }

    pub fn analyze(
        &self,
        aliases: &mut HashSet<String>,
        called_functions: &mut HashSet<String>,
    ) -> MypsLexerResult<()> {
        let Block { branch, items } = self;

        for item in items.iter() {
            item.analyze(aliases, called_functions)?;
        }
        branch.analyze(aliases)
    }
}
