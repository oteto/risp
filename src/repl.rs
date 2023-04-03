use std::io::{self, Write};

use crate::{env::RispEnv, eval::eval, parser::RispParser, tokenizer::RispTokenizer};

pub fn repl(env: &mut RispEnv) {
    loop {
        let expr = prompt();
        let tokens = RispTokenizer::tokenize(expr);

        match RispParser::parse(&tokens).and_then(|(exp, _)| eval(&exp, env)) {
            Ok(exp) => println!("{}", exp),
            Err(err) => eprintln!("\x1b[31m[ERROR] => {:?}\x1b[0m", err),
        }
    }
}

fn prompt() -> String {
    let mut expr = String::new();
    print!("risp > ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut expr)
        .expect("Failed to read line");
    expr
}
