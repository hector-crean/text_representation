//! This example shows how to implement a grapeheme iterator over the contents
//! of a `Rope` or `RopeSlice`.  This also serves as a good starting point for
//! iterators for other kinds of segementation, such as word boundaries.

#![allow(clippy::redundant_field_names)]
#![allow(dead_code)]

// use std::str::pattern::Pattern;
use core::cmp;
use unicode_segmentation::{GraphemeCursor, GraphemeIncomplete};

#[derive(Clone, Debug)]
pub struct Graphemes<'a> {
    string: &'a str,
    cursor: GraphemeCursor,
    cursor_back: GraphemeCursor,
}

impl<'a> Graphemes<'a> {
    #[inline]
    /// View the underlying data (the part yet to be iterated) as a slice of the original string.
    ///
    /// ```rust
    /// # use unicode_segmentation::UnicodeSegmentation;
    /// let mut iter = "abc".graphemes(true);
    /// assert_eq!(iter.as_str(), "abc");
    /// iter.next();
    /// assert_eq!(iter.as_str(), "bc");
    /// iter.next();
    /// iter.next();
    /// assert_eq!(iter.as_str(), "");
    /// ```
    pub fn as_str(&self) -> &'a str {
        &self.string[self.cursor.cur_cursor()..self.cursor_back.cur_cursor()]
    }

    pub fn new<'b>(s: &'b str, is_extended: bool) -> Graphemes<'b> {
        let len = s.len();
        Graphemes {
            string: s,
            cursor: GraphemeCursor::new(0, len, is_extended),
            cursor_back: GraphemeCursor::new(len, len, is_extended),
        }
    }
}

impl<'a> Iterator for Graphemes<'a> {
    type Item = &'a str;

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let slen = self.cursor_back.cur_cursor() - self.cursor.cur_cursor();
        (cmp::min(slen, 1), Some(slen))
    }

    #[inline]
    fn next(&mut self) -> Option<&'a str> {
        let start = self.cursor.cur_cursor();
        if start == self.cursor_back.cur_cursor() {
            return None;
        }
        let next = self.cursor.next_boundary(self.string, 0).unwrap().unwrap();
        Some(&self.string[start..next])
    }
}

impl<'a> DoubleEndedIterator for Graphemes<'a> {
    #[inline]
    fn next_back(&mut self) -> Option<&'a str> {
        let end = self.cursor_back.cur_cursor();
        if end == self.cursor.cur_cursor() {
            return None;
        }
        let prev = self
            .cursor_back
            .prev_boundary(self.string, 0)
            .unwrap()
            .unwrap();
        Some(&self.string[prev..end])
    }
}
