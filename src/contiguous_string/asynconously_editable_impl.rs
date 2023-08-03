use crate::api::{
    asynchonously_editable_text_buffer::AsynchonouslyEditableTextBuffer, edit_operations::EditOp,
};

impl<'cursor> AsynchonouslyEditableTextBuffer for String {
    type Operation = EditOp;

    fn apply_operation(&mut self, operation: Self::Operation) {
        match operation {
            _ => {}
        }
    }

    fn generate_operation(&self) -> Self::Operation {
        unimplemented!()
    }

    fn merge(&mut self, other: &Self) {
        unimplemented!()
    }

    fn get_state(&self) -> Self {
        unimplemented!()
    }

    fn set_state(&mut self, state: Self) {
        todo!()
    }
}
