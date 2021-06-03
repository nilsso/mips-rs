#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(dead_code)]
use mips_parser::prelude::*;
use mips_simulator::prelude::*;

fn main() {
    let file = File::open("./tests/device-kinds.ron").unwrap();
    let dev_kinds: DeviceKinds = from_reader(file).unwrap();
    let mut state = ICState::default();

    let solar_kind = &dev_kinds["SolarPanel"];
    for _ in 0..1 {
        state.dev_network_add(solar_kind.make());
    }

    let source = "\
        alias sensor d0
        alias x r0
        define SolarPanelHash -2045627372
        define H0 0 # some multiple of 90
        start:
        yield
        horizontal:
        l x sensor Horizontal
        sub x H0 x
        sb SolarPanelHash Horizontal x
        vertical:
        l x sensor Vertical
        sub x 75 x
        div x x 1.5
        sb SolarPanelHash Vertical x
        j start";

    let program = Program::try_from_str(&source).unwrap();
    let mut sim = ICSimulator::new(state, program);

    // sim.step_n(20).ok();

    let panels = sim.state.network_devices();
    let panel = panels.values().next().unwrap().iter().next().unwrap();

    use ron::ser::{to_string, to_string_pretty, PrettyConfig};

    let config = ron::ser::PrettyConfig::new();
    println!("{}", sim.state);
    println!("{}", to_string_pretty(&panel, config).unwrap());
    println!("{:#?}", panel);

    // println!("
    // println!("{}", sim.state);
}
