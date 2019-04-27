pub struct Reader {
    buf: Vec<char>,
    cursor: usize,
}

impl Reader {
    pub fn new<R: std::io::Read>(reader: &mut R) -> Self {
        let mut buf = String::new();
        reader.read_to_string(&mut buf);
        Reader {
            buf: buf.chars().collect(),
            cursor: 0,
        }
    }

    pub fn next(&mut self) -> Option<&char> {
        if let Some(c) = self.buf.get(self.cursor) {
            self.cursor += 1;
            return Some(c);
        }
        None
    }

    pub fn prev(&mut self) -> Option<&char> {
        self.cursor = self.cursor.checked_sub(1)?;
        self.buf.get(self.cursor)
    }

    pub fn end(&mut self) -> &mut Self {
        self.cursor = self.buf.len();
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next() {
        let mut reader = std::io::Cursor::new("ab");
        let mut reader = Reader::new(&mut reader);

        assert_eq!(reader.cursor, 0);
        assert_eq!(reader.next(), Some(&'a'));
        assert_eq!(reader.cursor, 1);
        assert_eq!(reader.next(), Some(&'b'));
        assert_eq!(reader.cursor, 2);
        assert_eq!(reader.next(), None);
        assert_eq!(reader.cursor, 2);
    }

    #[test]
    fn test_empty_next() {
        let mut reader = std::io::Cursor::new("");
        let mut reader = Reader::new(&mut reader);

        assert_eq!(reader.next(), None);
        assert_eq!(reader.cursor, 0);
    }

    #[test]
    fn test_end_and_prev() {
        let mut reader = std::io::Cursor::new("ab");
        let mut reader = Reader::new(&mut reader);

        reader.end();
        assert_eq!(reader.prev(), Some(&'b'));
        assert_eq!(reader.prev(), Some(&'a'));
        assert_eq!(reader.prev(), None);
    }

    #[test]
    fn test_empty_prev() {
        let mut reader = std::io::Cursor::new("");
        let mut reader = Reader::new(&mut reader);

        assert_eq!(reader.prev(), None);
        assert_eq!(reader.cursor, 0);
    }
}
