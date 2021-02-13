use std::cmp::min;

use dsf_core::components::Pos;

#[derive(Debug, Default)]
pub struct Selection {
    /// Inclusive bound.
    pub start: Pos,
    /// Inclusive bound. The end point of the selection is always set to the current location of the cursor.
    pub end: Pos,
}

impl Selection {
    pub fn lower_bounds(&self) -> Pos {
        Pos::new(min(self.start.x, self.end.x), min(self.start.y, self.end.y))
    }
    pub fn dimens(&self) -> Pos {
        Pos::new(
            (self.start.x - self.end.x).abs() + 1,
            (self.start.y - self.end.y).abs() + 1,
        )
    }
}
