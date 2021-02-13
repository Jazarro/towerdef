use crate::components::*;

use crate::resources::*;

use amethyst::core::num::FloatConst;
use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    ecs::prelude::{Join, Read, ReadStorage, System, WriteStorage},
};

/// For every entity with a velocity and a transform, updates the transform according to the
/// velocity.
pub struct VelocitySystem;

impl<'s> System<'s> for VelocitySystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Velocity>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut transforms, velocities, time): Self::SystemData) {
        for (transform, velocity) in (&mut transforms, &velocities).join() {
            transform
                .set_translation_x(transform.translation().x + time.fixed_seconds() * velocity.x);
            transform
                .set_translation_y(transform.translation().y + time.fixed_seconds() * velocity.y);
        }
    }
}

/// Sets velocity for all entities with steering.
pub struct MovementSystem;

impl<'s> System<'s> for MovementSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        ReadStorage<'s, Steering>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Velocity>,
        Read<'s, MovementConfig>,
        Read<'s, Time>,
    );

    fn run(
        &mut self,
        (steerings, mut transforms, mut velocities, config, _time): Self::SystemData,
    ) {
        for (transform, steering, velocity) in (&mut transforms, &steerings, &mut velocities).join()
        {
            // Flip sprite if character is facing left:
            transform.set_rotation_y_axis(if steering.facing.x == Direction1D::Negative {
                f32::PI()
            } else {
                0.
            });

            let (centered_x, centered_y) = steering.to_centered_coords(steering.pos);
            let (desired_pos_x, desired_pos_y) = steering.to_centered_coords(steering.destination);
            match steering.mode {
                SteeringMode::Grounded => {
                    // If grounded, correct y translation and zero out y velocity.
                    transform.set_translation_y(centered_y);
                    velocity.y = 0.0;
                }
                SteeringMode::Climbing => {
                    // If climbing, correct x translation and zero out x velocity.
                    transform.set_translation_x(centered_x);
                    velocity.x = 0.0;
                    // If climbing:
                    let delta = desired_pos_y - transform.translation().y;
                    if steering.facing.y.aligns_with(delta) {
                        velocity.y = steering.facing.y.signum() * config.player_speed;
                    } else {
                        velocity.y = 0.0;
                        transform.set_translation_y(centered_y);
                    }
                }
                SteeringMode::Falling {
                    starting_y_pos,
                    duration,
                    ..
                } => {
                    // Set y-position directly, based on movement function. We don't use velocity for this.
                    velocity.y = 0.0;
                    transform
                        .set_translation_y(starting_y_pos + steering.mode.calc_delta_y(duration));
                }
                SteeringMode::Jumping {
                    starting_y_pos,
                    duration,
                    ..
                } => {
                    // Set y-position directly, based on movement function. We don't use velocity for this.
                    velocity.y = 0.0;
                    transform
                        .set_translation_y(starting_y_pos + steering.mode.calc_delta_y(duration));
                }
            }

            // Set x-velocity based on current and desired position.
            // If necessary, adjust x-position, snap to grid.
            let delta = desired_pos_x - transform.translation().x;
            if steering.facing.x.aligns_with(delta) {
                velocity.x = steering.facing.x.signum() * config.player_speed;
            } else {
                velocity.x = 0.0;
                transform.set_translation_x(centered_x);
            }
        }
    }
}
