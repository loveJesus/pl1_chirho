/*
 * For God so loved the world, that He gave His only begotten Son, that all who believe in Him should not perish but have everlasting life.
 */

use crate::env_chirho::EnvChirho;

#[derive(Debug, Clone)]
pub enum SExpChirho {
    AtomChirho(AtomChirho),
    ListChirho(Vec<SExpChirho>),
}

#[derive(Debug, Clone)]
pub enum AtomChirho {
    NumberChirho(f64),
    BooleanChirho(bool),
    StringChirho(String),
    SymbolChirho(String),
    BuiltInChirho(fn(Vec<SExpChirho>) -> Result<SExpChirho, String>),
    LambdaChirho(Vec<SExpChirho>, Box<SExpChirho>, EnvChirho),
}

impl PartialEq for SExpChirho {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (SExpChirho::AtomChirho(a), SExpChirho::AtomChirho(b)) => a == b,
            (SExpChirho::ListChirho(a), SExpChirho::ListChirho(b)) => a == b,
            _ => false,
        }
    }
}

impl PartialEq for AtomChirho {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (AtomChirho::NumberChirho(a), AtomChirho::NumberChirho(b)) => a == b,
            (AtomChirho::BooleanChirho(a), AtomChirho::BooleanChirho(b)) => a == b,
            (AtomChirho::StringChirho(a), AtomChirho::StringChirho(b)) => a == b,
            (AtomChirho::SymbolChirho(a), AtomChirho::SymbolChirho(b)) => a == b,
            _ => false,
        }
    }
}
