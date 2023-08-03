use super::text_buffer::TextBuffer;
use std::{
    borrow::Cow,
    fs::File,
    io,
    ops::{Deref, DerefMut, Range},
};

/// A cursor with convenience functions for moving through a TextBuffer.
pub trait TextBufferCursor<'cursor, B: TextBuffer> {
    /// Set cursor position.
    fn set(&mut self, position: usize);

    /// Get cursor position.
    fn pos(&self) -> usize;

    /// Check if cursor position is at a codepoint boundary.
    fn is_boundary(&self) -> bool;

    /// Move cursor to previous codepoint boundary, if it exists.
    /// Returns previous codepoint as usize offset.
    fn prev(&mut self) -> Option<usize>;

    /// Move cursor to next codepoint boundary, if it exists.
    /// Returns current codepoint as usize offset.
    fn next(&mut self) -> Option<usize>;

    /// Get the next codepoint after the cursor position, without advancing
    /// the cursor.
    fn peek_next_codepoint(&self) -> Option<char>;

    /// Return codepoint preceding cursor offset and move cursor backward.
    fn prev_codepoint(&mut self) -> Option<char>;

    /// Return codepoint at cursor offset and move cursor forward.
    fn next_codepoint(&mut self) -> Option<char>;

    /// Return current offset if it's a boundary, else next.
    fn at_or_next(&mut self) -> Option<usize>;

    /// Return current offset if it's a boundary, else previous.
    fn at_or_prev(&mut self) -> Option<usize>;
}
