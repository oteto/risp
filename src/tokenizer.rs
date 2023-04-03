pub struct RispTokenizer {}

impl RispTokenizer {
    pub fn tokenize(expr: String) -> Vec<String> {
        expr.replace("(", " ( ")
            .replace(")", " ) ")
            .split_whitespace()
            .map(|c| c.to_string())
            .collect()
    }
}

#[cfg(test)]
mod tokenizer_tests {
    use crate::tokenizer::RispTokenizer;

    #[test]
    fn test_tokenize() {
        assert_eq!(
            RispTokenizer::tokenize("(+ 10 5)".to_string()),
            vec![
                "(".to_string(),
                "+".to_string(),
                "10".to_string(),
                "5".to_string(),
                ")".to_string()
            ]
        )
    }
}
