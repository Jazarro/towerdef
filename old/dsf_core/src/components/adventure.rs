use crate::components::Direction2D;
use serde::{Deserialize, Serialize};

use amethyst::ecs::{Component, DenseVecStorage};

/// This is used in the adventure and level selector. The entity with this component represents
/// where the player is on the map.
#[derive(Clone, Copy, Debug, Default, Component, Deserialize, Serialize)]
pub struct MapCursor {
    pub last_direction: Direction2D,
    pub cooldown: f32,
}
