use strum::{Display, EnumIter, EnumMessage, EnumString, IntoStaticStr};

#[derive(
    Display, EnumString, EnumIter, Clone, PartialEq, Eq, Debug, EnumMessage, IntoStaticStr,
)]
pub enum EditOp {
    #[strum(serialize = "move_line_up")]
    MoveLineUp,
    #[strum(serialize = "move_line_down")]
    MoveLineDown,
    #[strum(serialize = "insert_new_line")]
    InsertNewLine,
    #[strum(serialize = "insert_tab")]
    InsertTab,
    #[strum(serialize = "new_line_above")]
    NewLineAbove,
    #[strum(serialize = "new_line_below")]
    NewLineBelow,
    #[strum(serialize = "delete_backward")]
    DeleteBackward,
    #[strum(serialize = "delete_forward")]
    DeleteForward,
    #[strum(serialize = "delete_line")]
    DeleteLine,
    #[strum(serialize = "delete_forward_and_insert")]
    DeleteForwardAndInsert,
    #[strum(serialize = "delete_word_and_insert")]
    DeleteWordAndInsert,
    #[strum(serialize = "delete_line_and_insert")]
    DeleteLineAndInsert,
    #[strum(serialize = "delete_word_forward")]
    DeleteWordForward,
    #[strum(serialize = "delete_word_backward")]
    DeleteWordBackward,
    #[strum(serialize = "delete_to_beginning_of_line")]
    DeleteToBeginningOfLine,
    #[strum(serialize = "delete_to_end_of_line")]
    DeleteToEndOfLine,

    #[strum(serialize = "delete_to_end_and_insert")]
    DeleteToEndOfLineAndInsert,
    #[strum(message = "Join Lines")]
    #[strum(serialize = "join_lines")]
    JoinLines,
    #[strum(message = "Indent Line")]
    #[strum(serialize = "indent_line")]
    IndentLine,
    #[strum(message = "Outdent Line")]
    #[strum(serialize = "outdent_line")]
    OutdentLine,
    #[strum(message = "Toggle Line Comment")]
    #[strum(serialize = "toggle_line_comment")]
    ToggleLineComment,
    #[strum(serialize = "undo")]
    Undo,
    #[strum(serialize = "redo")]
    Redo,
    #[strum(message = "Copy")]
    #[strum(serialize = "clipboard_copy")]
    ClipboardCopy,
    #[strum(message = "Cut")]
    #[strum(serialize = "clipboard_cut")]
    ClipboardCut,
    #[strum(message = "Paste")]
    #[strum(serialize = "clipboard_paste")]
    ClipboardPaste,
    #[strum(serialize = "yank")]
    Yank,
    #[strum(serialize = "paste")]
    Paste,
    #[strum(serialize = "paste_before")]
    PasteBefore,

    #[strum(serialize = "normal_mode")]
    NormalMode,
    #[strum(serialize = "insert_mode")]
    InsertMode,
    #[strum(serialize = "insert_first_non_blank")]
    InsertFirstNonBlank,
    #[strum(serialize = "append")]
    Append,
    #[strum(serialize = "append_end_of_line")]
    AppendEndOfLine,
    #[strum(serialize = "toggle_visual_mode")]
    ToggleVisualMode,
    #[strum(serialize = "toggle_linewise_visual_mode")]
    ToggleLinewiseVisualMode,
    #[strum(serialize = "toggle_blockwise_visual_mode")]
    ToggleBlockwiseVisualMode,
    #[strum(serialize = "duplicate_line_up")]
    DuplicateLineUp,
    #[strum(serialize = "duplicate_line_down")]
    DuplicateLineDown,
}
