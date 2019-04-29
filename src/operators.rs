#[derive(Debug)]
pub enum Operators {
    Plus,
    Minus,
    Mul,
    Div,
    PlusPlus,
    Equal,
    PlusEqual,
    MinusEqual,
}

pub fn parse(word: &str) -> Option<Operators> {
    match word {
        "+" => Some(Operators::Plus),
        "-" => Some(Operators::Minus),
        "*" => Some(Operators::Mul),
        "/" => Some(Operators::Div),
        "++" => Some(Operators::PlusPlus),
        "=" => Some(Operators::Equal),
        "+=" => Some(Operators::PlusEqual),
        "-=" => Some(Operators::MinusEqual),
        _ => None,
    }
}
