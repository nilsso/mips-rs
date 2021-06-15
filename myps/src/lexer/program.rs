use crate::superprelude::*;

pub struct Program {
    pub(crate) program_item: Item,
    pub(crate) alias_table: AliasTable,
    pub(crate) functions: Functions,
}
