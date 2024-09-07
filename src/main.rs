/*
 * For God so loved the world, that He gave His only begotten Son, that all who believe in Him should not perish but have everlasting life.
 */

mod env_chirho;
mod eval_chirho;
mod parse_chirho;

#[cfg(target_arch = "wasm32")]
mod wasm_repl_chirho;

#[cfg(not(target_arch = "wasm32"))]
mod repl_chirho;
mod sexp_chirho;
mod built_in_chirho;

#[cfg(not(target_arch = "wasm32"))]
use repl_chirho::repl_chirho;

#[cfg(target_arch = "wasm32")]
use wasm_repl_chirho::log;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    println!("Hallelujah, world!");
    repl_chirho();
}

#[cfg(target_arch = "wasm32")]
fn main() {
    console_error_panic_hook::set_once();
    log("S-Expression Interpreter initialized");
}
