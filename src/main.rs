mod env;
mod error;
mod eval;
mod exp;
mod parser;
mod repl;
mod tokenizer;

fn main() {
    repl::repl(&mut env::RispEnv::default_env());
}
