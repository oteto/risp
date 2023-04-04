use core::fmt;

use crate::error::RispErr;

#[derive(Clone)]
pub enum RispExp {
    Symbol(String),
    Bool(bool),
    Number(f64),
    List(Vec<RispExp>),
    Func(fn(&[RispExp]) -> Result<RispExp, RispErr>),
    Lambda(RispLambda),
}

#[derive(Clone)]
pub struct RispLambda {
    pub params_exp: Box<RispExp>,
    pub body_exp: Box<RispExp>,
}

impl fmt::Display for RispExp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = match self {
            RispExp::Symbol(s) => s.clone(),
            RispExp::Bool(b) => b.to_string(),
            RispExp::Number(n) => n.to_string(),
            RispExp::List(list) => {
                let xs = list.iter().map(|x| x.to_string()).collect::<Vec<String>>();
                format!("({})", xs.join(","))
            }
            RispExp::Func(_) => "Function {}".to_string(),
            RispExp::Lambda(_) => "Lambda {}".to_string(),
        };
        write!(f, "{}", str)
    }
}
