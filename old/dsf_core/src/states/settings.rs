use crate::resources::{AudioSettings, UiHandles, UiType};
use crate::states::window_event_handler;
use amethyst::core::ecs::{Read, World, WriteStorage};
use amethyst::ui::UiText;
use amethyst::{
    ecs::prelude::{Entity, WorldExt},
    input::{is_close_requested, is_key_down, VirtualKeyCode},
    ui::{UiEvent, UiEventType, UiFinder},
    GameData, SimpleState, SimpleTrans, StateData, StateEvent, Trans,
};

const INCREASE_MUSIC_VOLUME_BUTTON_ID: &str = "btn_increase_music_volume";
const DECREASE_MUSIC_VOLUME_BUTTON_ID: &str = "btn_decrease_music_volume";
const MUSIC_VOLUME_LABEL_ID: &str = "label_music_volume";
const INCREASE_SFX_VOLUME_BUTTON_ID: &str = "btn_increase_sfx_volume";
const DECREASE_SFX_VOLUME_BUTTON_ID: &str = "btn_decrease_sfx_volume";
const SFX_VOLUME_LABEL_ID: &str = "label_sfx_volume";

#[derive(Default)]
pub struct SettingsState {
    ui: Option<Entity>,
    btn_increase_music_volume: Option<Entity>,
    btn_decrease_music_volume: Option<Entity>,
    label_music_volume: Option<Entity>,
    btn_increase_sfx_volume: Option<Entity>,
    btn_decrease_sfx_volume: Option<Entity>,
    label_sfx_volume: Option<Entity>,
}

impl SettingsState {
    fn init_ui(&mut self, data: StateData<GameData>) {
        UiHandles::add_ui(&UiType::Fps, data.world);
        self.ui = UiHandles::add_ui(&UiType::Settings, data.world);
        // invoke a world update to finish creating our ui entities
        data.data.update(&data.world);
        // look up our buttons
        data.world.exec(|ui_finder: UiFinder<'_>| {
            self.btn_increase_music_volume = ui_finder.find(INCREASE_MUSIC_VOLUME_BUTTON_ID);
            self.btn_decrease_music_volume = ui_finder.find(DECREASE_MUSIC_VOLUME_BUTTON_ID);
            self.label_music_volume = ui_finder.find(MUSIC_VOLUME_LABEL_ID);
            self.btn_increase_sfx_volume = ui_finder.find(INCREASE_SFX_VOLUME_BUTTON_ID);
            self.btn_decrease_sfx_volume = ui_finder.find(DECREASE_SFX_VOLUME_BUTTON_ID);
            self.label_sfx_volume = ui_finder.find(SFX_VOLUME_LABEL_ID);
        });
        self.set_labels(data.world);
    }
    fn set_labels(&self, world: &mut World) {
        if let Some(label_entity) = self.label_music_volume {
            world.exec(
                |(mut ui_text, audio_config): (WriteStorage<UiText>, Read<AudioSettings>)| {
                    if let Some(mut text_component) = ui_text.get_mut(label_entity) {
                        text_component.text =
                            format!("Music volume: {:}", audio_config.format_music_volume());
                    }
                },
            );
        }
        if let Some(label_entity) = self.label_sfx_volume {
            world.exec(
                |(mut ui_text, audio_config): (WriteStorage<UiText>, Read<AudioSettings>)| {
                    if let Some(mut text_component) = ui_text.get_mut(label_entity) {
                        text_component.text = format!(
                            "Sound effects volume: {:}",
                            audio_config.format_sfx_volume()
                        );
                    }
                },
            );
        }
    }

    fn handle_btn_click(&mut self, target: Entity, world: &mut World) {
        if Some(target) == self.btn_increase_music_volume {
            world
                .write_resource::<AudioSettings>()
                .add_to_music_volume(0.1);
            self.set_labels(world);
        } else if Some(target) == self.btn_decrease_music_volume {
            world
                .write_resource::<AudioSettings>()
                .add_to_music_volume(-0.1);
            self.set_labels(world);
        } else if Some(target) == self.btn_increase_sfx_volume {
            world
                .write_resource::<AudioSettings>()
                .add_to_sfx_volume(0.1);
            self.set_labels(world);
        } else if Some(target) == self.btn_decrease_sfx_volume {
            world
                .write_resource::<AudioSettings>()
                .add_to_sfx_volume(-0.1);
            self.set_labels(world);
        }
    }
}

impl SimpleState for SettingsState {
    fn on_start(&mut self, data: StateData<GameData>) {
        info!("SettingsState on_start");
        self.init_ui(data);
    }

    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        info!("SettingsState on_pause");
        data.world.delete_all();
    }

    fn on_pause(&mut self, data: StateData<GameData>) {
        info!("SettingsState on_pause");
        data.world.delete_all();
        self.btn_increase_music_volume = None;
        self.btn_decrease_music_volume = None;
        self.label_music_volume = None;
        self.btn_increase_sfx_volume = None;
        self.btn_decrease_sfx_volume = None;
        self.label_sfx_volume = None;
    }

    fn on_resume(&mut self, data: StateData<GameData>) {
        info!("SettingsState on_resume");
        self.init_ui(data);
    }

    fn handle_event(&mut self, data: StateData<GameData>, event: StateEvent) -> SimpleTrans {
        window_event_handler::handle(&event, data.world);
        match event {
            StateEvent::Window(event) => {
                if is_close_requested(&event) {
                    Trans::Quit
                } else if is_key_down(&event, VirtualKeyCode::Escape) {
                    Trans::Pop
                } else {
                    Trans::None
                }
            }
            StateEvent::Ui(UiEvent {
                event_type: UiEventType::Click,
                target,
            }) => {
                self.handle_btn_click(target, data.world);
                Trans::None
            }
            _ => Trans::None,
        }
    }
}
