use amethyst::{
    core::math::Vector3,
    core::timing::Time,
    core::transform::Transform,
    ecs::prelude::{Join, Read, ReadStorage, System, WriteStorage},
};

use crate::components::*;

use amethyst::core::num::FloatConst;

/// Responsible for animating the cursor previews (IE the ghostly outlines of the blocks that
/// would get placed if the user would press 'place' at that time).
pub struct AnimatePreviewsSystem;

impl<'s> System<'s> for AnimatePreviewsSystem {
    type SystemData = (
        ReadStorage<'s, PreviewGhostTag>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, (tags, mut transforms, time): Self::SystemData) {
        for (_, transform) in (&tags, &mut transforms).join() {
            let scale_factor = 1. - 0.1 * (time.absolute_time_seconds() * f64::PI()).sin().abs();
            transform.set_scale(Vector3::new(scale_factor, scale_factor, 1.0));
        }
    }
}
