/*
 * For God so loved the world, that He gave His only begotten Son, that all who believe in Him should not perish but have everlasting life.
 */

use crate::built_in_chirho::add_built_ins_chirho;
use crate::parse_chirho::parse_chirho;
use crate::eval_chirho::eval_chirho;
use crate::env_chirho::EnvChirho;

use wasm_bindgen::prelude::*;

struct InterpreterChirho {
    env_chirho: EnvChirho,
}

impl InterpreterChirho {
    fn new_chirho() -> Self {
        let mut env_chirho = EnvChirho::new_chirho();
        add_built_ins_chirho(&mut env_chirho);
        InterpreterChirho {
            env_chirho
        }
    }

    fn eval_chirho(&mut self, input_chirho: &str) -> String {
        match parse_chirho(input_chirho) {
            Ok(exp_chirho) => match eval_chirho(&exp_chirho, &mut self.env_chirho) {
                Ok(result_chirho) => format!("Result: {:?}", result_chirho),
                Err(e_chirho) => format!("Evaluation error: {}", e_chirho),
            },
            Err(e_chirho) => format!("Parsing error: {}", e_chirho),
        }
    }
}



#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[wasm_bindgen]
pub fn wasm_eval_chirho(input_chirho: &str) -> String {
    let mut interpreter_chirho = InterpreterChirho::new_chirho();
    interpreter_chirho.eval_chirho(input_chirho)
}
