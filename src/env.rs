use std::collections::HashMap;

use crate::{error::RispErr, exp::RispExp};

#[derive(Clone)]
pub struct RispEnv {
    pub data: HashMap<String, RispExp>,
}

impl RispEnv {
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
        Self { data }
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
