pub struct Reader {
    buf: Vec<char>,
    cursor: usize,
    line_number: usize,
}

impl Reader {
    pub fn new<R: std::io::Read>(reader: &mut R) -> Self {
        let mut buf = String::new();
        reader.read_to_string(&mut buf).unwrap(); // if fail we stop everything
        Reader {
            buf: buf.chars().collect(),
            cursor: 0,
            line_number: 0,
        }
    }

    pub fn next(&mut self) -> Option<&char> {
        if let Some(c) = self.buf.get(self.cursor) {
            if *c == '\n' {
                self.line_number += 1;
            }
            self.cursor += 1;
            return Some(c);
        }
        None
    }

    pub fn skip(&mut self, n: usize) -> &mut Self {
        for _ in 0..n {
            self.next();
        }
        self
    }

    pub fn prev(&mut self) -> Option<&char> {
        self.cursor = self.cursor.checked_sub(1)?;
        if let Some(c) = self.buf.get(self.cursor) {
            if *c == '\n' {
                self.line_number -= 1;
            }
            return Some(c);
        }
        None
    }

    /// This function break the line counter
    pub fn start(&mut self) -> &mut Self {
        self.cursor = 0;
        self
    }

    /// This function break the line counter
    pub fn end(&mut self) -> &mut Self {
        self.cursor = self.buf.len();
        self
    }

    pub fn skip_space(&mut self) -> &mut Self {
        while let Some(c) = self.next() {
            if c == &' ' || c == &'\t' || c == &'\n' {
                continue;
            }
            self.prev(); // to get back on the last non shit chr
            break;
        }
        self
    }

    pub fn line_number(&self) -> usize {
        self.line_number
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

    #[test]
    fn test_skip_space() {
        let mut reader = init("   a    bc  ");
        reader.skip_space();
        assert_eq!(reader.next(), Some(&'a'));
        reader.skip_space();
        assert_eq!(reader.next(), Some(&'b'));
        reader.skip_space();
        assert_eq!(reader.next(), Some(&'c'));
        reader.skip_space();
        assert_eq!(reader.next(), None);
    }

    #[test]
    fn test_skip() {
        let mut reader = init("abcdef");
        reader.skip(1);
        assert_eq!(reader.next(), Some(&'b'));
        reader.skip(2);
        assert_eq!(reader.next(), Some(&'e'));
        reader.skip(10);
        assert_eq!(reader.next(), None);
    }

    #[test]
    fn test_line_number_next() {
        let mut reader = init("a\nc");
        assert_eq!(reader.line_number(), 0);
        reader.next();
        assert_eq!(reader.line_number(), 0);
        reader.next();
        assert_eq!(reader.line_number(), 1);
        reader.next();
        assert_eq!(reader.line_number(), 1);
    }

    #[test]
    fn test_line_number_prev() {
        let mut reader = init("a\nc");
        reader.next();
        reader.next(); // line number should be one
        reader.prev(); // we should go back to zero
        assert_eq!(reader.line_number(), 0);
        reader.prev();
        assert_eq!(reader.line_number(), 0);
    }
}
