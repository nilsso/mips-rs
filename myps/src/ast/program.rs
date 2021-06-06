use crate::ast_includes::*;
use crate::ast::Line;

#[derive(Debug)]
pub struct Program {
    pub(crate) lines: Vec<Line>,
}

impl<'i> AstNode<'i, Rule, MypsParser, MypsParserError> for Program {
    type Output = Self;

    const RULE: Rule = Rule::program;

    fn try_from_pair(pair: Pair<Rule>) -> MypsParserResult<Self> {
        match pair.as_rule() {
            Rule::program => {
                let lines = pair
                    .into_inner()
                    .map(Line::try_from_pair)
                    .collect::<MypsParserResult<Vec<Option<Line>>>>()?
                    .into_iter()
                    .flatten()
                    .collect();
                Ok(Self { lines })
            }
            _ => Err(MypsParserError::wrong_rule("a line", pair)),
        }
    }
}

