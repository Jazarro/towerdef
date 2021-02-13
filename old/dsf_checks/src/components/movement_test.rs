use amethyst::{
    assets::PrefabData,
    derive::PrefabData,
    ecs::{prelude::Entity, Component, NullStorage, WriteStorage},
    error::Error,
};
use serde::{Deserialize, Serialize};

/// Entities tagged with this component were created to run a movement test.
/// Once the test is done, it is safe to remove them.
#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, PrefabData)]
#[prefab(Component)]
#[serde(deny_unknown_fields)]
pub struct MovementTestScopeTag;

impl Component for MovementTestScopeTag {
    type Storage = NullStorage<Self>;
}

/// Various sorts of tests that can be executed.
pub enum MovementTest {
    /// This test will prove that the player can ALWAYS make a jump across a 2-wide gap.
    Jump2Wide,
    /// This test will prove that the player can NEVER make a jump across a 4-wide gap.
    Jump4Wide,
}
