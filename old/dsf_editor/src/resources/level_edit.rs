use dsf_core::components::Pos;
use dsf_core::levels::LevelSave;
use dsf_core::resources::{Tile, TileDefinition, TileDefinitions, TileMap, WorldBounds};
use std::collections::{HashMap, HashSet};

/// The representation of a level in the level editor.
#[derive(Debug, Default, Clone)]
pub struct LevelEdit {
    pub tile_map: TileMap,
    /// A list of tile positions that are marked are dirty and must be redrawn.
    /// Whenever you add, update or remove a tile in the editor, you must mark it as dirty.
    pub dirty: HashSet<Pos>,
}

/// Implements the standard converter from LevelEdit to LevelSave. In other words: convert a level
/// from a format that the editor uses, to the format that levels are stored in on disk.
///
/// Note that only TileDefKey entries are saved, Dummy entries are discarded, because they can be
/// derived from the primary entries and only exist for faster lookup.
impl From<LevelEdit> for LevelSave {
    fn from(mut item: LevelEdit) -> Self {
        let mut map = HashMap::new();
        item.tile_map.tiles.drain().for_each(|(pos, tile)| {
            if let Tile::TileDefKey(tile_def_key) = tile {
                map.insert(pos, tile_def_key);
            }
        });
        LevelSave {
            world_bounds: item.tile_map.world_bounds,
            tiles: map,
        }
    }
}

impl LevelEdit {
    pub(crate) fn new(level_save: LevelSave, tile_defs: TileDefinitions) -> Self {
        let initial_dirty = level_save.tiles.keys().copied().collect::<HashSet<Pos>>();
        LevelEdit {
            tile_map: TileMap::for_editing(level_save, tile_defs),
            dirty: initial_dirty,
        }
    }

    /// Empty the set of dirty positions and collect them into a new collection.
    ///
    /// The process of re-collecting them before draining them (or otherwise iterating over them)
    /// again allows you to iterate over them without having an outstanding mutable borrow on the
    /// LevelEdit. This allows you to access the tile map while iterating.
    pub(crate) fn drain_dirty(&mut self) -> Vec<Pos> {
        self.dirty.drain().collect::<Vec<Pos>>()
    }

    /// Attempt to place the given tile at the given position.
    pub(crate) fn place_tile(&mut self, force_place: bool, pos: Pos, tile: Option<Tile>) {
        let mut dry_run = self.check_place_tile(force_place, pos, tile);
        dry_run.to_be_removed.iter().for_each(|delete_pos| {
            if let Some(removed_pos) = self.tile_map.remove_tile(&delete_pos) {
                self.dirty.insert(removed_pos);
            }
        });
        dry_run
            .to_be_added
            .drain(..)
            .for_each(|(pos, key, dimensions)| {
                self.tile_map.put_tile(pos, key, &dimensions);
                self.dirty.insert(pos);
            });
    }

    /// Does a dry-run to check what would happen if we'd place the given tile right now.
    /// Determines which existing tiles would need to be removed and which tiles would need to be
    /// placed.
    /// This does not actually place or remove anything.
    pub(crate) fn check_place_tile(
        &self,
        force_place: bool,
        pos: Pos,
        tile: Option<Tile>,
    ) -> PlaceTileDryRun {
        match tile {
            // Trying to place a new tile:
            Some(Tile::TileDefKey(key)) => self.check_add_tile(pos, key, force_place),
            // Air blocks are ignored unless force_place is enabled.
            // Delete whatever is at that location:
            Some(Tile::AirBlock) if force_place => {
                PlaceTileDryRun::remove_single(self.tile_map.get_actual_pos(&pos))
            }
            // When explicitly placing an empty Option, remove whatever is at that location whether
            // force_place is enabled or not:
            None => PlaceTileDryRun::remove_single(self.tile_map.get_actual_pos(&pos)),
            // In all other cases, do nothing:
            _ => PlaceTileDryRun::default(),
        }
    }

    /// Does a dry run to check what would happen if we added a new tile.
    fn check_add_tile(&self, pos: Pos, key: String, force_place: bool) -> PlaceTileDryRun {
        let dimensions = self.get_tile_def(&key).dimens;
        let obstructed = !self.bounds().encloses(&pos, &dimensions)
            || (!force_place
                && (0..dimensions.x).any(|x| {
                    (0..dimensions.y).any(|y| {
                        self.tile_map
                            .get_tile(&Pos::new(pos.x + x, pos.y + y))
                            .is_some()
                    })
                }));
        if obstructed {
            PlaceTileDryRun::default()
        } else {
            let to_be_removed = (0..dimensions.x)
                .flat_map(|x| (0..dimensions.y).map(move |y| (x, y)))
                .map(|(x, y)| {
                    self.tile_map
                        .get_actual_pos(&Pos::new(pos.x + x, pos.y + y))
                })
                .flatten()
                .collect();
            PlaceTileDryRun {
                to_be_added: vec![(pos, key, dimensions)],
                to_be_removed,
            }
        }
    }

    /// Returns the TileDefinition that belongs to the given key.
    pub(crate) fn get_tile_def(&self, tile_def_key: &str) -> &TileDefinition {
        self.tile_map.tile_defs.get(tile_def_key)
    }

    /// Returns the world bounds for this level.
    pub(crate) fn bounds(&self) -> &WorldBounds {
        &self.tile_map.world_bounds
    }

    /// Returns a mutable reference to the world bounds for this level.
    pub(crate) fn bounds_mut(&mut self) -> &mut WorldBounds {
        &mut self.tile_map.world_bounds
    }
}

/// When performing a place-tile dry-run to determine what tiles (if any) to place and what tiles
/// (if any) to remove, this is the object it returns that encapsulates that information.
#[derive(Debug, Default)]
pub struct PlaceTileDryRun {
    /// The tiles that should be placed.
    /// Is a collection of tuples: (tile_position_in_the_world, tile_def_key, tile_dimensions)
    pub to_be_added: Vec<(Pos, String, Pos)>,
    /// The anchor-positions of all existing tiles that should be removed.
    pub to_be_removed: HashSet<Pos>,
}

impl PlaceTileDryRun {
    fn remove_single(pos_to_be_removed: Option<Pos>) -> Self {
        PlaceTileDryRun {
            to_be_added: vec![],
            to_be_removed: pos_to_be_removed.iter().copied().collect(),
        }
    }

    /// Combines two dry runs into one.
    pub(crate) fn extend(mut self, other: PlaceTileDryRun) -> Self {
        self.to_be_added.extend(other.to_be_added);
        self.to_be_removed.extend(other.to_be_removed);
        self
    }
}
