use crate::superprelude::*;

/// Device node.
#[derive(Clone, PartialEq, Debug)]
pub enum Dev {
    Lit(usize, usize),
    Indexed(Int),
    Batch(i64),
    Var(String),
}

impl<'i> AstNode<'i, Rule, MypsParser, MypsLexerError> for Dev {
    type Output = Self;

    // dev = _{ dev_batch | dev_lit | var }
    //     dev_batch = ${ int ~ ".all" }
    //     dev_lit   = ${ "d" ~ "r"* ~ !".all" ~ int_lit }
    const RULE: Rule = Rule::dev;

    fn try_from_pair(pair: Pair<Rule>) -> MypsLexerResult<Self> {
        match pair.as_rule() {
            Rule::dev_base => {
                let s = pair.as_str();
                let indirections = s.bytes().filter(|b| *b == b'r').count() as usize;
                let base_index = pair.first_inner()?.as_str().parse()?;
                Ok(Dev::Lit(base_index, indirections))
            }
            Rule::dev_batch => {
                let hash = pair.first_inner()?.as_str().parse()?;
                Ok(Self::Batch(hash))
            }
            Rule::dev_id => {
                let index = pair.first_inner()?.try_into_ast()?;
                Ok(Self::Indexed(index))
            }
            Rule::var => Ok(Self::Var(pair.as_str().into())),
            _ => return Err(MypsLexerError::wrong_rule("a device", pair)),
        }
    }
}

impl Display for Dev {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Lit(b, i) => {
                write!(f, "d")?;
                for _ in 0..*i {
                    write!(f, "r")?;
                }
                write!(f, "{}", i)
            },
            Self::Indexed(id) => {
                write!(f, "d{}", id)
            },
            Self::Batch(hash) => {
                write!(f, "{}", hash)
            },
            Self::Var(var) => {
                write!(f, "{}", var)
            },
        }
    }
}
