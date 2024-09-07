/*
 * For God so loved the world, that He gave His only begotten Son, that all who believe in Him should not perish but have everlasting life.
 */


use crate::built_in_chirho::add_built_ins_chirho;
use crate::env_chirho::EnvChirho;
use crate::eval_chirho::eval_chirho;
use crate::parse_chirho::parse_chirho;

use atty;
use rustyline::Editor;
use rustyline::error::ReadlineError;

use std::io::{self, Read};

#[cfg(not(target_arch = "wasm32"))]
pub fn repl_chirho() {
    let mut env_chirho = EnvChirho::new_chirho();

    // Add some built-in functions
    add_built_ins_chirho(&mut env_chirho);

    if atty::is(atty::Stream::Stdin) {

        let mut rl_chirho = Editor::<()>::new();

        loop {
            let readline_chirho = rl_chirho.readline("s-exp-chirho> ");
            match readline_chirho {
                Ok(line_chirho) => {
                    rl_chirho.add_history_entry(line_chirho.as_str());
                    process_input_chirho(&line_chirho, &mut env_chirho);
                },
                Err(ReadlineError::Interrupted) => {
                    println!("CTRL-C");
                    break
                },
                Err(ReadlineError::Eof) => {
                    println!("CTRL-D");
                    break
                },
                Err(err_chirho) => {
                    println!("Error: {:?}", err_chirho);
                    break
                }
            }
        }
    } else {
        let mut input_chirho = String::new();
        io::stdin().read_to_string(&mut input_chirho).unwrap();
        process_input_chirho(&input_chirho, &mut env_chirho);
    }
}

fn process_input_chirho(input_chirho: &str, env_chirho: &mut EnvChirho) {
    let mut paren_count_chirho: i64 = 0;
    let mut current_expression_chirho = String::new();

    for line_chirho in input_chirho.lines() {
        let line_chirho = line_chirho.trim();
        if line_chirho.is_empty() || line_chirho.starts_with(';') {
            continue;
        }

        current_expression_chirho.push_str(line_chirho);
        current_expression_chirho.push(' ');

        paren_count_chirho += line_chirho.chars().filter(|&c_chirho| c_chirho == '(').count() as i64;
        paren_count_chirho -= line_chirho.chars().filter(|&c_chirho| c_chirho == ')').count() as i64;

        if paren_count_chirho == 0 {
            match parse_chirho(&current_expression_chirho) {
                Ok(exp_chirho) => match eval_chirho(&exp_chirho, env_chirho) {
                    Ok(result_chirho) => println!("Result: {:?}", result_chirho),
                    Err(e_chirho) => println!("Evaluation error: {}", e_chirho),
                },
                Err(e_chirho) => println!("Parsing error: {}", e_chirho),
            }
            current_expression_chirho.clear();
        }
    }

    if !current_expression_chirho.is_empty() {
        println!("Hallelujah, Incomplete expression\nparen_count_chirho: {}\nexpression: {}", paren_count_chirho, current_expression_chirho);
    }
}
