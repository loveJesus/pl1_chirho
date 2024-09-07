/*
 * For God so loved the world, that He gave His only begotten Son, that all who believe in Him should not perish but have everlasting life.
 */

use crate::sexp_chirho::SExpChirho;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct EnvChirho {
    vars_chirho: HashMap<String, SExpChirho>,
    outer_chirho: Option<Box<EnvChirho>>,
}

impl EnvChirho {
    pub fn new_chirho() -> Self {
        EnvChirho {
            vars_chirho: HashMap::new(),
            outer_chirho: None,
        }
    }

    pub fn with_outer_chirho(outer_chirho: EnvChirho) -> Self {
        EnvChirho {
            vars_chirho: HashMap::new(),
            outer_chirho: Some(Box::new(outer_chirho)),
        }
    }

    pub fn get_chirho(&self, key_chirho: &str) -> Option<SExpChirho> {
        match self.vars_chirho.get(key_chirho) {
            Some(value_chirho) => Some(value_chirho.clone()),
            None => self
                .outer_chirho
                .as_ref()
                .and_then(|o_chirho| o_chirho.get_chirho(key_chirho)),
        }
    }

    pub fn set_chirho(&mut self, key_chirho: String, value_chirho: SExpChirho) {
        self.vars_chirho.insert(key_chirho, value_chirho);
    }
}
