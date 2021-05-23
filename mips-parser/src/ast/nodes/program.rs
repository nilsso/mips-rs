use std::{fmt, fmt::Display};
use std::fs::read_to_string;
use std::path::PathBuf;

use itertools::join;
use pest::{iterators::Pair, Parser};

use crate::ast::{AstError, AstResult, FirstInner};
use crate::{MipsParser, MipsParserError, Rule};

use super::Expr;

/// Program node.
#[derive(Clone, PartialEq, Debug)]
pub struct Program(pub Vec<(usize, Expr)>);

impl Program {
    pub fn try_from_pair(pair: Pair<Rule>) -> AstResult<Self> {
        let program = match pair.as_rule() {
            Rule::program => {
                let results = pair
                    .into_inner()
                    .map(|expr_pair| Expr::try_from_pair(expr_pair))
                    .collect::<AstResult<Vec<Option<Expr>>>>()?;
                let expressions = results
                    .into_iter()
                    .enumerate()
                    .filter_map(|(i, expr)| expr.map(|expr| (i, expr)))
                    .collect();
                Self(expressions)
            }
            _ => return Err(AstError::Program),
        };
        Ok(program)
    }

    pub fn try_from_str(input: &str) -> Result<Self, MipsParserError> {
        let peg = MipsParser::parse(Rule::program, input).map_err(MipsParserError::ParserError)?;
        peg.first_inner()
            .and_then(Self::try_from_pair)
            .map_err(MipsParserError::AstError)
    }

    pub fn try_from_path<P: Into<PathBuf>>(path: P) -> Result<Self, MipsParserError> {
        let input = read_to_string(path.into()).map_err(|e| MipsParserError::IOError(e))?;
        Self::try_from_str(&input)
    }

    pub fn empty() -> Self {
        Self(Vec::new())
    }

    /// Iterator over the expressions of this node.
    pub fn iter(&self) -> impl Iterator<Item = &(usize, Expr)> {
        self.0.iter()
    }

    pub fn push(&mut self, expr: Expr) {
        let i = self.0.len();
        self.0.push((i, expr));
    }
}

impl Display for Program {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let lines = join(self.iter().map(|(_, expr)| expr).map(Expr::to_string), "\n");
        writeln!(fmt, "{}", lines)
    }
}
