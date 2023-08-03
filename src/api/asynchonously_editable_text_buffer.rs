use std::cmp::Ordering;

use super::text_buffer::TextBuffer;

pub trait AsynchonouslyEditableTextBuffer
where
    Self: TextBuffer,
{
    type Operation;

    /// Applies an update operation to the CRDT.
    fn apply_operation(&mut self, operation: Self::Operation);

    /// Generates an update operation based on the current state.
    fn generate_operation(&self) -> Self::Operation;

    /// Merges the state of another CRDT instance into the current one.
    fn merge(&mut self, other: &Self);

    /// Returns the current state of the CRDT.
    fn get_state(&self) -> Self;

    /// Sets the state of the CRDT.
    fn set_state(&mut self, state: Self);
}
