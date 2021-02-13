use crate::components::*;

use amethyst::core::ecs::Entities;

use amethyst::ecs::prelude::{ReadStorage, System};

/// This shows how to handle UI events.
#[derive(Default)]
pub struct TestSetupSystem;

impl<'s> System<'s> for TestSetupSystem {
    type SystemData = (Entities<'s>, ReadStorage<'s, MovementTestScopeTag>);

    fn run(&mut self, (_entities, _tags): Self::SystemData) {}
}
