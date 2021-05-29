#![allow(unused_macros)]
use std::fs::File;

use ron::de::from_reader;

use mips_parser::ast::{nodes::Program, Node};
use mips_simulator::prelude::*;

macro_rules! setup {
    ($s:ident) => {{
        let program = Program::try_from_str(&$s).unwrap();
        let state = ICState::default();
        let sim = ICSimulator::new(state, program);

        const DEVICE_KINDS: &'static str = "./tests/device-kinds.ron";
        let file = File::open(DEVICE_KINDS).unwrap();
        let kinds: DeviceKinds = from_reader(file).unwrap();

        (sim, kinds)
    }};
}

macro_rules! run_until_finished {
    ($sim:ident) => {{
        let mut i = 0;
        while !$sim.is_finished() {
            $sim.step().unwrap();
            i += 1;
            if i >= 1000 {
                panic!("Infinite loop?");
            }
        }
    }};
}

macro_rules! assert_alias {
    ($state:expr, $k:literal, $a: expr) => {
        let a = $state.get_alias(&$k.into()).unwrap();
        assert_eq!(a, &$a);
    };
}

macro_rules! assert_mem {
    ($state:expr, $i:literal, $v:literal) => {
        let v = $state.get_mem($i).unwrap();
        assert_eq!(v, &$v);
    };
}

#[test]
fn simulate_network_batch_ops() {
    const PROGRAM: &'static str = &"\
alias x r2
main:
add x 1 x
sb -851746783 Setting x # hash = Logic memory
blt x 5 main
";
    let (mut sim, kinds) = setup!(PROGRAM);
    // Add three LogicMemory devices to the network
    let kind = kinds.get("LogicMemory").unwrap();
    for _ in 0..3 {
        let dev = kind.make();
        sim.state.dev_network_add(dev);
    }

    run_until_finished!(sim);
    const LOGIC_DATA: i64 = -851746783;
    let read = |mode| sim.state.dev_network_read(LOGIC_DATA, &"Setting", mode);

    // Should be on the 6th line (a.k.a. index 5)
    assert_eq!(sim.next_line_index(), 5);
    // x should point directly to r2
    assert_alias!(sim.state, "x", AliasKind::MemId(2));
    // x (r0) should be 5
    assert_mem!(sim.state, 2, 5.0);
    // LogicMemory(Setting) average should be (3 * 5) / 3 = 5
    assert_eq!(read(0.0).unwrap(), 5.0);
    // LogicMemory(Setting) sum should be (3 * 5) = 15
    assert_eq!(read(1.0).unwrap(), 15.0);
    // LogicMemory(Setting) min should be 5
    assert_eq!(read(2.0).unwrap(), 5.0);
    // LogicMemory(Setting) max should be 5
    assert_eq!(read(3.0).unwrap(), 5.0);
}
