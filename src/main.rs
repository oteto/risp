use env::RispEnv;
use eval::eval;
use parser::RispParser;
use tokenizer::RispTokenizer;

mod env;
mod eval;
mod parser;
mod tokenizer;

#[derive(Clone)]
pub enum RispExp {
    Symbol(String),
    Number(f64),
    List(Vec<RispExp>),
    Func(fn(&[RispExp]) -> Result<RispExp, RispErr>),
}

#[derive(Debug)]
pub enum RispErr {
    Reason(String),
}

fn main() {
    let tokens = RispTokenizer::tokenize("(+ 10 (- 12 (+ 3 6) 1))".to_string());
    let mut env = RispEnv::default_env();

    match RispParser::parse(&tokens).and_then(|(exp, _)| eval(&exp, &mut env)) {
        Ok(ret) => match ret {
            RispExp::Number(v) => println!("{}", v),
            _ => eprintln!("failed to eval."),
        },
        Err(err) => {
            eprintln!("failed to parse. {:?}", err)
        }
    }
}
