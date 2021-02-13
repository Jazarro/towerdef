use std::path::PathBuf;

use amethyst::prelude::WorldExt;

use amethyst::StateEvent;
use amethyst::{
    animation::{
        get_animation_set, AnimationCommand, AnimationControlSet, AnimationSet, EndControl,
    },
    core::timing::Time,
    ecs::{prelude::World, Entities, Join, ReadStorage, WriteStorage},
    input::{is_close_requested, is_key_down, InputEvent, VirtualKeyCode},
    renderer::sprite::SpriteRender,
    StateData, Trans,
};
use amethyst::{GameData, SimpleState, SimpleTrans};

use dsf_precompile::AnimationId;

use crate::entities::*;
use crate::levels::*;
use crate::resources::*;
use crate::states::window_event_handler;
use crate::systems;
use crate::systems::SoundEvent;
use crate::utility::files::get_levels_dir;
use amethyst::core::ecs::{Dispatcher, DispatcherBuilder};
use amethyst::core::shrev::EventChannel;
use amethyst::core::SystemExt;

pub struct PlayState {
    dispatcher: Dispatcher<'static, 'static>,
    level_file: PathBuf,
}

impl<'a, 'b> PlayState {
    /// Creates a PlayState that starts in demo mode. It loads the demo level.
    pub fn demo() -> Self {
        let level_file = get_levels_dir().join("demo_level.ron");
        PlayState::new(level_file)
    }

    /// Creates a new PlayState that will load the given level.
    pub fn new(level_file: PathBuf) -> Self {
        PlayState {
            level_file,
            dispatcher: DispatcherBuilder::new()
                .with(
                    systems::PlayerSystem::default().pausable(CurrentState::Running),
                    "player_system",
                    &[],
                )
                .with(
                    systems::SteeringSystem::default().pausable(CurrentState::Running),
                    "steering_system",
                    &["player_system"],
                )
                .with(
                    systems::MovementSystem.pausable(CurrentState::Running),
                    "movement_system",
                    &["steering_system"],
                )
                .with(
                    systems::VelocitySystem.pausable(CurrentState::Running),
                    "velocity_system",
                    &["movement_system"],
                )
                .with(
                    systems::RewindControlSystem,
                    "rewind_control_system",
                    &["player_system"],
                )
                .with(
                    systems::RewindSystem.pausable(CurrentState::Rewinding),
                    "rewind_system",
                    &["rewind_control_system"],
                )
                .with(systems::DebugSystem, "debug_system", &[])
                .with(systems::KeyCollectionSystem, "key_collection_system", &[])
                .with(systems::PickupSystem, "pickup_system", &[])
                .with(systems::UseToolSystem, "use_tool_system", &[])
                .with(systems::LevelWrappingSystem, "level_wrapping_system", &[])
                .with(systems::WinSystem, "win_system", &[])
                .build(),
        }
    }

    fn handle_action(&mut self, action: &str, world: &mut World) -> SimpleTrans {
        if action == "speedUp" {
            let (old_scale, new_scale) = (*world.fetch_mut::<DebugSettings>()).increase_speed();
            info!("Speeding up time, from x{:?} to x{:?}. This feature exists for debugging purposes only.", old_scale, new_scale);
            self.update_time_scale(world, new_scale);
            Trans::None
        } else if action == "slowDown" {
            let (old_scale, new_scale) = (*world.fetch_mut::<DebugSettings>()).decrease_speed();
            info!("Slowing down time, from x{:?} to x{:?}. This feature exists for debugging purposes only.", old_scale, new_scale);
            self.update_time_scale(world, new_scale);
            Trans::None
        } else if action == "restart" {
            world
                .write_resource::<EventChannel<SoundEvent>>()
                .single_write(SoundEvent::new(SoundType::LvlReset));
            self.reset_level(world);
            Trans::None
        } else {
            Trans::None
        }
    }

    fn update_time_scale(&self, world: &mut World, time_scale: f32) {
        world.write_resource::<Time>().set_time_scale(time_scale);
    }

    fn reset_level(&self, world: &mut World) {
        world.delete_all();
        UiHandles::add_ui(&UiType::Fps, world);
        UiHandles::add_ui(&UiType::Play, world);
        create_camera(world);
        load_level(&self.level_file, world).expect("Failed to load level!");
    }
}

impl SimpleState for PlayState {
    fn on_start(&mut self, data: StateData<GameData>) {
        info!("PlayState on_start");
        self.dispatcher.setup(data.world);
        self.reset_level(data.world);
    }

    fn on_stop(&mut self, data: StateData<GameData>) {
        info!("PlayState on_stop");
        data.world.delete_all();
    }

    fn handle_event(&mut self, data: StateData<GameData>, event: StateEvent) -> SimpleTrans {
        window_event_handler::handle(&event, data.world);
        match event {
            // Events related to the window and inputs.
            StateEvent::Window(event) => {
                if is_close_requested(&event) {
                    Trans::Quit
                } else if is_key_down(&event, VirtualKeyCode::Escape) {
                    Trans::Pop
                } else {
                    Trans::None
                }
            }
            // Ui event. Button presses, mouse hover, etc...
            StateEvent::Ui(_) => Trans::None,
            StateEvent::Input(input_event) => {
                if let InputEvent::ActionPressed(action) = input_event {
                    self.handle_action(&action, data.world)
                } else {
                    Trans::None
                }
            }
        }
    }

    fn fixed_update(&mut self, data: StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        self.dispatcher.dispatch(&data.world);
        Trans::None
    }

    fn update(&mut self, data: &mut StateData<GameData>) -> SimpleTrans {
        // Execute a pass similar to a system
        data.world.exec(
            #[allow(clippy::type_complexity)]
            |(entities, animation_sets, mut control_sets): (
                Entities,
                ReadStorage<AnimationSet<AnimationId, SpriteRender>>,
                WriteStorage<AnimationControlSet<AnimationId, SpriteRender>>,
            )| {
                // For each entity that has AnimationSet
                for (entity, animation_set) in (&entities, &animation_sets).join() {
                    // Creates a new AnimationControlSet for the entity
                    let control_set = get_animation_set(&mut control_sets, entity).unwrap();
                    // Adds the `Fly` animation to AnimationControlSet and loops infinitely
                    control_set.add_animation(
                        AnimationId::Fly,
                        &animation_set.get(&AnimationId::Fly).unwrap(),
                        EndControl::Loop(None),
                        1.0,
                        AnimationCommand::Start,
                    );
                }
            },
        );
        Trans::None
    }
}
