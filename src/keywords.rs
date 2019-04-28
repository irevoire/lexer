pub enum Keywords {
    Function,
    If,
    Else,
    While,
    For,
    Do,
}

pub fn parse(word: &str) -> Option<Keywords> {
    match word {
        "def" => Some(Keywords::Function),
        "if" => Some(Keywords::If),
        "else" => Some(Keywords::Else),
        "while" => Some(Keywords::While),
        "for" => Some(Keywords::For),
        "do" => Some(Keywords::Do),
        _ => None,
    }
}
