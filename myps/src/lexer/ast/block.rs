use crate::superprelude::*;

#[derive(Clone, Debug)]
pub struct Block {
    pub(crate) branch: Branch,
    pub(crate) items: Vec<Item>,
}

impl Block {
    pub fn new(branch: Branch) -> Self {
        Self {
            branch,
            items: Vec::new(),
        }
    }

    pub fn program() -> Self {
        Self::new(Branch::Program)
    }
}
