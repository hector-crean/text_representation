use crate::api::{text_buffer::TextBuffer, text_buffer_cursor::TextBufferCursor};

#[derive(Debug)]
pub struct StringCursor<'a> {
    pub text: &'a str,
    pub position: usize,
}

impl<'a> StringCursor<'a> {
    /// Create a new cursor.
    pub fn new(text: &'a str) -> Self {
        Self { text, position: 0 }
    }
}

impl<'cursor> TextBufferCursor<'cursor, String> for StringCursor<'cursor> {
    fn set(&mut self, position: usize) {
        self.position = position;
    }

    fn pos(&self) -> usize {
        self.position
    }

    fn is_boundary(&self) -> bool {
        self.text.is_char_boundary(self.position)
    }

    fn prev(&mut self) -> Option<usize> {
        let current_pos = self.pos();

        if current_pos == 0 {
            None
        } else {
            let mut len = 1;
            while !self.text.is_char_boundary(current_pos - len) {
                len += 1;
            }
            self.set(self.pos() - len);
            Some(self.pos())
        }
    }

    fn next(&mut self) -> Option<usize> {
        let current_pos = self.pos();

        if current_pos == self.text.len() {
            None
        } else {
            let b = self.text.as_bytes()[current_pos];
            self.set(current_pos + len_utf8_from_first_byte(b));
            Some(current_pos)
        }
    }

    fn peek_next_codepoint(&self) -> Option<char> {
        self.text[self.pos()..].chars().next()
    }

    fn prev_codepoint(&mut self) -> Option<char> {
        if let Some(prev) = self.prev() {
            self.text[prev..].chars().next()
        } else {
            None
        }
    }

    fn next_codepoint(&mut self) -> Option<char> {
        let current_index = self.pos();
        if self.next().is_some() {
            self.text[current_index..].chars().next()
        } else {
            None
        }
    }

    fn at_or_next(&mut self) -> Option<usize> {
        if self.is_boundary() {
            Some(self.pos())
        } else {
            self.next()
        }
    }

    fn at_or_prev(&mut self) -> Option<usize> {
        if self.is_boundary() {
            Some(self.pos())
        } else {
            self.prev()
        }
    }
}

pub fn len_utf8_from_first_byte(b: u8) -> usize {
    match b {
        b if b < 0x80 => 1,
        b if b < 0xe0 => 2,
        b if b < 0xf0 => 3,
        _ => 4,
    }
}
