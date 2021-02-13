use amethyst::prelude::WorldExt;

use amethyst::StateEvent;
use amethyst::{
    animation::{
        get_animation_set, AnimationCommand, AnimationControlSet, AnimationSet, EndControl,
    },
    ecs::{Entities, Join, ReadStorage, WriteStorage},
    input::{is_close_requested, is_key_down, InputEvent, VirtualKeyCode},
    renderer::sprite::SpriteRender,
    StateData, Trans,
};
use amethyst::{GameData, SimpleState, SimpleTrans};

use dsf_precompile::AnimationId;

use crate::components::*;

use crate::states::setup_test;
use crate::systems;
use amethyst::core::ecs::{Dispatcher, DispatcherBuilder};
use dsf_core::entities::create_camera;
use dsf_core::resources::{History, UiHandles, UiType};
use dsf_core::states::window_event_handler;

pub struct MovementTestState {
    dispatcher: Dispatcher<'static, 'static>,
}

impl Default for MovementTestState {
    fn default() -> Self {
        MovementTestState {
            dispatcher: DispatcherBuilder::new()
                .with(
                    systems::TestSetupSystem::default(),
                    "test_setup_system",
                    &[],
                )
                .build(),
        }
    }
}

impl SimpleState for MovementTestState {
    fn on_start(&mut self, data: StateData<GameData>) {
        info!("MovementTestState on_start");
        self.dispatcher.setup(data.world);
        UiHandles::add_ui(&UiType::Fps, data.world);
        create_camera(data.world);
        data.world.insert(History::default());
    }

    fn on_stop(&mut self, data: StateData<GameData>) {
        info!("MovementTestState on_stop");
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
                match input_event {
                    InputEvent::KeyPressed {
                        key_code: VirtualKeyCode::Key1,
                        ..
                    } => setup_test(MovementTest::Jump2Wide, data.world),
                    InputEvent::KeyPressed {
                        key_code: VirtualKeyCode::Key2,
                        ..
                    } => setup_test(MovementTest::Jump4Wide, data.world),
                    InputEvent::KeyPressed {
                        key_code: VirtualKeyCode::Key3,
                        ..
                    } => {
                        //TODO:...
                    }
                    _ => (),
                };
                Trans::None
            }
        }
    }

    fn update(&mut self, data: &mut StateData<GameData>) -> SimpleTrans {
        self.dispatcher.dispatch(&data.world);
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
