use amethyst::{
    assets::PrefabData,
    derive::PrefabData,
    ecs::{prelude::Entity, Component, DenseVecStorage, NullStorage, WriteStorage},
    error::Error,
};
use dsf_core::components::Direction2D;
use serde::{Deserialize, Serialize};

/// Entities with this component are the ghostly outlines of tiles before they are placed.
/// For example, if the user has equipped the exit door tile on their brush, a ghostly outline
/// of the exit door will appear where it will be placed.
#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, PrefabData)]
#[prefab(Component)]
#[serde(deny_unknown_fields)]
pub struct PreviewGhostTag;

impl Component for PreviewGhostTag {
    type Storage = NullStorage<Self>;
}

/// This component identifies an entity as the cursor. There should be no more than one of these
/// at any given time. There is a separate component for the selection area.
#[derive(Clone, Copy, Component, Debug, Default, Deserialize, Serialize, PrefabData)]
#[prefab(Component)]
#[serde(deny_unknown_fields)]
pub struct Cursor {
    pub last_direction: Direction2D,
    /// Time in seconds before cursor is allowed to move again.
    pub movement_cooldown: f32,
    pub is_visible: bool,
    /// Time in seconds before cursor is allowed to change its visibility, as part of its
    /// blinking animation. This will be reset when the cursor moves, so as not to obscure the
    /// cursor when the user is actually moving it.
    pub blink_cooldown: f32,
}
