//! MIPS abstract syntax tree

use std::{fmt, fmt::Display};

use pest::iterators::Pair;

use crate::Rule;

#[derive(Clone, PartialEq, Debug)]
pub enum Register {
    Memory(Box<Register>),
    Device(Box<Register>),
    Alias(String),
}

impl Register {
    pub fn new(pair: Pair<Rule>) -> Self {
        let rule = pair.as_rule();
        match rule {
            Rule::reg => {
                let inner = pair.into_inner().next().unwrap();
                Register::new(inner)
            }
            Rule::reg_mem_base => Register::Alias(pair.as_str().into()),
            Rule::reg_mem => {
                let inner = pair.into_inner().next().unwrap();
                let boxed = match inner.as_rule() {
                    Rule::reg_mem_base => Box::new(Register::Alias(inner.as_str().into())),
                    Rule::reg_mem => Box::new(Register::new(inner)),
                    _ => unreachable!(),
                };
                Register::Memory(boxed)
            }
            Rule::reg_dev => {
                let inner = pair.into_inner().next().unwrap();
                let boxed = match inner.as_rule() {
                    Rule::reg_dev_base => Box::new(Register::Alias(inner.as_str().into())),
                    Rule::reg_mem => Box::new(Register::new(inner)),
                    _ => unreachable!(),
                };
                Register::Device(boxed)
            }
            Rule::reg_alias => {
                let inner = pair.into_inner().next().unwrap();
                let t = inner.as_str();
                Register::Alias(t.to_string())
            }
            _ => unreachable!(),
        }
    }
}

impl Display for Register {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Register::Memory(_) => write!(fmt, "Memory"),
            &Register::Device(_) => write!(fmt, "Device"),
            &Register::Alias(_) => write!(fmt, "Alias"),
        }
    }
}

/// Value node
/// Represents a literal float value or the value at a register.
#[derive(Clone, PartialEq, Debug)]
pub enum Value {
    Literal(f32),
    Register(Register),
}

impl Value {
    pub fn new(pair: Pair<Rule>) -> Self {
        let inner = pair.into_inner().next().unwrap();
        let rule = inner.as_rule();
        match rule {
            Rule::num => Value::Literal(inner.as_str().parse().unwrap()),
            Rule::reg => Value::Register(Register::new(inner)),
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum Node {
    Null,
    Program(Vec<Node>),
    Fn(String, Vec<Node>),
    Register(Register),
    Value(Value),
    Label(String),
}

impl Node {
    pub fn new(pair: Pair<Rule>) -> Self {
        match pair.as_rule() {
            Rule::program => {
                Node::Program(pair.into_inner().map(|inner| Self::new(inner)).collect())
            }
            Rule::fun => {
                let mut inner = pair.into_inner();
                let name = inner.next().unwrap().as_str().to_string();
                Node::Fn(name, inner.map(|pair| Self::new(pair)).collect())
            }
            Rule::reg => {
                let inner = pair.into_inner().next().unwrap();
                Node::Register(Register::new(inner))
            }
            Rule::value => Node::Value(Value::new(pair)),
            _ => Node::Null,
        }
    }

    pub fn as_register(self) -> Option<Register> {
        match self {
            Node::Register(r) => Some(r),
            _ => None,
        }
    }
}
