use amethyst::{
    core::math::Vector3,
    core::transform::Transform,
    ecs::prelude::{Join, Read, System, WriteStorage},
};

use dsf_core::resources::DepthLayer;

use crate::resources::*;
use amethyst::core::ecs::ReadStorage;
use dsf_core::components::BackgroundTag;

/// Responsible for updating the size and location of the background sprite whenever the
/// world bounds change.
pub struct UpdateBackgroundSystem;

impl<'s> System<'s> for UpdateBackgroundSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, BackgroundTag>,
        Read<'s, LevelEdit>,
    );

    fn run(&mut self, (mut transforms, backgrounds, level_edit): Self::SystemData) {
        for (_, transform) in (&backgrounds, &mut transforms).join() {
            // TODO: set scale requires knowledge about dimensions of sprite.
            transform.set_scale(Vector3::new(
                level_edit.bounds().width() as f32 / 128.,
                level_edit.bounds().height() as f32 / 128.,
                1.0,
            ));
            transform.set_translation_xyz(
                level_edit.bounds().x() as f32 + (level_edit.bounds().width() as f32 * 0.5),
                level_edit.bounds().y() as f32 + (level_edit.bounds().height() as f32 * 0.5),
                (&DepthLayer::Background).z(),
            );
        }
    }
}
