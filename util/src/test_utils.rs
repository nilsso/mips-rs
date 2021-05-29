#[macro_export]
macro_rules! test_setup_dev_kinds {
    () => {{
        let file = File::open("./tests/device-kinds.ron").unwrap();
        let dev_kinds: DeviceKinds = from_reader(file).unwrap();
        dev_kinds
    }};
}

#[macro_export]
macro_rules! test_setup_sim {
    ($dev_kinds:ident, $s:literal) => {{
        let program = Program::try_from_str(&$s).unwrap();
        let state = stationeers_ic(&$dev_kinds);
        let sim = ICSimulator::new(state, program);
        sim
    }};
}

#[macro_export]
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

#[macro_export]
macro_rules! assert_mem {
    ($sim:expr, $i:literal, $v:literal) => {
        let v = $sim.state.get_mem($i).unwrap();
        assert_eq!(v, &$v);
    };
}

#[macro_export]
macro_rules! assert_program_val {
    ($expr:literal, $r:literal, $ans:literal) => {
        let dev_kinds = test_setup_dev_kinds!();
        let mut sim = test_setup_sim!(dev_kinds, $expr);

        run_until_finished!(sim);

        assert_mem!(sim, $r, $ans);
    };
}

#[macro_export]
macro_rules! mips_ast_test {
    ($name:ident, $mips:literal, $rule:path, $ast:ty, $res:expr) => {
        #[test]
        fn $name() {
            let peg = MipsParser::parse($rule, $mips);
            println!("{:?}", peg);
            let peg = peg.unwrap().first_inner().unwrap();
            println!("{:?}", peg);
            let ast = <$ast>::try_from_pair(peg).unwrap();
            assert_eq!(ast, $res);
        }
    }
}
