#![forbid(unsafe_code)]

#[macro_use]
extern crate log;

mod state_loading;
mod state_main_menu;

use amethyst::{
    assets::{PrefabLoaderSystemDesc, Processor},
    audio::{DjSystemDesc, Source},
    GameDataBuilder, LoggerConfig,
};

use dsf_core::systems;

use crate::state_loading::LoadingState;
use dsf_core::resources::{create_default_adventure, Music};
use dsf_core::systems::PlaySfxSystem;
use dsf_core::utility::files::{get_assets_dir, get_config_dir};
use dsf_precompile::PrecompiledDefaultsBundle;
use dsf_precompile::PrecompiledRenderBundle;
use dsf_precompile::{start_game, MyPrefabData};

fn main() -> amethyst::Result<()> {
    amethyst::Logger::from_config(LoggerConfig::default()).start();
    let display_config_path = get_config_dir().join("display.ron");
    let bindings_config_path = get_config_dir().join("input.ron");
    create_default_adventure();

    let game_data = GameDataBuilder::default()
        .with_bundle(PrecompiledDefaultsBundle {
            bindings_config_path,
        })?
        .with_system_desc(
            PrefabLoaderSystemDesc::<MyPrefabData>::default(),
            "prefab_loader",
            &[],
        )
        .with(Processor::<Source>::new(), "source_processor", &[])
        .with(
            systems::FpsCounterUiSystem::default(),
            "fps_counter_ui_system",
            &[],
        )
        .with(systems::CameraSystem, "camera_system", &[])
        .with(
            systems::CameraControlSystem,
            "camera_control_system",
            &["camera_system"],
        )
        .with(
            // Temporarily include this system here, because it uses an event reader and must
            // therefore keep reading on every frame.
            dsf_editor::systems::RefreshPreviewsSystem::default(),
            "refresh_preview_system",
            &[],
        )
        .with_system_desc(
            DjSystemDesc::new(|music: &mut Music| music.music.next()),
            "dj",
            &[],
        )
        .with(PlaySfxSystem::default(), "play_sfx_system", &[])
        .with(systems::DummySystem, "dummy_system", &[])
        .with_bundle(PrecompiledRenderBundle {
            display_config_path,
        })?;

    start_game(
        get_assets_dir(),
        game_data,
        Some(Box::new(LoadingState::default())),
    );
    Ok(())
}
