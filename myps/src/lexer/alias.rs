use std::collections::HashMap;

use crate::superprelude::*;

#[derive(Debug)]
pub enum DevAlias {
    Lit(usize, usize),
    Indexed(Int),
    Batch(i64),
    Var(String),
}

#[derive(Debug)]
pub enum Alias {
    Dev(DevAlias),
    // TODO: Add functions
    Int(i64),
    Num(f64),
    Var,
}

#[derive(Debug)]
pub struct AliasTable {
    map: HashMap<String, Alias>,
}

// TODO:
// - Validate that aliases in expressions are previously defined
// - Replace aliases that are defined to be values
impl AliasTable {
    pub fn new() -> Self {
        let map = HashMap::new();
        Self { map }
    }

    // pub fn into_mips(self) -> HashMap<String, String> {
    //     println!("{:#?}", &self);
    //     let mut r = 0;
    //     self.map.into_iter().map(|(k, alias)| {
    //         let v = match alias {
    //             Alias::Dev(dev_alias) => {
    //                 match dev_alias {
    //                     DevAlias::Batch(hash) => format!("{}", hash),
    //                     DevAlias::Lit(b, i) => format!("d{:r<i$}{}", b, i=i),
    //                 }
    //             },
    //             Alias::Int(n) => format!("{}", n),
    //             Alias::Num(n) => format!("{}", n),
    //             Alias::Var => {
    //                 r += 1;
    //                 format!("r{}", r - 1)
    //             }
    //         };
    //         (k, v)
    //     }).collect()
    // }

    pub fn insert(&mut self, k: String, alias: Alias) {
        self.map.insert(k, alias);
    }

    pub fn lookup(&self, k: &String) -> MypsLexerResult<&Alias> {
        self.get(k).ok_or(MypsLexerError::undefined_alias(k))
    }

    pub fn validate_dev(&self, k: &String) -> MypsLexerResult<()> {
        let a = self.lookup(k)?;
        match a {
            Alias::Dev(..) | Alias::Var => Ok(()),
            _ => Err(MypsLexerError::wrong_alias("a device", &a))
        }
    }

    pub fn validate_int(&self, k: &String) -> MypsLexerResult<()> {
        let a = self.lookup(k)?;
        match a {
            Alias::Int(_) | Alias::Var => Ok(()),
            _ => Err(MypsLexerError::wrong_alias("an int", &a)),
        }
    }

    pub fn validate_num(&self, k: &String) -> MypsLexerResult<()> {
        let a = self.lookup(k)?;
        match a {
            Alias::Num(_) | Alias::Var => Ok(()),
            _ => Err(MypsLexerError::wrong_alias("a num", &a)),
        }
    }

    fn get(&self, k: &String) -> Option<&Alias> {
        self.map.get(k)
    }
}
