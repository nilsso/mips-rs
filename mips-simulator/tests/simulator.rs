// use mips_parser::ast::nodes::Program;
// use mips_simulator::prelude::*;

// macro_rules! run {
//     ($s:literal) => {{
//         let mut state = ICState::default();
//         let program = Program::try_from_str("alias x r1\nmove x 5").unwrap();
//         for (_, expr) in program.iter() {
//             assert!(state.try_exec_expr(expr).is_ok());
//         }
//         state
//     }};
// }

// macro_rules! assert_alias {
//     ($state:ident, $k:literal, $a: expr) => {
//         let a = $state.try_get_alias(&$k.into()).unwrap();
//         assert_eq!(a, &$a);
//     };
// }

// macro_rules! assert_mem {
//     ($state:ident, $i:literal, $v:literal) => {
//         let v = $state.try_get_mem($i).unwrap();
//         assert_eq!(v, &$v);
//     };
// }

// #[test]
// fn test() {
//     let state = run!("alias x r1\nmove x 5");
//     assert_alias!(state, "x", AliasKind::MemId(1));
//     assert_mem!(state, 1, 5_f64);
// }
