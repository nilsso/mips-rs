use super::memory::Memory;

use pest::iterators::Pair;

use crate::Rule;

/// Value node.
///
/// Values in MIPS expressions can be floating-point literals or those stored in state memory.
/// The former is encapsulated in [`ValLit(f32)`](Value::ValLit),
/// while the later involves reducing a [`Memory`] node to a `StateIndex::Memory(i)`
/// with which the ith value from state memory is obtained.
#[derive(PartialEq, Debug)]
pub enum Value {
    ValLit(f32),
    ValMem(Memory),
}

impl Value {
    pub fn new(pair: Pair<Rule>) -> Self {
        match pair.as_rule() {
            Rule::value => {
                let inner = pair.into_inner().next().unwrap();
                Value::new(inner)
            }
            Rule::num => {
                let x = pair.as_str().parse().unwrap();
                Value::ValLit(x)
            },
            Rule::mem => {
                Value::ValMem(Memory::new(pair))
            },
            _ => unreachable!(),
        }
    }
}

