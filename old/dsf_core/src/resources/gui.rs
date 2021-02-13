use std::collections::HashMap;

use amethyst::prelude::WorldExt;

use amethyst::ui::UiPrefab;
use serde::{Deserialize, Serialize};

use amethyst::{assets::Handle, ecs::prelude::Entity, prelude::*};

/// This resource stores handles to UI prefabs that were loaded in the LoadingState.
#[derive(Default, Debug)]
pub struct UiHandles {
    map: HashMap<UiType, Handle<UiPrefab>>,
}

impl UiHandles {
    pub fn put_handle(mut self, key: UiType, handle: Handle<UiPrefab>) -> Self {
        self.map.insert(key, handle);
        self
    }

    fn clone_handle(&self, key: &UiType) -> Option<Handle<UiPrefab>> {
        self.map
            .get(key)
            .or_else(|| {
                error!("Tried using UI element {:?} but that element was not loaded! To use this element, add it to the LoadingConfig.", key);
                None
            })
            .map(|handle| (*handle).clone())
    }

    /// Convenience method that grabs the correct UiHandle and uses it to create an entity.
    /// This is the recommended way to create a GUI.
    pub fn add_ui(key: &UiType, world: &mut World) -> Option<Entity> {
        let handle = world.read_resource::<UiHandles>().clone_handle(key);
        handle.map(|handle| world.create_entity().with(handle).build())
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub enum UiType {
    /// Small debug FPS meter.
    Fps,
    /// A message in the center of the screen that tells you that you completed the level.
    WinMessage,
    /// Dialog that pops up when you want to save a level in the editor.
    Save,
    /// Ui for the level editor.
    Editor,
    /// The paused menu. Not currently in use, but will be implemented in the future.
    Paused,
    /// The main menu.
    MainMenu,
    /// For the adventure and level select screen. Contains the details of the selected node,
    /// such as name and description.
    LevelSelect,
    /// Debug controls explanation. Tells players that F5 resets level.
    Play,
    Settings,
}
