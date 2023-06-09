use crate::{error::RispErr, exp::RispExp};

pub struct RispParser {}

impl RispParser {
    pub fn parse<'a>(tokens: &'a [String]) -> Result<(RispExp, &'a [String]), RispErr> {
        let (token, rest) = tokens
            .split_first()
            .ok_or(RispErr::Reason("could not get token".to_string()))?;

        match &token[..] {
            "(" => Self::read_req(rest),
            ")" => Err(RispErr::Reason("unexpected `)`".to_string())),
            _ => Ok((Self::parse_atom(token), rest)),
        }
    }

    fn read_req<'a>(tokens: &'a [String]) -> Result<(RispExp, &'a [String]), RispErr> {
        let mut res = vec![];
        let mut xs = tokens;

        loop {
            let (next_token, rest) = xs
                .split_first()
                .ok_or(RispErr::Reason("could not find closing `)`".to_string()))?;
            if next_token == ")" {
                return Ok((RispExp::List(res), rest));
            }
            let (exp, new_xs) = Self::parse(xs)?;
            res.push(exp);
            xs = new_xs;
        }
    }

    fn parse_atom(token: &str) -> RispExp {
        match token {
            "true" => RispExp::Bool(true),
            "false" => RispExp::Bool(false),
            _ => {
                let potential_float = token.parse::<f64>();
                match potential_float {
                    Ok(v) => RispExp::Number(v),
                    Err(_) => RispExp::Symbol(token.to_string()),
                }
            }
        }
    }
}

#[cfg(test)]
mod parser_tests {

    use crate::exp::RispExp;

    use super::RispParser;

    #[test]
    fn test_parse_add() {
        let tokens = vec!["(", "+", "10", "5", ")"]
            .iter()
            .map(|t| t.to_string())
            .collect::<Vec<String>>();

        let (exp, rest) = RispParser::parse(&tokens).unwrap();

        assert_eq!(rest.len(), 0);
        match exp {
            RispExp::List(list) => match (&list[0], &list[1], &list[2]) {
                (RispExp::Symbol(s), RispExp::Number(a), RispExp::Number(b)) => {
                    assert_eq!(s, "+");
                    assert_eq!(*a, 10.0);
                    assert_eq!(*b, 5.0);
                }
                _ => assert!(false),
            },
            _ => assert!(false),
        }
    }

    #[test]
    fn test_parse_bool() {
        let tokens = vec!["(", "true", "false", ")"]
            .iter()
            .map(|t| t.to_string())
            .collect::<Vec<String>>();
        let (exp, _) = RispParser::parse(&tokens).unwrap();
        match exp {
            RispExp::List(list) => match (&list[0], &list[1]) {
                (RispExp::Bool(t), RispExp::Bool(f)) => {
                    assert_eq!(*t, true);
                    assert_eq!(*f, false);
                }
                _ => assert!(false),
            },
            _ => assert!(false),
        }
    }

    #[test]
    fn test_parse_fn() {
        let tokens = vec![
            "(", "fn", "add", "(", "a", "b", ")", "(", "+", "a", "b", ")", ")",
        ]
        .iter()
        .map(|t| t.to_string())
        .collect::<Vec<String>>();

        let (exp, _) = RispParser::parse(&tokens).unwrap();
        match exp {
            RispExp::List(list) => {
                match &list[0] {
                    RispExp::Symbol(f) => assert_eq!(f, "fn"),
                    _ => assert!(false),
                }
                match &list[1] {
                    RispExp::Symbol(f) => assert_eq!(f, "add"),
                    _ => assert!(false),
                }
                match &list[2] {
                    RispExp::List(args) => match (&args[0], &args[1]) {
                        (RispExp::Symbol(a), RispExp::Symbol(b)) => {
                            assert_eq!(a, "a");
                            assert_eq!(b, "b");
                        }
                        _ => assert!(false),
                    },
                    _ => assert!(false),
                }
                match &list[3] {
                    RispExp::List(body) => match (&body[0], &body[1], &body[2]) {
                        (RispExp::Symbol(o), RispExp::Symbol(a), RispExp::Symbol(b)) => {
                            assert_eq!(o, "+");
                            assert_eq!(a, "a");
                            assert_eq!(b, "b");
                        }
                        _ => assert!(false),
                    },
                    _ => assert!(false),
                }
            }
            _ => assert!(false),
        }
    }
}
