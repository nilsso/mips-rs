use std::collections::HashMap;

use crate::lexer::Item;

pub fn analyze(item: &Item) -> HashMap<String, String> {
    item.dependencies
        .iter()
        .enumerate()
        .map(|(i, k)| (k.clone(), format!("r{}", i)))
        .collect()
}
