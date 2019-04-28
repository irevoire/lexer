use crate::keywords;
use crate::operators;

pub enum Types {
    Nil,
    Number(String),
    String(String),
    Keyword(keywords::Keywords),
    Operator(operators::Operators),
    Value(String),
}

impl Types {
    pub fn get_type(word: &str) -> Types {
        assert!(!word.is_empty());
        let word = word.to_string();
        if word == "nil" {
            return Types::Nil;
        }
        // unwrap should be safe since `word` is not empty
        if word.chars().next().unwrap().is_digit(10) {
            return Types::Number(word);
        }
        if let Some(k) = keywords::parse(&word) {
            return Types::Keyword(k);
        }
        if let Some(op) = operators::parse(&word) {
            return Types::Operator(op);
        }
        Types::Value(word)
    }
}
