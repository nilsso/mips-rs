#![allow(unused_imports)]
use std::path::Path;
use std::fs;
use std::fmt::Debug;
use std::io::Error as IOError;

use pest::{RuleType, Parser};
use pest::iterators::{Pair, Pairs};
use pest::error::Error as PegError;

#[derive(Debug)]
pub enum AstError {
    InsufficientPairs,
}

pub trait ParserError<R> = From<PegError<R>> + From<AstError> + From<IOError> + Debug;

pub trait NextPair<'i, R>
where
    R: RuleType,
{
    fn next_pair(&mut self) -> Result<Pair<'i, R>, AstError>;
}

pub trait FirstInner<'i, R>
where
    R: RuleType,
{
    fn first_inner(self) -> Result<Pair<'i, R>, AstError>;
}

impl<'i, R> NextPair<'i, R> for Pairs<'i, R>
where
    R: RuleType,
{
    fn next_pair(&mut self) -> Result<Pair<'i, R>, AstError> {
        self.next().ok_or(AstError::InsufficientPairs.into())
    }
}

impl<'i, R> FirstInner<'i, R> for Pairs<'i, R>
where
    R: RuleType,
{
    fn first_inner(mut self) -> Result<Pair<'i, R>, AstError> {
        self.next_pair()
    }
}

impl<'i, R> FirstInner<'i, R> for Pair<'i, R>
where
    R: RuleType,
{
    fn first_inner(self) -> Result<Pair<'i, R>, AstError> {
        self.into_inner().first_inner()
    }
}

pub trait AstNode<'i, R, P, E>
where
    Self: Sized,
    R: RuleType,
    P: Parser<R>,
    E: ParserError<R>,
{
    type Output;

    const RULE: R;

    fn try_from_pair(pair: Pair<R>) -> Result<Self::Output, E>;

    fn try_from_str<S: AsRef<str>>(source: &S) -> Result<Self::Output, E> {
        let pairs = P::parse(Self::RULE, source.as_ref())?;
        pairs.first_inner().map_err(E::from).and_then(Self::try_from_pair)
    }

    fn try_from_file(path: &Path) -> Result<Self::Output, E> {
        let input = fs::read_to_string(path)?;
        Self::try_from_str(&input)
    }

    fn from_pair(pair: Pair<R>) -> Self::Output {
        Self::try_from_pair(pair).unwrap()
    }

    fn from_str<S: AsRef<str>>(source: &S) -> Self::Output {
        Self::try_from_str(source).unwrap()
    }

    fn from_file(path: &Path) -> Self::Output {
        Self::try_from_file(path).unwrap()
    }
}

pub trait IntoAst<'i, R, P, E>
where
    Self: Sized,
    R: RuleType,
    P: Parser<R>,
    E: ParserError<R>,
{
    fn try_into_ast<A: AstNode<'i, R, P, E, Output = A>>(self) -> Result<A, E>;

    fn into_ast<A: AstNode<'i, R, P, E, Output = A>>(self) -> A {
        Self::try_into_ast(self).unwrap()
    }
}

impl<'i, R, P, E> IntoAst<'i, R, P, E> for Pair<'i, R>
where
    R: RuleType,
    P: Parser<R>,
    E: ParserError<R>,
{
    fn try_into_ast<A: AstNode<'i, R, P, E, Output = A>>(self) -> Result<A, E> {
        A::try_from_pair(self)
    }
}

// pub trait IntoAst<A: Ast>
// where
//     Self: Sized,
// {
//     fn try_into_ast(self) -> MypsParserResult<A>;

//     fn into_ast(self) -> A {
//         Self::try_into_ast(self).unwrap()
//     }
// }

// impl<'i, A: Ast<Output = A>> IntoAst<A> for Pair<'i, Rule> {
//     fn try_into_ast(self) -> MypsParserResult<A> {
//         A::try_from_pair(self)
//     }
// }

