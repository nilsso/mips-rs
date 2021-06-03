// use mips_parser::prelude::{Node, Program};
// use mips_simulator::prelude::{stationeers_ic, DeviceKinds, ICSimulator, ICStateDefault};
// use ron::de::from_reader;
// use std::fs::File;
// use util::{run_until_finished, test_setup_dev_kinds, test_setup_sim};

#[test]
fn test_stack_push() {
    // let dev_kinds = test_setup_dev_kinds!();
    // let mut sim = test_setup_sim!(
    //     dev_kinds,
    //     "push 10
    //      push 11
    //      push 12
    //      pop r0
    //      peek r1
    //      pop r2
    //      pop r3"
    // );

    // run_until_finished!(sim);

    // let stack = sim.state.get_stack_buffer();
    // let mem = sim.state.get_mem_buffer();
    // assert_eq!(stack[0], 10.0);
    // assert_eq!(stack[1], 11.0);
    // assert_eq!(stack[2], 12.0);
    // assert_eq!(mem[0], 12.0);
    // assert_eq!(mem[1], 11.0);
    // assert_eq!(mem[2], 11.0);
    // assert_eq!(mem[3], 10.0);
    // assert_eq!(mem[ICStateDefault::SP], 0.0);
}
