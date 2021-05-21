use pest::iterators::Pair;

use crate::Rule;
use crate::InnerUnchecked;

use super::Memory;

/// Device register node.
///
/// Comes in three flavors:
/// 1. [`DevBase`](Device::DevBase) - a base register, e.g. "d0"
/// 3. [`DevAlias`](Device::DevAlias) - an (unvalidated) alias, e.g. "x"
/// 2. [`Dev`](Device::Dev) - an indirect (recursive) register, e.g. "dr0"
#[derive(PartialEq, Debug)]
pub enum Device {
    DevBase(usize),
    DevAlias(String),
    Dev(Box<Memory>),
}

impl Device {
    /// New device register node from Pest pair.
    ///
    /// Should be called on outer most `Rule::dev` pair,
    /// so that all scenarios (base, alias, indirect) are handled.
    pub fn new(pair: Pair<Rule>) -> Self {
        match pair.as_rule() {
            Rule::dev => Device::new(pair.inner()),
            Rule::dev_base => Device::DevBase(pair.inner().as_str().parse().unwrap()),
            Rule::alias => Device::DevAlias(pair.as_str().into()),
            Rule::dev_recur => {
                let inner = pair.inner();
                match inner.as_rule() {
                    Rule::mem_recur => Device::Dev(Box::new(Memory::new(inner))),
                    Rule::dev_base => Device::new(inner),
                    _ => unreachable!(),
                }
            }
            _ => unreachable!(),
        }
    }
}
