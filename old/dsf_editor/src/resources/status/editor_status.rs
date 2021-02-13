use crate::resources::{Brush, Selection};

/// Contains some transient data related to the status of the editor.
/// Holds things like the position of the cursor.
/// This is not persisted upon exiting the editor, though it persists through play testing.
/// Is only reset when the EditorState goes through on_create.
#[derive(Debug)]
pub struct EditorStatus {
    /// Contains information on which tile is currently selected to be placed. Also contains the
    /// palette: all possible tiles that the editor could use.
    pub brush: Brush,
    /// The area that is currently selected.
    pub selection: Selection,
    /// If true, air will be included when copying a selection. In combination with the
    /// force_place flag, that means that copied air can clear out existing tiles.
    pub copy_air: bool,
    /// If true, existing tiles will be removed if they are in the way when placing tiles.
    /// If false, existing tiles will never be removed when placing tiles or pasting blueprints.
    ///     That means that it could happen that only part of the tiles are actually placed.
    pub force_place: bool,
}

impl Default for EditorStatus {
    fn default() -> Self {
        EditorStatus {
            brush: Brush::default(),
            selection: Selection::default(),
            copy_air: true,
            force_place: true,
        }
    }
}
