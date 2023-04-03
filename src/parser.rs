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
        let potential_float = token.parse::<f64>();
        match potential_float {
            Ok(v) => RispExp::Number(v),
            Err(_) => RispExp::Symbol(token.to_string()),
        }
    }
}

#[cfg(test)]
mod parser_tests {

    use crate::exp::RispExp;

    use super::RispParser;

    #[test]
    fn test_parse() {
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
}
