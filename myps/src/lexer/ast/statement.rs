use std::{fmt, fmt::Display};

use crate::superprelude::*;

#[derive(Clone, Debug)]
pub enum Statement {
    // AssignAlias(String, Dev),
    AssignValue(Vec<LValue>, Vec<RValue>),
    FunctionCall(String, Vec<Arg>),
    Return(Vec<RValue>),
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
            Rule::assign_value => {
                let mut l_values = Vec::new();
                let mut r_values = Vec::new();

                let mut inner_pairs = pair.into_inner();
                let mut pair_opt = inner_pairs.next();
                while let Some(pair) = &pair_opt {
                    if pair.as_rule() == Rule::l_value {
                        l_values.push(pair.clone().try_into_ast().unwrap());
                        pair_opt = inner_pairs.next();
                    } else {
                        break;
                    }
                }
                while let Some(pair) = pair_opt {
                    r_values.push(pair.try_into_ast().unwrap());
                    pair_opt = inner_pairs.next();
                }

                Ok(Self::AssignValue(l_values, r_values))
            }
            Rule::assign_op => {
                let mut inner_pairs = pair.into_inner();
                // let l_value = inner_pairs.next_pair()?.try_into_ast::<LValue>()?;
                let l_value = inner_pairs
                    .next_pair()
                    .unwrap()
                    .try_into_ast::<LValue>()
                    .unwrap();
                // let op_asn_pair = inner_pairs.next_pair()?;
                let op_asn_pair = inner_pairs.next_pair().unwrap();
                // let r_value = inner_pairs.next_pair()?.try_into_ast()?;
                let r_value = inner_pairs.next_pair().unwrap().try_into_ast().unwrap();
                let op = match op_asn_pair.as_rule() {
                    Rule::add_asn => BinaryOp::Add,
                    Rule::sub_asn => BinaryOp::Sub,
                    Rule::mul_asn => BinaryOp::Mul,
                    Rule::div_asn => BinaryOp::Div,
                    Rule::rem_asn => BinaryOp::Rem,
                    _ => unreachable!("{:?}", op_asn_pair),
                };
                let expr = Expr::binary(op, Expr::RValue(l_value.as_rvalue()), r_value);
                let r_value = RValue::Expr(Box::new(expr));
                Ok(Self::AssignValue(vec![l_value], vec![r_value]))
            }
            Rule::func => {
                let mut inner_pairs = pair.into_inner();
                // let name = inner_pairs.next_pair()?.as_str().into();
                let name = inner_pairs.next_pair().unwrap().as_str().into();
                let args = inner_pairs
                    // .first_inner()?
                    .first_inner()
                    .unwrap()
                    .into_inner()
                    .map(|pair| match pair.as_rule() {
                        Rule::dev => Dev::try_from_pair(pair).unwrap().into(),
                        Rule::r_value => RValue::try_from_pair(pair).unwrap().into(),
                        _ => unreachable!("{:?}", pair),
                    })
                    // .collect::<MypsLexerResult<Vec<RValue>>>()?;
                    // .collect::<MypsLexerResult<Vec<RValue>>>().unwrap();
                    .collect::<Vec<Arg>>();
                Ok(Self::FunctionCall(name, args))
            }
            Rule::func_return => {
                let rvalues = pair
                    .into_inner()
                    .map(RValue::try_from_pair)
                    .collect::<MypsLexerResult<Vec<RValue>>>()
                    .unwrap();
                Ok(Self::Return(rvalues))
            }
            _ => Err(MypsLexerError::wrong_rule("a statement", pair)),
        }
    }
}
