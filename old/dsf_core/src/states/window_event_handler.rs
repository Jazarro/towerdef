use amethyst::StateEvent;
use amethyst::{
    core::{transform::Transform, Parent},
    ecs::{prelude::World, Entities, Join, ReadStorage, WriteStorage},
    input::InputEvent,
    renderer::Camera,
    window::{ScreenDimensions, Window},
    winit::{Event, WindowEvent},
};

use crate::components::*;

use amethyst::core::ecs::ReadExpect;

/// Handle some general behaviour related to the window that should be executed in any State.
pub fn handle(event: &StateEvent, world: &mut World) {
    match event {
        StateEvent::Input(InputEvent::ActionPressed(action)) => {
            if action == "toggleFullscreen" {
                toggle_fullscreen(world);
            }
        }
        StateEvent::Window(Event::WindowEvent {
            window_id: _,
            event: WindowEvent::Resized(_),
        }) => {
            resize_camera(world);
        }
        _ => (),
    };
}

/// Toggles fullscreen mode.
fn toggle_fullscreen(world: &mut World) {
    let window = world.fetch_mut::<Window>();
    window.set_fullscreen(if window.get_fullscreen().is_none() {
        Some(window.get_current_monitor())
    } else {
        None
    });
}

/// Responds to window resize events. Recreates the camera with the new dimensions.
fn resize_camera(world: &mut World) {
    world.exec(
        #[allow(clippy::type_complexity)]
        |data: (
            Entities,
            WriteStorage<Camera>,
            WriteStorage<Transform>,
            WriteStorage<Parent>,
            ReadStorage<CameraFrame>,
            ReadExpect<ScreenDimensions>,
        )| {
            let (entities, mut cameras, mut transforms, mut parents, camera_frames, screen_dimens) =
                data;
            let frame = (&*entities, &camera_frames)
                .join()
                .map(|(entity, _)| entity)
                .next();
            let cam = (&*entities, &cameras)
                .join()
                .map(|(entity, _)| entity)
                .next();
            if let Some(frame) = frame {
                if let Some(cam) = cam {
                    entities
                        .delete(cam)
                        .expect("Trying to resize, but failed to delete camera.");
                }
                if screen_dimens.width() > f32::EPSILON && screen_dimens.height() > f32::EPSILON {
                    entities
                        .build_entity()
                        .with(Parent { entity: frame }, &mut parents)
                        .with(
                            Camera::standard_2d(screen_dimens.width(), screen_dimens.height()),
                            &mut cameras,
                        )
                        .with(Transform::default(), &mut transforms)
                        .build();
                }
            }
        },
    );
}
