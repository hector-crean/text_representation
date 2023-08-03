use crate::api::formatting::Formatting;

pub mod buffer_impl;
pub mod cursor_impl;
pub mod dot;
pub mod grapheme;

pub struct TextNode {
    text: String,
    formatting: Vec<Formatting>,
    left: Option<Box<TextNode>>,
    right: Option<Box<TextNode>>,
    weight: usize,
}
