use pest::iterators::Pair;

use crate::Rule;
use crate::InnerUnchecked;

/// Memory register node.
///
/// Comes in three flavors:
/// 1. [`MemBase`](Memory::MemBase) - a base register, e.g. "r0"
/// 3. [`MemAlias`](Memory::MemAlias) - an (unvalidated) alias, e.g. "x"
/// 2. [`Mem`](Memory::Mem) - an indirect (recursive) register, e.g. "rr0"
#[derive(PartialEq, Debug)]
pub enum Memory {
    MemBase(usize),
    MemAlias(String),
    Mem(Box<Memory>),
}

impl Memory {
    /// New memory register node from Pest pair.
    ///
    /// Should be called on outer most `Rule::mem` pair,
    /// so that all scenarios (base, alias, indirect) are handled.
    pub fn new(pair: Pair<Rule>) -> Self {
        match pair.as_rule() {
            Rule::mem => Memory::new(pair.inner()),
            Rule::mem_base => Memory::MemBase(pair.inner().as_str().parse().unwrap()),
            Rule::alias => Memory::MemAlias(pair.as_str().into()),
            Rule::mem_recur => {
                let inner = pair.inner();
                match inner.as_rule() {
                    Rule::mem_recur => Memory::Mem(Box::new(Memory::new(inner))),
                    Rule::mem_base => Memory::new(inner),
                    _ => unreachable!(),
                }
            }
            _ => unreachable!(),
        }
    }
}
