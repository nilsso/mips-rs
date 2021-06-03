use mips_simulator::test_utils::setup_run_and_test_mem;

#[test]
fn test_math_abs() {
    // let (_, sim) = setup();
    // assert_program_val!("abs r0  3.14", 0, 3.14);
    // assert_program_val!("abs r0 -3.14", 0, 3.14);
}

#[test]
fn test_math_acos() {}

#[test]
fn test_math_add() {
    setup_run_and_test_mem("add r0  100  50", 0,  150.0);
    setup_run_and_test_mem("add r0  100 -50", 0,   50.0);
    setup_run_and_test_mem("add r0 -100  50", 0,  -50.0);
    setup_run_and_test_mem("add r0 -100 -50", 0, -150.0);
}

#[test]
fn test_math_asin() {}

#[test]
fn test_math_atan() {}

#[test]
fn test_math_ceil() {
    setup_run_and_test_mem("ceil r0 0.1",        0, 1.0);
    setup_run_and_test_mem("ceil r0 0.00000001", 0, 1.0);
    setup_run_and_test_mem("ceil r0 0.99999999", 0, 1.0);
}

#[test]
fn test_math_cos() {}

#[test]
fn test_math_div() {
    setup_run_and_test_mem("div r0  10  2", 0,  5.0);
    setup_run_and_test_mem("div r0   1 -2", 0, -0.5);
    setup_run_and_test_mem("div r0   1  3", 0,  0.333333333333333333); // over precise
    setup_run_and_test_mem("div r0  -1  3", 0, -0.333333333333333333);
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
    setup_run_and_test_mem("mod r0  0  10", 0, 0.0);
    setup_run_and_test_mem("mod r0  1  10", 0, 1.0);
    setup_run_and_test_mem("mod r0  10 10", 0, 0.0);
    setup_run_and_test_mem("mod r0  11 10", 0, 1.0);
    setup_run_and_test_mem("mod r0 -1  10", 0, 9.0);
    setup_run_and_test_mem("mod r0 -9  10", 0, 1.0);
    setup_run_and_test_mem("mod r0 -10 10", 0, 0.0);
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
