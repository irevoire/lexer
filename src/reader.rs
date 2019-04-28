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

    /// Define what should separate each word in the langage
    /// At this time it's ' ', '\n', '\n'
    fn is_separator(c: char) -> bool {
        c == ' ' || c == '\t' || c == '\n'
    }

    /// Return a word between the current position and the next separator
    /// See the is_separator function to see when it stop
    /// TODO This function do a copy of the word. It could probably do it
    /// in place with a better way to handle the buffer
    pub fn get_word(&mut self) -> String {
        let start = self.cursor;

        while let Some(c) = self.next() {
            if Reader::is_separator(*c) {
                self.prev();
                break;
            }
        }
        self.buf[start..self.cursor].iter().collect::<String>()
    }

    pub fn skip_separator(&mut self) -> &mut Self {
        while let Some(c) = self.next() {
            if Reader::is_separator(*c) {
                continue;
            }
            self.prev(); // to get back on the last non shitty chr
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
    fn test_next_utf8() {
        let mut reader = init("a😘b🥰");
        reader.next();
        assert_eq!(reader.next(), Some(&'😘'));
        assert_eq!(reader.cursor, 2);
        reader.next();
        assert_eq!(reader.next(), Some(&'🥰'));
        assert_eq!(reader.cursor, 4);
        assert_eq!(reader.next(), None);
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
    fn test_prev_utf8() {
        let mut reader = init("a😘b🥰");
        reader.skip(4);
        assert_eq!(reader.prev(), Some(&'🥰'));
        assert_eq!(reader.cursor, 3);
        reader.prev();
        assert_eq!(reader.prev(), Some(&'😘'));
        assert_eq!(reader.cursor, 1);
        reader.prev();
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
    fn test_get_word() {
        let mut reader = init("Hello Word!");
        assert_eq!(reader.get_word(), "Hello");
        assert_eq!(reader.next(), Some(&' '));
        assert_eq!(reader.get_word(), "Word!");
        assert_eq!(reader.next(), None);
    }

    #[test]
    fn test_skip_separator() {
        let mut reader = init("   a    bc  ");
        reader.skip_separator();
        assert_eq!(reader.next(), Some(&'a'));
        reader.skip_separator();
        assert_eq!(reader.next(), Some(&'b'));
        reader.skip_separator();
        assert_eq!(reader.next(), Some(&'c'));
        reader.skip_separator();
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
