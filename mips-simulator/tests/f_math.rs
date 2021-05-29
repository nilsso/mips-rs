use mips_parser::prelude::{Node, Program};
use mips_simulator::prelude::{stationeers_ic, DeviceKinds, ICSimulator};
use ron::de::from_reader;
use std::fs::File;
use util::{
    assert_mem, assert_program_val, run_until_finished, test_setup_dev_kinds, test_setup_sim,
};

#[test]
fn test_math_abs() {
    assert_program_val!("abs r0  3.14", 0, 3.14);
    assert_program_val!("abs r0 -3.14", 0, 3.14);
}

#[test]
fn test_math_acos() {}

#[test]
fn test_math_add() {
    assert_program_val!("add r0  100  50", 0,  150.0);
    assert_program_val!("add r0  100 -50", 0,   50.0);
    assert_program_val!("add r0 -100  50", 0,  -50.0);
    assert_program_val!("add r0 -100 -50", 0, -150.0);
}

#[test]
fn test_math_asin() {}

#[test]
fn test_math_atan() {}

#[test]
fn test_math_ceil() {
    assert_program_val!("ceil r0 0.1",        0, 1.0);
    assert_program_val!("ceil r0 0.00000001", 0, 1.0);
    assert_program_val!("ceil r0 0.99999999", 0, 1.0);
}

#[test]
fn test_math_cos() {}

#[test]
fn test_math_div() {
    assert_program_val!("div r0  10  2", 0,  5.0);
    assert_program_val!("div r0   1 -2", 0, -0.5);
    assert_program_val!("div r0   1  3", 0,  0.333333333333333333); // over precise
    assert_program_val!("div r0  -1  3", 0, -0.333333333333333333);
}

#[test]
fn test_math_exp() {}

#[test]
fn test_math_floor() {}

#[test]
fn test_math_log() {}

#[test]
fn test_math_max() {}

#[test]
fn test_math_min() {}

#[test]
fn test_math_mod() {
    assert_program_val!("mod r0  0  10", 0, 0.0);
    assert_program_val!("mod r0  1  10", 0, 1.0);
    assert_program_val!("mod r0  10 10", 0, 0.0);
    assert_program_val!("mod r0  11 10", 0, 1.0);
    assert_program_val!("mod r0 -1  10", 0, 9.0);
    assert_program_val!("mod r0 -9  10", 0, 1.0);
    assert_program_val!("mod r0 -10 10", 0, 0.0);
}

#[test]
fn test_math_mul() {}

#[test]
fn test_math_rand() {}

#[test]
fn test_math_round() {}

#[test]
fn test_math_sin() {}

#[test]
fn test_math_sqrt() {}

#[test]
fn test_math_sub() {}

#[test]
fn test_math_tan() {}

#[test]
fn test_math_trunc() {}
