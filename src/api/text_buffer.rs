use super::text_buffer_cursor::TextBufferCursor;
use std::{
    borrow::Cow,
    fs::File,
    io,
    ops::{Deref, DerefMut, Range},
};

pub trait TextBuffer: Sized {
    type Cursor<'cursor>: TextBufferCursor<'cursor, Self>
    where
        Self: 'cursor;

    /// Create a cursor with a reference to the text and a offset position.
    ///
    /// Returns None if the position isn't a codepoint boundary.
    fn cursor<'cursor>(&'cursor self, position: usize) -> Option<Self::Cursor<'cursor>>;

    /// Replace range with new text.
    /// Can panic if supplied an invalid range.
    // TODO: make this generic over Self
    // fn edit(&mut self, range: Range<usize>, new: impl Into<String>);

    /// Get slice of text at range.
    fn slice(&self, range: Range<usize>) -> Option<Cow<str>>;

    /// Get length of text (in bytes).
    fn len(&self) -> usize;

    /// Get the previous word offset from the given offset, if it exists.
    fn prev_word_offset(&self, offset: usize) -> Option<usize>;

    /// Get the next word offset from the given offset, if it exists.
    fn next_word_offset(&self, offset: usize) -> Option<usize>;

    /// Get the next grapheme offset from the given offset, if it exists.
    fn prev_grapheme_offset(&self, offset: usize) -> Option<usize>;

    /// Get the next grapheme offset from the given offset, if it exists.
    fn next_grapheme_offset(&self, offset: usize) -> Option<usize>;

    /// Get the previous codepoint offset from the given offset, if it exists.
    fn prev_codepoint_offset(&self, offset: usize) -> Option<usize>;

    /// Get the next codepoint offset from the given offset, if it exists.
    fn next_codepoint_offset(&self, offset: usize) -> Option<usize>;

    /// Get the preceding line break offset from the given offset
    fn preceding_line_break(&self, offset: usize) -> usize;

    /// Get the next line break offset from the given offset
    fn next_line_break(&self, offset: usize) -> usize;

    /// Returns `true` if this text has 0 length.
    fn is_empty(&self) -> bool;

    /// Construct an instance of this type from a `&str`.
    fn from_str(s: &str) -> Self;
}
