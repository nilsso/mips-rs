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
