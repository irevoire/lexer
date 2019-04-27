pub struct Reader {
    buf: Vec<char>,
    cursor: usize,
}

impl Reader {
    pub fn new<R: std::io::Read>(reader: &mut R) -> Self {
        let mut buf = String::new();
        reader.read_to_string(&mut buf).unwrap(); // if fail we stop everything
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

    pub fn start(&mut self) -> &mut Self {
        self.cursor = 0;
        self
    }

    pub fn end(&mut self) -> &mut Self {
        self.cursor = self.buf.len();
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init(s: &str) -> Reader {
        let mut reader = std::io::Cursor::new(s);
        Reader::new(&mut reader)
    }

    #[test]
    fn test_next() {
        let mut reader = init("ab");
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
        let mut reader = init("");
        assert_eq!(reader.next(), None);
        assert_eq!(reader.cursor, 0);
    }

    #[test]
    fn test_prev() {
        let mut reader = init("ab");
        assert_eq!(reader.prev(), None);
    }

    #[test]
    fn test_end_and_prev() {
        let mut reader = init("ab");
        reader.end();
        assert_eq!(reader.prev(), Some(&'b'));
        assert_eq!(reader.prev(), Some(&'a'));
        assert_eq!(reader.prev(), None);
    }

    #[test]
    fn test_empty_prev() {
        let mut reader = init("");
        assert_eq!(reader.prev(), None);
        assert_eq!(reader.cursor, 0);
    }

    #[test]
    fn test_next_and_prev() {
        let mut reader = init("ab");
        assert_eq!(reader.cursor, 0);
        assert_eq!(reader.next(), Some(&'a'));
        assert_eq!(reader.cursor, 1);
        assert_eq!(reader.next(), Some(&'b'));
        assert_eq!(reader.cursor, 2);
        assert_eq!(reader.next(), None);
        assert_eq!(reader.cursor, 2);

        assert_eq!(reader.prev(), Some(&'b'));
        assert_eq!(reader.cursor, 1);
        assert_eq!(reader.prev(), Some(&'a'));
        assert_eq!(reader.cursor, 0);
        assert_eq!(reader.prev(), None);
        assert_eq!(reader.cursor, 0);
    }

    #[test]
    fn test_start_and_end() {
        let mut reader = init("ab");
        assert_eq!(reader.cursor, 0);
        reader.end();
        assert_eq!(reader.cursor, 2);
        reader.start();
        assert_eq!(reader.cursor, 0);
    }
}
