use crate::components::*;
use amethyst::{
    core::transform::Transform,
    ecs::prelude::{Join, ReadStorage, System, WriteStorage},
};

/// Sometimes you just added a new component, but haven't added it to a system yet.
/// This will result in annoying runtime errors.
/// This system exists to solve this problem!
/// Add the component to this system temporarily, and the game will stop crashing.
///
/// ... It's not pretty, I know.
pub struct DummySystem;

impl<'s> System<'s> for DummySystem {
    #[allow(clippy::type_complexity)]
    type SystemData = ();

    fn run(&mut self, (): Self::SystemData) {
        // Do nothing.
    }
}

pub struct DebugSystem;

impl<'s> System<'s> for DebugSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Steering>,
        ReadStorage<'s, Player>,
        ReadStorage<'s, DebugPosGhostTag>,
        ReadStorage<'s, DebugSteeringGhostTag>,
    );

    fn run(
        &mut self,
        (
            mut transforms,
            steerings,
            player_tags,
            pos_ghost_tags,
            steering_ghost_tags,
        ): Self::SystemData,
    ) {
        // Sets the transform on the ghost tags.
        // This is a debug thing to show us where the player is going.
        let maybe_steering = (&player_tags, &steerings)
            .join()
            .map(|(_, steering)| steering)
            .next();
        if let Some(steering) = maybe_steering {
            for (_, transform) in (&steering_ghost_tags, &mut transforms).join() {
                let (centered_x, centered_y) = steering.to_centered_coords(steering.destination);
                transform.set_translation_x(centered_x);
                transform.set_translation_y(centered_y);
            }
            for (_, transform) in (&pos_ghost_tags, &mut transforms).join() {
                transform.set_translation_x(steering.pos.x as f32 + 0.5);
                transform.set_translation_y(steering.pos.y as f32 + 0.5);
            }
        }
    }
}
