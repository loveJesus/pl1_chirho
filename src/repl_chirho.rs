/*
 * For God so loved the world, that He gave His only begotten Son, that all who believe in Him should not perish but have everlasting life.
 */

use crate::env_chirho::EnvChirho;
use crate::eval_chirho::eval_chirho;
use crate::parse_chirho::parse_chirho;
use crate::sexp_chirho::AtomChirho;
use crate::sexp_chirho::SExpChirho;
use std::io::{self, Write};

pub fn repl_chirho() {
    let mut env_chirho = EnvChirho::new_chirho();

    // Add some built-in functions
    env_chirho.set_chirho(
        "+".to_string(),
        SExpChirho::AtomChirho(AtomChirho::BuiltInChirho(|args_chirho| {
            let sum_chirho: f64 = args_chirho
                .iter()
                .filter_map(|arg_chirho| {
                    if let SExpChirho::AtomChirho(AtomChirho::NumberChirho(n_chirho)) = arg_chirho {
                        Some(*n_chirho)
                    } else {
                        None
                    }
                })
                .sum();
            Ok(SExpChirho::AtomChirho(AtomChirho::NumberChirho(sum_chirho)))
        })),
    );

    env_chirho.set_chirho(
        "-".to_string(),
        SExpChirho::AtomChirho(AtomChirho::BuiltInChirho(|args_chirho| {
            if args_chirho.is_empty() {
                return Err("- requires at least one argument".to_string());
            }
            let mut nums_chirho: Vec<f64> = args_chirho
                .iter()
                .filter_map(|arg_chirho| {
                    if let SExpChirho::AtomChirho(AtomChirho::NumberChirho(n_chirho)) = arg_chirho {
                        Some(*n_chirho)
                    } else {
                        None
                    }
                })
                .collect();
            if nums_chirho.len() != args_chirho.len() {
                return Err("All arguments to - must be numbers".to_string());
            }
            let mut nums_0_chirho = nums_chirho[0].clone();
            let result_chirho = if nums_chirho.len() == 1 {
                -nums_chirho[0]
            } else {
                nums_chirho
                    .drain(1..)
                    .fold(nums_0_chirho.clone(), |acc_chirho, x_chirho| {
                        acc_chirho - x_chirho
                    })
            };
            Ok(SExpChirho::AtomChirho(AtomChirho::NumberChirho(
                result_chirho,
            )))
        })),
    );

    env_chirho.set_chirho(
        "*".to_string(),
        SExpChirho::AtomChirho(AtomChirho::BuiltInChirho(|args_chirho| {
            let product_chirho: f64 = args_chirho
                .iter()
                .filter_map(|arg_chirho| {
                    if let SExpChirho::AtomChirho(AtomChirho::NumberChirho(n_chirho)) = arg_chirho {
                        Some(*n_chirho)
                    } else {
                        None
                    }
                })
                .product();
            Ok(SExpChirho::AtomChirho(AtomChirho::NumberChirho(
                product_chirho,
            )))
        })),
    );

    env_chirho.set_chirho(
        "/".to_string(),
        SExpChirho::AtomChirho(AtomChirho::BuiltInChirho(|args_chirho| {
            if args_chirho.is_empty() {
                return Err("/ requires at least one argument".to_string());
            }
            let mut nums_chirho: Vec<f64> = args_chirho
                .iter()
                .filter_map(|arg_chirho| {
                    if let SExpChirho::AtomChirho(AtomChirho::NumberChirho(n_chirho)) = arg_chirho {
                        Some(*n_chirho)
                    } else {
                        None
                    }
                })
                .collect();
            if nums_chirho.len() != args_chirho.len() {
                return Err("All arguments to / must be numbers".to_string());
            }
            let mut nums_0_chirho = nums_chirho[0].clone();
            let result_chirho = if nums_chirho.len() == 1 {
                1.0 / nums_chirho[0]
            } else {
                nums_chirho
                    .drain(1..)
                    .fold(nums_0_chirho.clone(), |acc_chirho, x_chirho| {
                        acc_chirho / x_chirho
                    })
            };
            Ok(SExpChirho::AtomChirho(AtomChirho::NumberChirho(
                result_chirho,
            )))
        })),
    );

    env_chirho.set_chirho(
        ">".to_string(),
        SExpChirho::AtomChirho(AtomChirho::BuiltInChirho(|args_chirho| {
            if args_chirho.len() != 2 {
                return Err("> requires exactly two arguments".to_string());
            }
            if let (
                SExpChirho::AtomChirho(AtomChirho::NumberChirho(a_chirho)),
                SExpChirho::AtomChirho(AtomChirho::NumberChirho(b_chirho)),
            ) = (&args_chirho[0], &args_chirho[1])
            {
                Ok(SExpChirho::AtomChirho(AtomChirho::BooleanChirho(
                    a_chirho > b_chirho,
                )))
            } else {
                Err("Both arguments to > must be numbers".to_string())
            }
        })),
    );

    loop {
        print!("s-exp-chirho> ");
        io::stdout().flush().unwrap();

        let mut input_chirho = String::new();
        io::stdin().read_line(&mut input_chirho).unwrap();

        if input_chirho.trim() == "exit" {
            break;
        }

        match parse_chirho(&input_chirho) {
            Ok(exp_chirho) => match eval_chirho(&exp_chirho, &mut env_chirho) {
                Ok(result_chirho) => println!("Result: {:?}", result_chirho),
                Err(e_chirho) => println!("Evaluation error: {}", e_chirho),
            },
            Err(e_chirho) => println!("Parsing error: {}", e_chirho),
        }
    }
}
