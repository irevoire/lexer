use crate::Reader;
use crate::Keywords;
use crate::Types;

pub struct Token {
    id: usize,
    type: Types,

}

pub struct Tokenizer {
    reader: reader::Reader,
}

impl Tokenizer {
    pub fn new(reader: reader::Reader) -> Self {
        Tokenizer { reader }
    }

    pub fn get_token() -> Token {
        let word = self.reader.get_word();
        Types::get_type(word)
    }
}
