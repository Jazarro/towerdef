use std::cmp::min;

use amethyst::{
    core::math::Vector3,
    core::transform::Transform,
    ecs::prelude::{Join, Read, System, WriteStorage},
};

use dsf_core::resources::DepthLayer;

use crate::components::*;
use crate::resources::*;
use amethyst::core::ecs::ReadStorage;

/// Responsible for managing the selection.
pub struct SelectionSystem;

impl<'s> System<'s> for SelectionSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, SelectionTag>,
        Read<'s, EditorStatus>,
    );

    fn run(&mut self, (mut transforms, selection_tags, status): Self::SystemData) {
        for (_, transform) in (&selection_tags, &mut transforms).join() {
            let width = (status.selection.start.x - status.selection.end.x).abs() + 1;
            let height = (status.selection.start.y - status.selection.end.y).abs() + 1;
            // TODO: set scale requires knowledge about dimensions of sprite.
            // Maybe solve with child entity.
            // Or accept hardcoded nature, because sprite unlikely to change?
            if width == 1 && height == 1 {
                transform.set_scale(Vector3::new(0., 0., 1.0));
            } else {
                transform.set_scale(Vector3::new(width as f32 / 128., height as f32 / 128., 1.0));
            }

            transform.set_translation_xyz(
                (width as f32 * 0.5) + min(status.selection.start.x, status.selection.end.x) as f32,
                (height as f32 * 0.5)
                    + min(status.selection.start.y, status.selection.end.y) as f32,
                (&DepthLayer::Selection).z(),
            );
        }
    }
}
