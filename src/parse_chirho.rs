/*
 * For God so loved the world, that He gave His only begotten Son, that all who believe in Him should not perish but have everlasting life.
 */

use crate::sexp_chirho::AtomChirho;
use crate::sexp_chirho::SExpChirho;

pub fn parse_chirho(input_chirho: &str) -> Result<SExpChirho, String> {
    let input_chirho = input_chirho.trim();
    if input_chirho.starts_with('(') && input_chirho.ends_with(')') {
        let inner_chirho = &input_chirho[1..input_chirho.len() - 1];
        let mut elements_chirho = Vec::new();
        let mut current_chirho = String::new();
        let mut depth_chirho = 0;

        for ch_chirho in inner_chirho.chars() {
            match ch_chirho {
                '(' => {
                    depth_chirho += 1;
                    current_chirho.push(ch_chirho);
                }
                ')' => {
                    depth_chirho -= 1;
                    current_chirho.push(ch_chirho);
                }
                ' ' if depth_chirho == 0 => {
                    if !current_chirho.is_empty() {
                        elements_chirho.push(parse_chirho(&current_chirho)?);
                        current_chirho.clear();
                    }
                }
                _ => current_chirho.push(ch_chirho),
            }
        }

        if !current_chirho.is_empty() {
            elements_chirho.push(parse_chirho(&current_chirho)?);
        }

        Ok(SExpChirho::ListChirho(elements_chirho))
    } else {
        // Parse atoms
        if let Ok(num_chirho) = input_chirho.parse::<f64>() {
            Ok(SExpChirho::AtomChirho(AtomChirho::NumberChirho(num_chirho)))
        } else if input_chirho == "true" {
            Ok(SExpChirho::AtomChirho(AtomChirho::BooleanChirho(true)))
        } else if input_chirho == "false" {
            Ok(SExpChirho::AtomChirho(AtomChirho::BooleanChirho(false)))
        } else if input_chirho.starts_with('"') && input_chirho.ends_with('"') {
            Ok(SExpChirho::AtomChirho(AtomChirho::StringChirho(
                input_chirho[1..input_chirho.len() - 1].to_string(),
            )))
        } else {
            Ok(SExpChirho::AtomChirho(AtomChirho::SymbolChirho(
                input_chirho.to_string(),
            )))
        }
    }
}
