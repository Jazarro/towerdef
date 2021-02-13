use crate::components::*;

use amethyst::{
    core::transform::Transform,
    core::{
        math::{partial_clamp, Vector2},
        timing::Time,
    },
    ecs::prelude::{Join, Read, ReadStorage, System, WriteStorage},
    input::{InputHandler, StringBindings},
};

/// This system handles player input to control certain aspects of the camera.
/// Specifically: camera panning, camera zoom.
pub struct CameraControlSystem;

impl<'s> System<'s> for CameraControlSystem {
    type SystemData = (
        WriteStorage<'s, CameraFrame>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut camera_frames, input, time): Self::SystemData) {
        let _zoom = input.axis_value("zoom").unwrap_or(0.0);
        let pan_x = input.axis_value("pan_x").unwrap_or(0.0);
        let pan_y = input.axis_value("pan_y").unwrap_or(0.0);
        for frame in (&mut camera_frames).join() {
            if pan_x.abs() < f32::EPSILON && pan_y.abs() < f32::EPSILON {
                if frame.pan.magnitude() > f32::EPSILON {
                    // Recovery (jump back to zero pan)
                    let pan_add = frame.pan.normalize()
                        * frame.panning_recovery_speed
                        * time.delta_seconds()
                        * -1.;
                    if pan_add.magnitude() >= frame.pan.magnitude() {
                        frame.pan = Vector2::new(0., 0.);
                    } else {
                        frame.pan += pan_add;
                    }
                }
            } else {
                let pan_add =
                    Vector2::new(pan_x, pan_y) * frame.panning_speed * time.delta_seconds();
                frame.pan += pan_add;
                frame.pan = Vector2::new(
                    *partial_clamp(&frame.pan.x, &-frame.max_pan, &frame.max_pan)
                        .expect("Oh noes!"),
                    *partial_clamp(&frame.pan.y, &-frame.max_pan, &frame.max_pan)
                        .expect("Oh noes!"),
                );
            }
        }
    }
}

/// This system updates the camera frame position to center on the player's position.
pub struct CameraSystem;

impl<'s> System<'s> for CameraSystem {
    type SystemData = (
        ReadStorage<'s, Player>,
        ReadStorage<'s, CameraFrame>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, (players, camera_frames, mut transforms): Self::SystemData) {
        let maybe_player_pos = (&players, &transforms)
            .join()
            .map(|(_, transform)| (transform.translation().x, transform.translation().y))
            .next();
        if let Some((player_x, player_y)) = maybe_player_pos {
            for (frame, transform) in (&camera_frames, &mut transforms).join() {
                transform.set_translation_x(player_x + frame.pan.x);
                transform.set_translation_y(player_y + frame.pan.y);
            }
        }
    }
}
