use crate::keywords;
use crate::reader;
use crate::types;

pub struct Token {
    value: types::Types,
}

pub struct Tokenizer {
    reader: reader::Reader,
}

impl Tokenizer {
    pub fn new(reader: reader::Reader) -> Self {
        Tokenizer { reader }
    }

    pub fn next(&mut self) -> Option<Token> {
        self.reader.skip_separator();
        let word = self.reader.get_word();
        let value = types::get_type(&word);
        Some(Token { value })
    }
}
