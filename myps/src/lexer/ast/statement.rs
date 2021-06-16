use std::{fmt, fmt::Display};

use crate::superprelude::*;

#[derive(Clone, Debug)]
pub enum Statement {
    // AssignAlias(String, Dev),
    AssignValue(Vec<LValue>, Vec<RValue>),
    FunctionCall(String, Vec<RValue>),
}

impl<'i> AstNode<'i, Rule, MypsParser, MypsLexerError> for Statement {
    type Output = Self;

    const RULE: Rule = Rule::stmt;

    fn try_from_pair(pair: Pair<Rule>) -> MypsLexerResult<Self> {
        match pair.as_rule() {
            // Rule::stmt => pair.first_inner()?.try_into_ast(),
            Rule::stmt => pair.first_inner().unwrap().try_into_ast(),
            // Rule::assign_alias => {
            //     let mut inner_pairs = pair.into_inner();
            //     // let alias = inner_pairs.next_pair()?.as_str().into();
            //     let alias = inner_pairs.next_pair().unwrap().as_str().into();
            //     // let dev = inner_pairs.next_pair()?.try_into_ast()?;
            //     let dev = inner_pairs.next_pair().unwrap().try_into_ast().unwrap();
            //     Ok(Self::AssignAlias(alias, dev))
            // }
            Rule::stmt_assign_value => {

                let (l_value_pairs, r_value_pairs) = pair
                    .into_inner().partition::<Vec<Pair<Rule>>, _>(|pair| {
                        match pair.as_rule() {
                            Rule::lv => true,
                            Rule::expr => false,
                            _ => unreachable!("{:?}", pair),
                        }
                    });

                if l_value_pairs.len() != r_value_pairs.len() {
                    panic!();
                }

                let l_values = l_value_pairs.into_iter().map(LValue::try_from_pair)
                    .collect::<MypsLexerResult<Vec<LValue>>>()
                    .unwrap();

                let r_values = r_value_pairs.into_iter().map(RValue::try_from_pair)
                    .collect::<MypsLexerResult<Vec<RValue>>>()
                    .unwrap();

                Ok(Self::AssignValue(l_values, r_values))
            }
            Rule::stmt_assign_self => {
                let mut inner_pairs = pair.into_inner();
                // let l_value = inner_pairs.next_pair()?.try_into_ast::<LValue>()?;
                let l_value = inner_pairs
                    .next_pair()
                    .unwrap()
                    .try_into_ast::<LValue>()
                    .unwrap();
                // let op_asn_pair = inner_pairs.next_pair()?;
                let op_pair = inner_pairs.next_pair().unwrap();
                // let r_value = inner_pairs.next_pair()?.try_into_ast()?;
                let r_value = inner_pairs.next_pair().unwrap().try_into_ast().unwrap();
                let op = match op_pair.as_str() {
                    "+=" => BinaryOp::Add,
                    "-=" => BinaryOp::Sub,
                    "*=" => BinaryOp::Mul,
                    "/=" => BinaryOp::Div,
                    "%/" => BinaryOp::Rem,
                    _ => unreachable!("{:?}", op_pair),
                };
                let expr = Expr::binary(op, Expr::RValue(l_value.as_rvalue()), r_value);
                let r_value = RValue::Expr(Box::new(expr));
                Ok(Self::AssignValue(vec![l_value], vec![r_value]))
            }
            Rule::stmt_func_nullary => {
                unreachable!();
                // let mut inner_pairs = pair.into_inner();
                // // let name = inner_pairs.next_pair()?.as_str().into();
                // let name = inner_pairs.next_pair().unwrap().as_str().into();
                // let args = inner_pairs
                //     // .first_inner()?
                //     .first_inner()
                //     .unwrap()
                //     .into_inner()
                //     .map(|pair| match pair.as_rule() {
                //         Rule::dev => Dev::try_from_pair(pair).unwrap().into(),
                //         Rule::r_value => RValue::try_from_pair(pair).unwrap().into(),
                //         _ => unreachable!("{:?}", pair),
                //     })
                //     // .collect::<MypsLexerResult<Vec<RValue>>>()?;
                //     // .collect::<MypsLexerResult<Vec<RValue>>>().unwrap();
                //     .collect::<Vec<Arg>>();
                // Ok(Self::FunctionCall(name, args))
            }
            Rule::stmt_func_unary => {
                unreachable!();
            }
            _ => Err(MypsLexerError::wrong_rule("a statement", pair)),
        }
    }
}
