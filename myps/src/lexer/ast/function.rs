use crate::superprelude::*;

#[derive(Clone, Debug)]
pub struct Function {
    items: Vec<Item>,
}

impl Function {
    pub fn new(items: Vec<Item>) -> Self {
        Self { items }
    }
}
