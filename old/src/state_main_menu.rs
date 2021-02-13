use amethyst::{
    ecs::prelude::{Entity, WorldExt},
    input::{is_close_requested, is_key_down, VirtualKeyCode},
    ui::{UiEvent, UiEventType, UiFinder},
    GameData, SimpleState, SimpleTrans, StateData, StateEvent, Trans,
};

use dsf_core::resources::{DebugSettings, UiHandles, UiType};
use dsf_core::states::{window_event_handler, LevelSelectState, SettingsState};
use dsf_editor::states::EditorState;

const PLAY_BUTTON_ID: &str = "play";
const EDITOR_BUTTON_ID: &str = "editor";
const SETTINGS_BUTTON_ID: &str = "settings";
const EXIT_BUTTON_ID: &str = "exit";

#[derive(Default)]
pub struct MainMenuState {
    ui: Option<Entity>,
    play_button: Option<Entity>,
    editor_button: Option<Entity>,
    settings_button: Option<Entity>,
    exit_button: Option<Entity>,
}

impl MainMenuState {
    pub fn new() -> MainMenuState {
        MainMenuState::default()
    }

    fn init_ui(&mut self, data: StateData<GameData>) {
        UiHandles::add_ui(&UiType::Fps, data.world);
        self.ui = UiHandles::add_ui(&UiType::MainMenu, data.world);
        // invoke a world update to finish creating our ui entities
        data.data.update(&data.world);
        // look up our buttons
        data.world.exec(|ui_finder: UiFinder<'_>| {
            self.play_button = ui_finder.find(PLAY_BUTTON_ID);
            self.editor_button = ui_finder.find(EDITOR_BUTTON_ID);
            self.settings_button = ui_finder.find(SETTINGS_BUTTON_ID);
            self.exit_button = ui_finder.find(EXIT_BUTTON_ID);
        });
    }
}

impl SimpleState for MainMenuState {
    fn on_start(&mut self, data: StateData<GameData>) {
        info!("MainMenuState on_start");
        self.init_ui(data);
    }

    fn on_pause(&mut self, data: StateData<GameData>) {
        info!("MainMenuState on_pause");
        data.world.delete_all();
        self.play_button = None;
        self.editor_button = None;
        self.settings_button = None;
        self.exit_button = None;
    }

    fn on_resume(&mut self, data: StateData<GameData>) {
        info!("MainMenuState on_resume");
        self.init_ui(data);
    }

    fn handle_event(&mut self, data: StateData<GameData>, event: StateEvent) -> SimpleTrans {
        window_event_handler::handle(&event, data.world);
        match event {
            StateEvent::Window(event) => {
                if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                    Trans::Quit
                } else {
                    Trans::None
                }
            }
            StateEvent::Ui(UiEvent {
                event_type: UiEventType::Click,
                target,
            }) => {
                if Some(target) == self.play_button {
                    Trans::Push(Box::new(LevelSelectState::demo()))
                } else if Some(target) == self.editor_button {
                    Trans::Push(Box::new(EditorState::new(data.world)))
                } else if Some(target) == self.settings_button {
                    Trans::Push(Box::new(SettingsState::default()))
                } else if Some(target) == self.exit_button {
                    Trans::Quit
                } else {
                    Trans::None
                }
            }
            _ => Trans::None,
        }
    }

    fn update(&mut self, data: &mut StateData<GameData>) -> SimpleTrans {
        let skip_straight_to_editor =
            (*data.world.read_resource::<DebugSettings>()).skip_straight_to_editor;
        if skip_straight_to_editor {
            info!("Bypassing main menu, skipping straight to editor.");
            (*data.world.write_resource::<DebugSettings>()).skip_straight_to_editor = false;
            Trans::Push(Box::new(EditorState::new(data.world)))
        } else {
            Trans::None
        }
    }
}
