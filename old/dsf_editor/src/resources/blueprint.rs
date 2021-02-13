use crate::resources::{EditorStatus, LevelEdit};
use dsf_core::components::Pos;
use dsf_core::resources::Tile;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Contains a tile map. Is a blueprint for a structure of tiles inside a level.
/// If you copy a selection in the level editor, that selection is stored as a Blueprint.
/// Blueprints can be pasted. Blueprints can potentially be imported and exported from the editor.
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Blueprint {
    pub dimensions: Pos,
    pub tiles: HashMap<Pos, Tile>,
}

impl Blueprint {
    pub fn new(dimensions: Pos) -> Self {
        Blueprint {
            dimensions,
            tiles: HashMap::default(),
        }
    }

    /// Create a new instance of Blueprint, based on the current selection and the tile on the
    /// brush. The blueprint will consist of rows and columns of whatever tile is on the brush,
    /// starting at the lower-left corner of the selection.
    ///
    /// When creating the Blueprint, existing tiles are ignored. It is therefore not guaranteed
    /// that all tiles in the blueprint will be placed; if force-place is not enabled and there are
    /// tiles in the way, that will prevent the whole blueprint being placed.
    pub fn from_placing_tiles(status: &EditorStatus, level_edit: &LevelEdit) -> Self {
        let key = status.brush.get_key().as_ref();
        let tile_def = key.map(|key| level_edit.tile_map.tile_defs.get(key));
        let brush_dimens = tile_def
            .map(|def| def.dimens)
            .unwrap_or_else(|| Pos::new(1, 1));
        let selection_dimens = (*status).selection.dimens();
        let mut blueprint = Blueprint::new(selection_dimens);
        for x in (0..(selection_dimens.x)).step_by(brush_dimens.x as usize) {
            for y in (0..(selection_dimens.y)).step_by(brush_dimens.y as usize) {
                if let Some(key) = key {
                    blueprint.insert_tile(
                        Pos::new(x, y),
                        &brush_dimens,
                        Tile::TileDefKey(key.clone()),
                    );
                } else {
                    blueprint.tiles.insert(Pos::new(x, y), Tile::AirBlock);
                }
            }
        }
        blueprint
    }

    fn insert_tile(&mut self, pos: Pos, dimens: &Pos, tile: Tile) {
        self.tiles.insert(pos, tile);
        for x in pos.x..(pos.x + dimens.x) {
            for y in pos.y..(pos.y + dimens.y) {
                if x != pos.x || y != pos.y {
                    self.tiles.insert(Pos::new(x, y), Tile::Dummy(pos));
                }
            }
        }
    }

    /// Returns true iff the given rectangle overlaps with any of the tiles in the blueprint.
    /// The given position must be relative to the blueprint.
    pub fn overlaps(&self, pos: Pos, dimens: Pos) -> bool {
        (pos.x..(pos.x + dimens.x))
            .any(|x| (pos.y..(pos.y + dimens.y)).any(|y| self.tiles.get(&Pos::new(x, y)).is_some()))
    }
}
