/*
 * For God so loved the world, that He gave His only begotten Son, that all who believe in Him should not perish but have everlasting life.
 */

mod env_chirho;
mod eval_chirho;
mod parse_chirho;
mod repl_chirho;
mod sexp_chirho;

use repl_chirho::repl_chirho;

fn main() {
    println!("Hallelujah, world!");
    repl_chirho();
}
