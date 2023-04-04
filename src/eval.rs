use std::collections::HashMap;

use crate::{
    env::RispEnv,
    error::RispErr,
    exp::{RispExp, RispLambda},
};

pub fn eval(exp: &RispExp, env: &mut RispEnv) -> Result<RispExp, RispErr> {
    match exp {
        RispExp::Symbol(s) => env
            .get(s)
            .ok_or(RispErr::Reason(format!("unexpected symbol `{}`", s)))
            .map(|x| x.clone()),
        RispExp::Bool(_) => Ok(exp.clone()),
        RispExp::Number(_) => Ok(exp.clone()),
        RispExp::List(list) => {
            let first_form = list
                .first()
                .ok_or(RispErr::Reason("expected a non-empty list".to_string()))?;
            let arg_forms = &list[1..];

            match eval_builtin_form(first_form, arg_forms, env) {
                Some(res) => res,
                None => {
                    let first_eval = eval(first_form, env)?;
                    match first_eval {
                        RispExp::Func(f) => f(&eval_forms(arg_forms, env)?),
                        RispExp::Lambda(lambda) => {
                            let new_env = &mut env_for_lambda(lambda.params_exp, arg_forms, env)?;
                            eval(&lambda.body_exp, new_env)
                        }
                        _ => Err(RispErr::Reason("first form must be a function".to_string())),
                    }
                }
            }
        }
        RispExp::Func(_) => Err(RispErr::Reason("unexpected form".to_string())),
        RispExp::Lambda(_) => Err(RispErr::Reason("unexpected form".to_string())),
    }
}

fn eval_builtin_form(
    exp: &RispExp,
    arg_forms: &[RispExp],
    env: &mut RispEnv,
) -> Option<Result<RispExp, RispErr>> {
    match exp {
        RispExp::Symbol(s) => match s.as_ref() {
            "if" => Some(eval_if_args(arg_forms, env)),
            "def" => Some(eval_def_args(arg_forms, env)),
            "fn" => Some(eval_lambda_args(arg_forms)),
            _ => None,
        },
        _ => None,
    }
}

fn eval_if_args(args: &[RispExp], env: &mut RispEnv) -> Result<RispExp, RispErr> {
    let test_form = args
        .first()
        .ok_or(RispErr::Reason("expected test form".to_string()))?;
    let test_eval = eval(test_form, env)?;
    match test_eval {
        RispExp::Bool(b) => {
            let form_idx = if b { 1 } else { 2 };
            let res_form = args
                .get(form_idx)
                .ok_or(RispErr::Reason(format!("expected form idx = {}", form_idx)))?;
            eval(res_form, env)
        }
        _ => Err(RispErr::Reason(format!(
            "unexpected test form = `{}`",
            test_form.to_string()
        ))),
    }
}

fn eval_def_args(args: &[RispExp], env: &mut RispEnv) -> Result<RispExp, RispErr> {
    if args.len() > 2 {
        return Err(RispErr::Reason("def can only 2 forms".to_string()));
    }

    let ident_form = args
        .first()
        .ok_or(RispErr::Reason("expected ident form".to_string()))?;
    let ident_str = match ident_form {
        RispExp::Symbol(s) => Ok(s.clone()),
        _ => Err(RispErr::Reason(
            "expected ident form to be a symbol".to_string(),
        )),
    }?;
    let value_form = args
        .get(1)
        .ok_or(RispErr::Reason("expected value form".to_string()))?;

    let value_exp = eval(value_form, env)?;
    env.data.insert(ident_str, value_exp.clone());
    Ok(value_exp)
}

fn eval_lambda_args(args: &[RispExp]) -> Result<RispExp, RispErr> {
    if args.len() < 2 {
        return Err(RispErr::Reason(
            "fn definition can only have two forms".to_string(),
        ));
    }
    let params_form = args
        .first()
        .ok_or(RispErr::Reason("expected args form".to_string()))?;
    let body_exp = args
        .get(1)
        .ok_or(RispErr::Reason("expected body form".to_string()))?;

    Ok(RispExp::Lambda(RispLambda {
        params_exp: Box::new(params_form.clone()),
        body_exp: Box::new(body_exp.clone()),
    }))
}

fn eval_forms(arg_forms: &[RispExp], env: &mut RispEnv) -> Result<Vec<RispExp>, RispErr> {
    arg_forms.iter().map(|x| eval(x, env)).collect()
}

fn env_for_lambda<'a>(
    params: Box<RispExp>,
    arg_forms: &[RispExp],
    outer_env: &'a mut RispEnv,
) -> Result<RispEnv<'a>, RispErr> {
    let ks = parse_list_of_symbol_string(params)?;
    if ks.len() != arg_forms.len() {
        return Err(RispErr::Reason(format!(
            "expected {} arguments, got {}",
            ks.len(),
            arg_forms.len()
        )));
    }

    let vs = eval_forms(arg_forms, outer_env)?;
    let mut data = HashMap::new();
    for (k, v) in ks.iter().zip(vs.iter()) {
        data.insert(k.clone(), v.clone());
    }

    Ok(RispEnv {
        data,
        outer: Some(outer_env),
    })
}

fn parse_list_of_symbol_string(form: Box<RispExp>) -> Result<Vec<String>, RispErr> {
    let list = match form.as_ref() {
        RispExp::List(s) => Ok(s.clone()),
        _ => Err(RispErr::Reason(
            "expected args form to be a list".to_string(),
        )),
    }?;

    list.iter()
        .map(|x| match x {
            RispExp::Symbol(s) => Ok(s.clone()),
            _ => Err(RispErr::Reason(
                "expected symbols in the argument list".to_string(),
            )),
        })
        .collect()
}
