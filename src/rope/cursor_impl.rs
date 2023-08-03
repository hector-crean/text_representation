use ropey::Rope;

use crate::api::{text_buffer::TextBuffer, text_buffer_cursor::TextBufferCursor};

#[derive(Debug)]
pub struct RopeCursor<'a> {
    rope: &'a Rope,
    cursor: usize,
}

impl<'a> RopeCursor<'a> {
    /// Create a new cursor.
    pub fn new(text: &'a str) -> Self {
        todo!()
    }
}

impl<'cursor> TextBufferCursor<'cursor, Rope> for RopeCursor<'cursor> {
    fn set(&mut self, position: usize) {
        self.cursor = position;
    }

    fn pos(&self) -> usize {
        self.cursor
    }

    fn is_boundary(&self) -> bool {
        todo!()
    }

    fn prev(&mut self) -> Option<usize> {
        // let prev_boundary = self.rope.prev_codepoint_boundary(self.cursor);
        // if let Some(offset) = prev_boundary {
        //     self.cursor = offset;
        // }
        // prev_boundary
        todo!()
    }

    fn next(&mut self) -> Option<usize> {
        // let next_boundary = self.rope.next_codepoint_boundary(self.cursor);
        // if let Some(offset) = next_boundary {
        //     self.cursor = offset;
        // }
        // next_boundary
        todo!()
    }

    fn peek_next_codepoint(&self) -> Option<char> {
        if self.cursor < self.rope.len_chars() {
            Some(self.rope.char(self.cursor))
        } else {
            None
        }
    }

    fn prev_codepoint(&mut self) -> Option<char> {
        if let Some(offset) = self.prev() {
            Some(self.rope.char(offset))
        } else {
            None
        }
    }

    fn next_codepoint(&mut self) -> Option<char> {
        let current_codepoint = self.peek_next_codepoint();
        self.next();
        current_codepoint
    }

    fn at_or_next(&mut self) -> Option<usize> {
        if self.is_boundary() {
            Some(self.cursor)
        } else {
            self.next()
        }
    }

    fn at_or_prev(&mut self) -> Option<usize> {
        if self.is_boundary() {
            Some(self.cursor)
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
