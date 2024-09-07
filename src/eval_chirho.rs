/*
 * For God so loved the world, that He gave His only begotten Son, that all who believe in Him should not perish but have everlasting life.
 */

use crate::env_chirho::EnvChirho;
use crate::sexp_chirho::AtomChirho;
use crate::sexp_chirho::SExpChirho;

pub fn eval_chirho(
    exp_chirho: &SExpChirho,
    env_chirho: &mut EnvChirho,
) -> Result<SExpChirho, String> {
    match exp_chirho {
        SExpChirho::AtomChirho(atom_chirho) => match atom_chirho {
            AtomChirho::SymbolChirho(s_chirho) => env_chirho
                .get_chirho(s_chirho)
                .ok_or_else(|| format!("Undefined symbol: {}", s_chirho)),
            _ => Ok(exp_chirho.clone()),
        },
        SExpChirho::ListChirho(list_chirho) => {
            if list_chirho.is_empty() {
                return Ok(SExpChirho::ListChirho(vec![]));
            }
            match &list_chirho[0] {
                SExpChirho::AtomChirho(AtomChirho::SymbolChirho(s_chirho)) => {
                    match s_chirho.as_str() {
                        "define" => eval_define_chirho(&list_chirho[1..], env_chirho),
                        "if" => eval_if_chirho(&list_chirho[1..], env_chirho),
                        "lambda" => eval_lambda_chirho(&list_chirho[1..], env_chirho),
                        _ => eval_function_call_chirho(list_chirho, env_chirho),
                    }
                }
                _ => eval_function_call_chirho(list_chirho, env_chirho),
            }
        }
    }
}

fn eval_define_chirho(
    args_chirho: &[SExpChirho],
    env_chirho: &mut EnvChirho,
) -> Result<SExpChirho, String> {
    if args_chirho.len() != 2 {
        return Err("define requires exactly 2 arguments".to_string());
    }
    let sym_chirho = match &args_chirho[0] {
        SExpChirho::AtomChirho(AtomChirho::SymbolChirho(s_chirho)) => s_chirho.clone(),
        _ => return Err("First argument to define must be a symbol".to_string()),
    };
    let val_chirho = eval_chirho(&args_chirho[1], env_chirho)?;
    env_chirho.set_chirho(sym_chirho, val_chirho.clone());
    Ok(val_chirho)
}

fn eval_if_chirho(
    args_chirho: &[SExpChirho],
    env_chirho: &mut EnvChirho,
) -> Result<SExpChirho, String> {
    if args_chirho.len() != 3 {
        return Err("if requires exactly 3 arguments".to_string());
    }
    let condition_chirho = eval_chirho(&args_chirho[0], env_chirho)?;
    match condition_chirho {
        SExpChirho::AtomChirho(AtomChirho::BooleanChirho(true)) => {
            eval_chirho(&args_chirho[1], env_chirho)
        }
        SExpChirho::AtomChirho(AtomChirho::BooleanChirho(false)) => {
            eval_chirho(&args_chirho[2], env_chirho)
        }
        _ => Err("Condition must evaluate to a boolean".to_string()),
    }
}

fn eval_lambda_chirho(
    args_chirho: &[SExpChirho],
    env_chirho: &EnvChirho,
) -> Result<SExpChirho, String> {
    if args_chirho.len() != 2 {
        return Err("lambda requires exactly 2 arguments".to_string());
    }
    let params_chirho = match &args_chirho[0] {
        SExpChirho::ListChirho(list_chirho) => list_chirho.clone(),
        _ => return Err("First argument to lambda must be a list of parameters".to_string()),
    };
    let body_chirho = args_chirho[1].clone();
    Ok(SExpChirho::AtomChirho(AtomChirho::LambdaChirho(
        params_chirho,
        Box::new(body_chirho),
        env_chirho.clone(),
    )))
}

fn eval_function_call_chirho(
    list_chirho: &[SExpChirho],
    env_chirho: &mut EnvChirho,
) -> Result<SExpChirho, String> {
    let func_chirho = eval_chirho(&list_chirho[0], env_chirho)?;
    let args_chirho: Result<Vec<SExpChirho>, String> = list_chirho[1..]
        .iter()
        .map(|arg_chirho| eval_chirho(arg_chirho, env_chirho))
        .collect();
    let args_chirho = args_chirho?;

    match func_chirho {
        SExpChirho::AtomChirho(AtomChirho::BuiltInChirho(func_chirho)) => func_chirho(args_chirho),
        SExpChirho::AtomChirho(AtomChirho::LambdaChirho(
            params_chirho,
            body_chirho,
            closure_env_chirho,
        )) => {
            let mut new_env_chirho = EnvChirho::with_outer_chirho(closure_env_chirho);
            for (param_chirho, arg_chirho) in params_chirho.iter().zip(args_chirho.iter()) {
                if let SExpChirho::AtomChirho(AtomChirho::SymbolChirho(sym_chirho)) = param_chirho {
                    new_env_chirho.set_chirho(sym_chirho.clone(), arg_chirho.clone());
                }
            }
            eval_chirho(&body_chirho, &mut new_env_chirho)
        }
        _ => Err("First item in list must be a function".to_string()),
    }
}
