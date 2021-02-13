use amethyst::{
    assets::PrefabData,
    derive::PrefabData,
    ecs::{prelude::Entity, Component, DenseVecStorage, WriteStorage},
    error::Error,
};
use dsf_core::components::Pos;
use serde::{Deserialize, Serialize};

/// The entity with this component is the graphical representation of a tile in the LevelEdit
/// resource. It has a position by which one can look up the corresponding Tile in the LevelEdit.
#[derive(Clone, Copy, Debug, Default, Component, Deserialize, Serialize, PrefabData)]
#[prefab(Component)]
#[serde(deny_unknown_fields)]
pub struct PaintedTile {
    pub pos: Pos,
}

impl PaintedTile {
    pub fn new(pos: Pos) -> Self {
        PaintedTile { pos }
    }
}
