use mips_parser::prelude::*;
use mips_state::prelude::*;

fn pair(s: &str) -> Node {
    Node::new(MipsParser::parse(Rule::reg, s).expect("").next().unwrap())
}

fn state_with_ascending_offset_values(offset: usize) -> MipsState {
    MipsStateBuilder::new(16, 6)
        .with_default_aliases()
        .with_mem((0..16).map(|v| (v + offset) as f32).collect())
        .build()
}

fn default_state() -> MipsState {
    state_with_ascending_offset_values(1)
}

fn default_value(s: &str) -> Result<f32, StateError> {
    let state = default_state();
    let r = pair(s).as_register().unwrap();
    state.value(&r)
}

#[test]
fn state_value_r0() {
    assert_eq!(default_value("r0"), Ok(1_f32));
}

#[test]
fn state_value_rr0() {
    assert_eq!(default_value("rr0"), Ok(2_f32));
}

#[test]
fn state_value_rrrrrrr8() {
    assert_eq!(default_value("rrrrrrr8"), Ok(15_f32));
}

#[test]
#[should_panic(expected = "Invalid memory index 16")]
fn state_value_mem_index_out_of_bounds() {
    if let Err(e) = default_value("rr15") {
        panic!("{}", e);
    }
}

#[test]
#[should_panic(expected = "Invalid alias x")]
fn state_value_invalid_alias() {
    if let Err(e) = default_value("x") {
        panic!("{}", e);
    }
}

#[test]
#[should_panic(expected = "Cannot find value a from register of type Device")]
fn state_value_invalid_register1() {
    if let Err(e) = default_value("d0") {
        panic!("{}", e);
    }
}

#[test]
#[should_panic(expected = "Cannot find value a from register of type Device")]
fn state_value_invalid_register2() {
    if let Err(e) = default_value("drrr0") {
        panic!("{}", e);
    }
}
