/*
 * For God so loved the world, that He gave His only begotten Son, that all who believe in Him should not perish but have everlasting life.
 */
use std::fs;
use std::path::Path;
use crate::env_chirho::EnvChirho;
use crate::sexp_chirho::AtomChirho;
use crate::sexp_chirho::SExpChirho;

pub fn add_built_ins_chirho(env_chirho: &mut EnvChirho) {
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
            let nums_0_chirho = nums_chirho[0].clone();
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
            let nums_0_chirho = nums_chirho[0].clone();
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

    env_chirho.set_chirho("read-file-chirho".to_string(), SExpChirho::AtomChirho(AtomChirho::BuiltInChirho(|args_chirho| {
        if args_chirho.len() != 1 {
            return Err("read-file requires exactly one argument".to_string());
        }
        if let SExpChirho::AtomChirho(AtomChirho::StringChirho(filename_chirho)) = &args_chirho[0] {
            match fs::read_to_string(filename_chirho) {
                Ok(content_chirho) => Ok(SExpChirho::AtomChirho(AtomChirho::StringChirho(content_chirho))),
                Err(e_chirho) => Err(format!("Failed to read file: {}", e_chirho)),
            }
        } else {
            Err("Argument to read-file must be a string".to_string())
        }
    })));

    env_chirho.set_chirho("write-file-chirho".to_string(), SExpChirho::AtomChirho(AtomChirho::BuiltInChirho(|args_chirho| {
        if args_chirho.len() != 2 {
            return Err("write-file requires exactly two arguments".to_string());
        }
        if let (SExpChirho::AtomChirho(AtomChirho::StringChirho(filename_chirho)),
                SExpChirho::AtomChirho(AtomChirho::StringChirho(content_chirho))) = (&args_chirho[0], &args_chirho[1]) {
            match fs::write(filename_chirho, content_chirho) {
                Ok(_) => Ok(SExpChirho::AtomChirho(AtomChirho::BooleanChirho(true))),
                Err(e_chirho) => Err(format!("Failed to write file: {}", e_chirho)),
            }
        } else {
            Err("Arguments to write-file must be strings".to_string())
        }
    })));

    env_chirho.set_chirho("file-exists-chirho?".to_string(), SExpChirho::AtomChirho(AtomChirho::BuiltInChirho(|args_chirho| {
        if args_chirho.len() != 1 {
            return Err("file-exists? requires exactly one argument".to_string());
        }
        if let SExpChirho::AtomChirho(AtomChirho::StringChirho(filename_chirho)) = &args_chirho[0] {
            Ok(SExpChirho::AtomChirho(AtomChirho::BooleanChirho(Path::new(filename_chirho).exists())))
        } else {
            Err("Argument to file-exists? must be a string".to_string())
        }
    })));

    env_chirho.set_chirho("create-dir-chirho".to_string(), SExpChirho::AtomChirho(AtomChirho::BuiltInChirho(|args_chirho| {
        if args_chirho.len() != 1 {
            return Err("create-dir requires exactly one argument".to_string());
        }
        if let SExpChirho::AtomChirho(AtomChirho::StringChirho(dirname_chirho)) = &args_chirho[0] {
            match fs::create_dir_all(dirname_chirho) {
                Ok(_) => Ok(SExpChirho::AtomChirho(AtomChirho::BooleanChirho(true))),
                Err(e_chirho) => Err(format!("Failed to create directory: {}", e_chirho)),
            }
        } else {
            Err("Argument to create-dir must be a string".to_string())
        }
    })));

    env_chirho.set_chirho("string-append-chirho".to_string(), SExpChirho::AtomChirho(AtomChirho::BuiltInChirho(|args_chirho| {
        let result_chirho = args_chirho.iter().filter_map(|arg_chirho| {
            if let SExpChirho::AtomChirho(AtomChirho::StringChirho(s_chirho)) = arg_chirho {
                Some(s_chirho.as_str())
            } else {
                None
            }
        }).collect::<Vec<&str>>().join("");
        Ok(SExpChirho::AtomChirho(AtomChirho::StringChirho(result_chirho)))
    })));

    env_chirho.set_chirho("string-chirho->number-chirho".to_string(), SExpChirho::AtomChirho(AtomChirho::BuiltInChirho(|args_chirho| {
        if args_chirho.len() != 1 {
            return Err("string->number requires exactly one argument".to_string());
        }
        if let SExpChirho::AtomChirho(AtomChirho::StringChirho(s_chirho)) = &args_chirho[0] {
            match s_chirho.parse::<f64>() {
                Ok(n_chirho) => Ok(SExpChirho::AtomChirho(AtomChirho::NumberChirho(n_chirho))),
                Err(_) => Err("Failed to parse string as number".to_string()),
            }
        } else {
            Err("Argument to string->number must be a string".to_string())
        }
    })));

    env_chirho.set_chirho("number-chirho->string-chirho".to_string(), SExpChirho::AtomChirho(AtomChirho::BuiltInChirho(|args_chirho| {
        if args_chirho.len() != 1 {
            return Err("number->string requires exactly one argument".to_string());
        }
        if let SExpChirho::AtomChirho(AtomChirho::NumberChirho(n_chirho)) = args_chirho[0] {
            Ok(SExpChirho::AtomChirho(AtomChirho::StringChirho(n_chirho.to_string())))
        } else {
            Err("Argument to number->string must be a number".to_string())
        }
    })));

    env_chirho.set_chirho("directory-list-chirho".to_string(), SExpChirho::AtomChirho(AtomChirho::BuiltInChirho(|args_chirho| {
        if args_chirho.len() != 1 {
            return Err("directory-list requires exactly one argument".to_string());
        }
        if let SExpChirho::AtomChirho(AtomChirho::StringChirho(dirname_chirho)) = &args_chirho[0] {
            match fs::read_dir(dirname_chirho) {
                Ok(entries_chirho) => {
                    let files_chirho: Result<Vec<String>, std::io::Error> = entries_chirho
                        .map(|res| res.map(|e| e.file_name().into_string().unwrap()))
                        .collect();
                    match files_chirho {
                        Ok(files_chirho) => Ok(SExpChirho::ListChirho(
                            files_chirho.into_iter()
                                .map(|f| SExpChirho::AtomChirho(AtomChirho::StringChirho(f)))
                                .collect()
                        )),
                        Err(e_chirho) => Err(format!("Failed to read directory: {}", e_chirho)),
                    }
                },
                Err(e_chirho) => Err(format!("Failed to read directory: {}", e_chirho)),
            }
        } else {
            Err("Argument to directory-list must be a string".to_string())
        }
    })));
}
