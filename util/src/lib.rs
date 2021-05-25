#[macro_export]
macro_rules! is_as_inner {
    ($Self:ty, $Error:ty, $error:path,
        [$( ($self:path, $is:ident, $as:ident, $inner:ident, $Inner:ty) ),*$(,)*]
    ) => {
        $(
            pub fn $is(this: &Self) -> bool {
                matches!($self, this)
            }

            pub fn $as(&self) -> Option<&Self> {
                <$Self>::$is(self).then_some(self)
            }

            pub fn $inner(&self) -> Result<$Inner, $Error> {
                match self {
                    $self(inner) => Ok(inner),
                    _ => Err($error(format!(""))),
                }
            }
        )*
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
