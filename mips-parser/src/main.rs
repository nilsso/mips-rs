#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

// use std::collections::HashMap;

use mips_parser::prelude::*;

// #[derive(Copy, Clone, Debug)]
// enum StateIndex {
//     Memory(usize),
//     Device(usize),
// }

// impl StateIndex {
//     pub fn as_memory(self) -> Option<usize> {
//         match self {
//             StateIndex::Memory(i) => Some(i),
//             _ => None,
//         }
//     }

//     pub fn as_device(self) -> Option<usize> {
//         match self {
//             StateIndex::Device(i) => Some(i),
//             _ => None,
//         }
//     }
// }

// #[derive(Clone, Debug)]
// struct State {
//     pub mem: Vec<f32>,
//     pub aliases: HashMap<String, StateIndex>,
// }

// impl State {
//     pub fn new(mem_size: usize) -> Self {
//         Self {
//             mem: vec![0_f32; mem_size],
//             aliases: HashMap::new(),
//         }
//     }

//     pub fn reduce_register(&self, r: Register) -> Result<StateIndex, ()> {
//         match r {
//             Register::RegMem(r) => match *r {
//                 r @ Register::RegMem(_) => {
//                     let a = self.reduce_register(r)?;
//                     a.as_memory()
//                         .and_then(|i| self.mem.get(i).copied())
//                         .map(|j| StateIndex::Memory(j as usize))
//                         .ok_or(())
//                 }
//                 Register::RegLit(i) => Ok(StateIndex::Memory(i)),
//                 _ => unreachable!(),
//             },
//             //         Register::Device(r) => match *r {
//             //             r @ Register::Memory(_) => {
//             //                 let a = self.reduce_register(rlf)?;
//             //                 a.as_memory()
//             //                     .and_then(|i| self.mem.get(i).copied())
//             //                     .map(|j| StateIndex::Device(j as usize))
//             //                     .ok_or(())
//             //             }
//             //             Register::Literal(i) => Ok(StateIndex::Device(i)),
//             //             _ => unreachable!(),
//             //         },
//             _ => Err(()),
//         }
//     }
// }

fn main() {
    let path = "./example-scripts/solar.mips";
    let program = build_ast_from_path(path).unwrap();
    // for expr in program.iter() {
        // println!("{:?}", expr);
    // }
}
