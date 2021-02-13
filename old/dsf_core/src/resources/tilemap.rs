use crate::components::*;
use crate::levels::*;
use crate::resources::{TileDefinition, TileDefinitions, WorldBounds};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Default, Clone)]
pub struct TileMap {
    pub world_bounds: WorldBounds,
    pub tiles: HashMap<Pos, Tile>,
    pub tile_defs: TileDefinitions,
}

impl TileMap {
    /// Construct a TileMap for use during the PlayState.
    /// Keeps track of some relevant tiles only: climbable, collidable, destructable tiles.
    pub fn for_play(level: LevelSave, tile_defs: TileDefinitions) -> Self {
        TileMap::new(level, tile_defs, true)
    }

    pub fn for_editing(level: LevelSave, tile_defs: TileDefinitions) -> Self {
        TileMap::new(level, tile_defs, false)
    }

    fn new(level: LevelSave, tile_defs: TileDefinitions, apply_filter: bool) -> Self {
        let mut tiles = HashMap::new();
        level.tiles.iter()
            .map(|(pos, key)| {
                (pos, key, tile_defs.get(key))
            })
            .filter(|(_, _, tile_def)| {
                // Make sure we only add relevant stuff to the tile map.
                !apply_filter || tile_def.climbable || tile_def.collision.is_some() || tile_def.is_breakable()
            })
            .for_each(|(pos, key, tile_def)| {
                let dimens = tile_def.dimens;
                for x in 0..dimens.x {
                    for y in 0..dimens.y {
                        let tile = if x == 0 && y == 0 {
                            Tile::TileDefKey(key.clone())
                        } else {
                            Tile::Dummy(*pos)
                        };
                        if let Some(replaced_value) = tiles.insert(Pos::new(pos.x + x, pos.y + y), tile) {
                            error!("Error! At pos ({:?},{:?}), there are multiple tiles! {:?} replaces {:?}",
                                   x,
                                   y,
                                   (pos, key),
                                   replaced_value);
                        }
                    }
                }
            });
        TileMap {
            world_bounds: level.world_bounds,
            tiles,
            tile_defs,
        }
    }

    pub fn get_tile(&self, pos: &Pos) -> Option<&TileDefinition> {
        self.tiles
            .get(pos)
            .map(|tile| match tile {
                Tile::Dummy(pos) => match self.tiles.get(pos) {
                    Some(Tile::TileDefKey(key)) => Some(key),
                    _ => {
                        error!("Error! Dummy position lookup failed for tile {:?}", tile);
                        None
                    }
                },
                Tile::TileDefKey(key) => Some(key),
                _ => None,
            })
            .flatten()
            .map(|tile_def_key| self.tile_defs.get(tile_def_key))
    }

    pub fn is_tile_def_key(&self, pos: &Pos) -> bool {
        matches!(self.tiles.get(pos), Some(Tile::TileDefKey(_)))
    }

    /// Removes the tile at the given location. Also removes any Dummy tiles associated with the
    /// tile that is being removed.
    /// It is allowed to give the location of one of the dummy tiles, this function will remove
    /// the tile that the dummy is pointing at.
    ///
    /// If a tile was removed, returns Some() containing the actual position of the removed tile.
    /// This position is not necessarily the same as the given pos, because that pos might point
    /// at a dummy.
    ///
    /// If no tiles were removed, then this returns None.
    pub fn remove_tile(&mut self, pos: &Pos) -> Option<Pos> {
        let actual_pos = self.get_actual_pos(pos);
        if let Some(actual_pos) = actual_pos {
            let tile_def = self.get_tile(&actual_pos).unwrap_or_else(|| unreachable!());
            let dimens = tile_def.dimens;
            (0..dimens.x).for_each(|x| {
                (0..dimens.y).for_each(|y| {
                    self.tiles.remove(&actual_pos.append_xy(x, y));
                });
            });
        }
        actual_pos
    }

    /// For the given position, finds the anchor position of the tile that covers that position.
    /// If there is no tile covering the given position, None is returned.
    ///
    /// Example: if there is a 2x2 tile at position (0, 0), enquiring about any of the following
    /// four positions will return Some((0, 0)): (0, 0), (1, 0), (0, 1), (1, 1).
    pub fn get_actual_pos(&self, pos: &Pos) -> Option<Pos> {
        match self.tiles.get(pos) {
            Some(Tile::TileDefKey(_)) => Some(*pos),
            Some(Tile::Dummy(anchor_pos)) => Some(*anchor_pos),
            _ => None,
        }
    }

    pub fn put_tile(&mut self, pos: Pos, tile_def_key: String, dimensions: &Pos) {
        self.tiles.insert(pos, Tile::TileDefKey(tile_def_key));
        (0..dimensions.x).for_each(|x| {
            (0..dimensions.y).for_each(|y| {
                if x != 0 || y != 0 {
                    self.tiles.insert(pos.append_xy(x, y), Tile::Dummy(pos));
                }
            })
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Tile {
    /// A dummy tile, points towards its anchor point, where the real tile is stored.
    /// Dummy tiles are used when a tile is bigger than 1 by 1. The bottom-left position within the
    /// tile will be set as the tile itself. All other positions will contain a dummy that points
    /// towards that bottom-left position.
    Dummy(Pos),
    /// A tile. Contains a tile definition key.
    /// TODO: Should we perhaps include the dimensions here? Consider.
    TileDefKey(String),
    /// Normally, if there is no mapping for a position, that position counts as occupied by
    /// an air block. Therefore, Tile::AirBlock is not used during normal game play.
    ///
    /// However, if you specifically want to override an existing block with an air block,
    /// you can use this. Currently, this only happens in the level editor.
    AirBlock,
}

impl Tile {
    /// Returns true iff the enum is an actual tile, rather than an air block or a dummy reference.
    pub fn is_tile_def(&self) -> bool {
        matches!(self, Tile::TileDefKey(_))
    }
}
