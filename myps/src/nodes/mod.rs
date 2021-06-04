// #[derive(Clone, PartialEq, Debug)]
// pub struct Token(String);

// impl<'i> AstNode<'i, Rule, MypsParser, MypsParserError> for Token {
//     type Output = Self;

//     const RULE: Rule = Rule::token;

//     fn try_from_pair(pair: Pair<Rule>) -> MypsParserResult<Self> {
//         match pair.as_rule() {
//             Rule::token => Ok(Self(pair.as_str().into())),
//             _ => Err(MypsParserError::new_wrong_rule("token", pair)),
//         }
//     }
// }

mod types;
pub use types::{Int, Num, Dev};

mod expr;
pub use expr::Expr;
pub use expr::operators::{UnaryOp, BinaryOp};
pub use expr::l_value::LValue;
pub use expr::r_value::RValue;

mod statement;
pub use statement::Statement;
