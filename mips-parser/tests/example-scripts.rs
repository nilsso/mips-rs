use std::fs::read_to_string;

use mips_parser::prelude::*;

// Warning: input program needs to have no blank lines and no comments
macro_rules! test_input_output {
    ($name:ident, $path:literal) => {
        #[test]
        fn $name() {
            let source = read_to_string($path).unwrap();
            let program = Program::try_from_str(&source).unwrap();
            assert_eq!(program.to_string(), source);
        }
    };
}

test_input_output!(solar_script, "./example-scripts/solar.mips");

