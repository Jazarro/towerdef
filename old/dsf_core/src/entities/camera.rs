use crate::components::*;
use crate::resources::DepthLayer;
use amethyst::{
    core::{math::Vector3, transform::Transform, Parent},
    ecs::Entity,
    prelude::{Builder, World, WorldExt},
    renderer::Camera,
    window::ScreenDimensions,
};

/// Initialise the camera.
pub fn create_camera(world: &mut World) {
    let frame = initialise_camera_frame(world);
    create_camera_under_parent(world, frame);
}

pub fn create_camera_under_parent(world: &mut World, parent: Entity) {
    let (width, height) = {
        let dim = world.fetch::<ScreenDimensions>();
        (dim.width(), dim.height())
    };
    world
        .create_entity()
        .with(Parent { entity: parent })
        .with(Camera::standard_2d(width, height))
        .with(Transform::default())
        .build();
}

pub fn initialise_camera_frame(world: &mut World) -> Entity {
    let mut transform = Transform::default();
    transform.set_translation_xyz(0.0, 0.0, (&DepthLayer::Camera).z());
    let scale = 1. / 32.;
    transform.set_scale(Vector3::new(scale, scale, 1.0));
    world
        .create_entity()
        .with(CameraFrame::default())
        .with(transform)
        .build()
}
