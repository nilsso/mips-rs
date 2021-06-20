use std::collections::HashSet;

use crate::superprelude::*;

#[derive(Clone, Debug)]
pub enum FunctionCall {
    Nullary(String),
    Unary(String, RValue),
    User(String),
}

#[derive(Clone, Debug)]
pub enum Statement {
    // AssignAlias(String, Dev),
    AssignValue(Vec<LValue>, Vec<RValue>),
    AssignSelf(BinaryOp, LValue, RValue),
    FunctionCall(FunctionCall),
    Empty,
}

impl Statement {
    pub fn analyze(&self, aliases: &mut HashSet<String>, called_functions: &mut HashSet<String>) -> MypsLexerResult<()> {
        match self {
            Self::AssignValue(l_values, r_values) => {
                for l_value in l_values.iter() {
                    l_value.analyze(aliases)?;
                }
                for r_value in r_values.iter() {
                    r_value.analyze(aliases)?;
                }
            },
            Self::AssignSelf(_, l_value, r_value) => {
                l_value.analyze(aliases)?;
                r_value.analyze(aliases)?;
            },
            Self::FunctionCall(function_call) => {
                match function_call {
                    FunctionCall::Nullary(..) | FunctionCall::Unary(..) => {},
                    FunctionCall::User(name) => {
                        called_functions.insert(name.to_owned());
                    }
                }
            }
            Self::Empty => {},
        }
        Ok(())
    }
}

impl<'i> AstNode<'i, Rule, MypsParser, MypsLexerError> for Statement {
    type Output = Self;

    const RULE: Rule = Rule::stmt;

    fn try_from_pair(pair: Pair<Rule>) -> MypsLexerResult<Self> {
        match pair.as_rule() {
            // Rule::stmt => pair.first_inner()?.try_into_ast(),
            Rule::stmt => pair.only_inner().unwrap().try_into_ast(),
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
                            _ => false,
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
                // let expr = Expr::binary(op, Expr::RValue(l_value.as_rvalue()), r_value);
                // let r_value = RValue::Expr(Box::new(expr));
                // let stmt = Self::AssignValue(vec![l_value], vec![r_value]);
                let stmt = Self::AssignSelf(op, l_value, r_value);
                Ok(stmt)
            }
            Rule::stmt_func_nullary => {
                let name = pair.only_inner()?.as_str().into();
                let function_call = FunctionCall::Nullary(name);
                Ok(Self::FunctionCall(function_call))
            }
            Rule::stmt_func_unary => {
                let mut pairs = pair.into_inner();
                let name = pairs.next_pair()?.as_str().into();
                let rv = pairs.final_pair()?.try_into_ast()?;
                let function_call = FunctionCall::Unary(name, rv);
                Ok(Self::FunctionCall(function_call))
            }
            Rule::stmt_func_user => {
                let name = pair.only_inner()?.as_str().into();
                let function_call = FunctionCall::User(name);
                Ok(Self::FunctionCall(function_call))
            }
            _ => Err(MypsLexerError::wrong_rule("a statement", pair)),
        }
    }
}
