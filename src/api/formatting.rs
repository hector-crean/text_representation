use std::ops::Range;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Formatting {
    Bold,
    Italic,
    Underline,
    StrikeThrough,
    FontSize(u32),
    FontColor(u8, u8, u8), // RGB
    Link { url: String },
    Citation { text: String },
}

pub trait Formattable {
    /// Apply a given formatting to the specified range of text.
    fn apply_formatting(&mut self, range: Range<usize>, formatting: Formatting);

    /// Remove a given formatting from the specified range of text.
    fn remove_formatting(&mut self, range: Range<usize>, formatting: Formatting);

    /// Get the formattings applied to the text at the given position.
    fn formattings_at(&self, position: usize) -> Vec<Formatting>;

    /// Get the formattings applied to the text within the given range.
    fn formattings_in_range(&self, range: Range<usize>) -> Vec<(Range<usize>, Formatting)>;
}

// interval tree ?
