use std::collections::HashMap;

use crate::{error::RispErr, exp::RispExp};

#[derive(Clone)]
pub struct RispEnv<'a> {
    pub data: HashMap<String, RispExp>,
    pub outer: Option<&'a RispEnv<'a>>,
}

impl<'a> RispEnv<'a> {
    pub fn get(&self, key: &str) -> Option<RispExp> {
        match self.data.get(key) {
            Some(exp) => Some(exp.clone()),
            None => match self.outer {
                Some(outer_env) => outer_env.get(key),
                None => None,
            },
        }
    }

    pub fn default_env() -> Self {
        let mut data = HashMap::new();
        data.insert(
            "+".to_string(),
            RispExp::Func(|args| {
                let sum = parse_list_of_floats(args)?
                    .iter()
                    .fold(0.0, |sum, v| sum + v);
                Ok(RispExp::Number(sum))
            }),
        );
        data.insert(
            "-".to_string(),
            RispExp::Func(|args| {
                let floats = parse_list_of_floats(args)?;
                let first = floats
                    .first()
                    .ok_or(RispErr::Reason("expected at latest one number".to_string()))?;
                let sum_of_rest = floats[1..].iter().fold(0.0, |sum, v| sum + v);
                Ok(RispExp::Number(*first - sum_of_rest))
            }),
        );
        data.insert(
            "=".to_string(),
            RispExp::Func(crate::ensure_tonicity!(|a, b| a == b)),
        );
        data.insert(
            ">".to_string(),
            RispExp::Func(crate::ensure_tonicity!(|a, b| a > b)),
        );
        data.insert(
            ">=".to_string(),
            RispExp::Func(crate::ensure_tonicity!(|a, b| a >= b)),
        );
        data.insert(
            "<".to_string(),
            RispExp::Func(crate::ensure_tonicity!(|a, b| a < b)),
        );
        data.insert(
            "<=".to_string(),
            RispExp::Func(crate::ensure_tonicity!(|a, b| a <= b)),
        );
        Self { data, outer: None }
    }
}

fn parse_list_of_floats(args: &[RispExp]) -> Result<Vec<f64>, RispErr> {
    args.iter().map(parse_single_float).collect()
}

fn parse_single_float(exp: &RispExp) -> Result<f64, RispErr> {
    match exp {
        RispExp::Number(v) => Ok(*v),
        _ => Err(RispErr::Reason("expected a number".to_string())),
    }
}

#[macro_export]
macro_rules! ensure_tonicity {
    ($check_fn:expr) => {{
        |args: &[RispExp]| -> Result<RispExp, RispErr> {
            let floats = parse_list_of_floats(args)?;
            let first = floats
                .first()
                .ok_or(RispErr::Reason("expected at latest one number".to_string()))?;
            let rest = &floats[1..];
            fn f(prev: &f64, xs: &[f64]) -> bool {
                match xs.first() {
                    Some(x) => $check_fn(prev, x) && f(x, &xs[1..]),
                    None => true,
                }
            }
            Ok(RispExp::Bool(f(first, rest)))
        }
    }};
}
