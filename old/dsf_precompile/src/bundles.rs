use std::path::PathBuf;

use crate::structs::AnimationId;
use amethyst::{
    animation::AnimationBundle,
    core::{transform::TransformBundle, SystemBundle},
    ecs::DispatcherBuilder,
    error::Error,
    input::{InputBundle, StringBindings},
    prelude::World,
    renderer::{
        plugins::{RenderDebugLines, RenderFlat2D, RenderToWindow},
        sprite::SpriteRender,
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{RenderUi, UiBundle},
    utils::fps_counter::FpsCounterBundle,
};

pub struct PrecompiledRenderBundle {
    pub display_config_path: PathBuf,
}

impl<'a, 'b> SystemBundle<'a, 'b> for PrecompiledRenderBundle {
    fn build(
        self,
        world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        // Saves ~10 seconds.
        // The renderer must be executed on the same thread consecutively, so we initialize it as thread_local
        // which will always execute on the main thread.
        RenderingBundle::<DefaultBackend>::new()
            // The RenderToWindow plugin provides all the scaffolding for opening a window and drawing on it
            .with_plugin(
                RenderToWindow::from_config_path(self.display_config_path)?
                    .with_clear([0.0, 0.0, 0.0, 1.0]),
            )
            // RenderFlat2D plugin is used to render entities with `SpriteRender` component.
            .with_plugin(RenderFlat2D::default())
            .with_plugin(RenderUi::default())
            .with_plugin(RenderDebugLines::default())
            .build(world, builder)?;
        Ok(())
    }
}

pub struct PrecompiledDefaultsBundle {
    pub bindings_config_path: PathBuf,
}

impl<'a, 'b> SystemBundle<'a, 'b> for PrecompiledDefaultsBundle {
    fn build(
        self,
        world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        // Saves ~0.5 - 1.5 seconds.
        AnimationBundle::<AnimationId, SpriteRender>::new(
            "sprite_animation_control",
            "sprite_sampler_interpolation",
        )
        .build(world, builder)?;

        // saves ~2 seconds
        InputBundle::<StringBindings>::new()
            .with_bindings_from_file(self.bindings_config_path)?
            .build(world, builder)?;

        // Saves ~0.3 seconds
        TransformBundle::new().build(world, builder)?;

        // Saves ~2.25 seconds.
        UiBundle::<StringBindings>::new().build(world, builder)?;
        FpsCounterBundle {}.build(world, builder)?;

        Ok(())
    }
}
