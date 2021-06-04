use crate::prelude::*;
use mips_parser::prelude::{Node, Program};

pub fn dev_kinds() -> DeviceKinds {
    const PATH: &'static str = "./tests/device-kinds.ron";
    let file = File::open(PATH).unwrap();
    from_reader(file).unwrap()
}

pub fn setup(source: &'static str) -> ICSimulatorDefault {
    let program = Program::try_from_str(&source).unwrap();
    let state = ICState::default();
    ICSimulator::new(state, program)
}

pub fn setup_and_run(source: &'static str) -> ICSimulatorDefault {
    let mut sim = setup(source);
    sim.run_until_finished().unwrap();
    sim
}

pub fn setup_run_and_test_mem(source: &'static str, i: usize, ans: f64) {
    let sim = setup_and_run(source);
    assert_eq!(sim.state.mem[i], ans);
}
