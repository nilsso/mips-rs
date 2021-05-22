use std::fs::read_to_string;

use mips_parser::prelude::*;

macro_rules! test_input_output {
    ($name:ident, $path:literal) => {
        #[test]
        fn $name() {
            let source = read_to_string($path).unwrap();
            let program = build_ast_from_str(&source).unwrap();
            assert_eq!(source, program.to_string());
        }
    };
}

test_input_output!(solar_script, "./example-scripts/solar.mips");

