use amethyst::prelude::WorldExt;

use amethyst::StateEvent;
use amethyst::{
    animation::{
        get_animation_set, AnimationCommand, AnimationControlSet, AnimationSet, EndControl,
    },
    ecs::{prelude::World, Entities, Join, ReadStorage, WriteStorage},
    input::{is_close_requested, is_key_down, InputEvent, VirtualKeyCode},
    prelude::*,
    renderer::sprite::SpriteRender,
    StateData, Trans,
};
use dsf_precompile::AnimationId;

use crate::resources::*;
use crate::states::file_actions::{auto_save, auto_save_file, load_auto_save};
use crate::systems;

use amethyst::core::ecs::shrev::EventChannel;
use amethyst::core::ecs::{Dispatcher, DispatcherBuilder, Read, Write};
use amethyst::input::StringBindings;

use crate::components::{Cursor, SelectionTag};
use crate::systems::RefreshPreviewsEvent;
use amethyst::core::Transform;
use amethyst::renderer::Transparent;
use dsf_core::components::Pos;
use dsf_core::entities::*;
use dsf_core::levels::*;
use dsf_core::resources::{
    AssetType, Assets, DepthLayer, EventReaders, SpriteType, UiHandles, UiType,
};
use dsf_core::states::{window_event_handler, PlayState};

pub struct EditorState {
    /// Whether this state is currently on top of the stack.
    is_active: bool,
    dispatcher: Dispatcher<'static, 'static>,
}

impl<'a, 'b> EditorState {
    pub fn new(_world: &mut World) -> Self {
        EditorState {
            is_active: false,
            dispatcher: DispatcherBuilder::new()
                .with(systems::PlaceTilesSystem, "place_tile_system", &[])
                .with_barrier()
                .with(
                    systems::EditorUiUpdateSystem,
                    "editor_ui_update_system",
                    &[],
                )
                .with(
                    systems::ConfigureEditorSystem,
                    "configure_editor_system",
                    &[],
                )
                .with(
                    systems::AnimatePreviewsSystem,
                    "animate_previews_system",
                    &[],
                )
                .with(systems::CursorSystem, "cursor_system", &[])
                .with(
                    systems::UpdateBackgroundSystem,
                    "update_background_system",
                    &["cursor_system"],
                )
                .with(
                    systems::SelectionSystem,
                    "selection_system",
                    &["cursor_system"],
                )
                .with(
                    systems::TilePaintSystem,
                    "tile_paint_system",
                    &["selection_system"],
                )
                .build(),
        }
    }

    /// Perform setup that should be executed both upon starting and upon resuming the State.
    fn setup(&self, world: &mut World) {
        init_cursor(world);
        UiHandles::add_ui(&UiType::Editor, world);
        setup_debug_lines(world);
        create_camera(world);
        let tile_defs = load_tile_definitions().expect("Tile definitions failed to load!");
        world
            .write_resource::<EditorStatus>()
            .brush
            .set_palette(&tile_defs);
        let level_edit = LevelEdit::new(load_auto_save(), tile_defs);
        add_background(world, &level_edit.tile_map.world_bounds);
        world.insert(level_edit);
    }
}

impl SimpleState for EditorState {
    fn on_start(&mut self, data: StateData<GameData>) {
        info!("EditorState on_start");
        self.is_active = true;
        let readers = EventReaders::default()
            .add_reader("place_tiles_system".to_string(), data.world)
            .add_reader("choose_brush_system".to_string(), data.world);
        data.world.insert(readers);
        self.dispatcher.setup(data.world);
        data.world.insert(EditorStatus::default());
        self.setup(data.world);
    }

    fn on_stop(&mut self, data: StateData<GameData>) {
        info!("EditorState on_stop");
        data.world.insert(EventReaders::default());
        self.is_active = false;
        data.world.delete_all();
    }

    fn on_pause(&mut self, data: StateData<GameData>) {
        info!("EditorState on_pause");
        self.is_active = false;
        data.world.delete_all();
    }

    fn on_resume(&mut self, data: StateData<GameData>) {
        info!("EditorState on_resume");
        self.is_active = true;
        self.setup(data.world);
    }

    fn handle_event(&mut self, data: StateData<GameData>, event: StateEvent) -> SimpleTrans {
        window_event_handler::handle(&event, data.world);
        match event {
            // Events related to the window and inputs.
            StateEvent::Window(event) => {
                if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                    auto_save(data.world).expect("Failed to auto-save level!");
                    Trans::Pop
                } else {
                    Trans::None
                }
            }
            // Ui event. Button presses, mouse hover, etc...
            StateEvent::Ui(_) => Trans::None,
            StateEvent::Input(input_event) => match input_event {
                InputEvent::KeyReleased {
                    key_code: VirtualKeyCode::F1,
                    scancode: _,
                } => {
                    auto_save(data.world).expect("Failed to auto-save level!");
                    Trans::Push(Box::new(PlayState::new(auto_save_file())))
                }
                _ => Trans::None,
            },
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

    /// If this State is not active, then the systems associated with its dispatcher will not be
    /// able to drain the event channels that they are registered to. This is a problem,
    /// because events will start bunching up.
    ///
    /// To solve this, we drain the event channel in the shadow update.
    fn shadow_update(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        if !self.is_active {
            data.world.exec(
                |(mut readers, channel): (
                    Write<EventReaders>,
                    Read<EventChannel<InputEvent<StringBindings>>>,
                )| {
                    readers.drain_event_channel(channel);
                },
            );
        }
    }
}

/// TODO: Temporary function, clean up later.
/// Adds a selection and a cursor entity.
fn init_cursor(world: &mut World) {
    let sprite_handle = world
        .read_resource::<Assets>()
        .get_still(&SpriteType::Selection);
    let mut selection_transform = Transform::default();
    selection_transform.set_translation_z((&DepthLayer::Selection).z());
    world
        .create_entity()
        .with(SpriteRender {
            sprite_sheet: sprite_handle,
            sprite_number: 1,
        })
        .with(Transparent)
        .with(selection_transform)
        .with(SelectionTag)
        .build();
    let mut cursor_transform = Transform::default();
    cursor_transform.set_translation_xyz(0.5, 0.5, DepthLayer::Cursor.z());
    let cursor_entity = world
        .create_entity()
        .with(cursor_transform)
        .with(Cursor::default())
        .build();
    attach_graphics(
        world,
        cursor_entity,
        &AssetType::Still(SpriteType::Selection, 0),
        &Pos::new(1, 1),
        None,
    );
    world.exec(|mut channel: Write<EventChannel<RefreshPreviewsEvent>>| {
        channel.single_write(RefreshPreviewsEvent);
    });
}
