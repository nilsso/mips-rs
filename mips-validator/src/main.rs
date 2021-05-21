#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_braces)]

use std::collections::HashMap;

// use mips_parser::prelude::*;
// use mips_state::prelude::*;

#[derive(Copy, Clone, Debug)]
enum StateIndex {
    Memory(usize),
    Device(usize),
}

impl StateIndex {
    pub fn as_memory(self) -> Option<usize> {
        match self {
            StateIndex::Memory(i) => Some(i),
            _ => None,
        }
    }

    pub fn as_device(self) -> Option<usize> {
        match self {
            StateIndex::Device(i) => Some(i),
            _ => None,
        }
    }
}

#[derive(Clone, Debug)]
struct State {
    pub mem: Vec<f32>,
    pub aliases: HashMap<String, StateIndex>,
}

impl State {
    pub fn new(mem_size: usize) -> Self {
        Self {
            mem: vec![0_f32; mem_size],
            aliases: HashMap::new(),
        }
    }
}

#[derive(Clone, Debug)]
enum Register {
    Memory(Box<Register>),
    Device(Box<Register>),
    Literal(usize),
    Alias(String),
}

impl Register {
    fn reduce(self, state: &State) -> Result<StateIndex, ()> {
        println!("{:?}", self);
        match self {
            Register::Memory(r) => match *r {
                r @ Register::Memory(_) => {
                    let a = r.reduce(state)?;
                    a.as_memory()
                        .and_then(|i| state.mem.get(i).copied())
                        .map(|j| StateIndex::Memory(j as usize))
                        .ok_or(())
                }
                Register::Literal(i) => Ok(StateIndex::Memory(i)),
                _ => unreachable!(),
            },
            Register::Device(r) => match *r {
                r @ Register::Memory(_) => {
                    let a = r.reduce(state)?;
                    a.as_memory()
                        .and_then(|i| state.mem.get(i).copied())
                        .map(|j| StateIndex::Device(j as usize))
                        .ok_or(())
                }
                Register::Literal(i) => Ok(StateIndex::Device(i)),
                _ => unreachable!(),
            },
            _ => Err(()),
        }
    }
}

struct RegisterBuilder {
    head: Box<Register>,
}

impl RegisterBuilder {
    pub fn new(i: usize) -> Self {
        Self {
            head: Box::new(Register::Literal(i)),
        }
    }

    pub fn mem(mut self) -> Self {
        if let Register::Device(_) = *self.head {
            panic!();
        }
        self.head = Box::new(Register::Memory(self.head));
        self
    }

    pub fn dev(mut self) -> Self {
        if let Register::Device(_) = *self.head {
            panic!();
        }
        self.head = Box::new(Register::Device(self.head));
        self
    }

    pub fn build(self) -> Register {
        *self.head
    }
}

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct DeviceKind {
    name: String,
}

use ron::de::from_reader as ron_from_reader;
use ron::de::from_str as ron_from_str;
use std::path::PathBuf;

fn device_kinds<P: Into<PathBuf>>(p: P) -> Result<HashMap<String, DeviceKind>, ()> {
    let file = std::fs::File::open(p.into()).or(Err(()))?;
    let device_kinds: Vec<DeviceKind> = ron_from_reader(file).or(Err(()))?;
    Ok(device_kinds
        .into_iter()
        .map(|dk| (dk.name.replace(" ", "_"), dk))
        .collect())
}

fn main() -> Result<(), ()> {
    let mut state = State::new(16);
    // for (i, v) in state.mem.iter_mut().enumerate() {
    // *v = (i + 1) as f32;
    // }
    let dks = device_kinds("./stationeers/devices.ron").unwrap();

    use serde_closure::{traits::Fn, Fn};

    let sin = Fn!(|a: f32| -> f32 { a.sin() });
    let s = serde_json::to_string(&sin);
    println!("{:#?}", s);
    // let s: serde_closure::structs::Fn<fn(f32) -> f32>;
    // let s = ron::ser::to_string(&sin).unwrap();
    // println!("{:#?}", s);
    // println!("{:#?}", serde_json::from_str::<serde_closure::structs::Fn>(&s));
    // let sin: fn(f32) -> f32 = ron::de::from_str(s);
    // let serializer = ron::ser::Serializer::new();
    // Serialize::serialize(sin, ron::ser::Serializer);
    // Deserialize::deserialize(sin).unwrap();
    // ron::ser::to_string(&sin);

    // let r = RegisterBuilder::new(0).mem().dev().build();
    // println!("{:#?}", r);
    // println!("{:#?}", r.reduce(&state));

    // let mut state = MipsState::default();
    // let unparsed_file = std::fs::read_to_string("./mips-validator/example-scripts/test.mips").expect("Unable to read file");
    // let pairs = MipsParser::parse(Rule::file, &unparsed_file).expect("");
    // println!("{:#?}", pairs);
    // let program = Node::new(pairs.into_iter().next().unwrap());
    // println!("{:#?}", program);
    // state.validate(&program)?;
    Ok(())
}
