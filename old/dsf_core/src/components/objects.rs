use crate::components::Pos;
use crate::resources::{SpriteType, ToolType};
use amethyst::core::ecs::{HashMapStorage, NullStorage, VecStorage};
use amethyst::{
    assets::PrefabData,
    derive::PrefabData,
    ecs::{prelude::Entity, Component, WriteStorage},
    error::Error,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, PrefabData)]
#[prefab(Component)]
pub struct Key {
    pub pos: Pos,
}

impl Component for Key {
    type Storage = HashMapStorage<Self>;
}

impl Key {
    pub fn new(pos: Pos) -> Self {
        Key { pos }
    }
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, PrefabData)]
#[prefab(Component)]
pub struct Tool {
    pub tool_type: ToolType,
    pub sprite: SpriteType,
    pub sprite_nr: usize,
}

impl Component for Tool {
    type Storage = HashMapStorage<Self>;
}

impl Tool {
    pub fn new(tool_type: ToolType, sprite: SpriteType, sprite_nr: usize) -> Self {
        Tool {
            tool_type,
            sprite,
            sprite_nr,
        }
    }
}

/// All destructible entities must have this component, this is how we find and delete them.
#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, PrefabData)]
#[prefab(Component)]
pub struct Block {
    pub pos: Pos,
}

impl Component for Block {
    type Storage = VecStorage<Self>;
}

/// A miniature version of every key is found on the exit door.
#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, PrefabData)]
#[prefab(Component)]
pub struct KeyDisplay {
    /// The position of the corresponding key in the world. NOT the actual position of this display.
    /// The display is a miniature version of the key located somewhere on top of the door.
    pub pos: Pos,
}

impl Component for KeyDisplay {
    type Storage = HashMapStorage<Self>;
}

impl KeyDisplay {
    pub fn new(pos: Pos) -> Self {
        KeyDisplay { pos }
    }
}

/// The exit door.
#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, PrefabData)]
#[prefab(Component)]
pub struct ExitDoor;

impl Component for ExitDoor {
    type Storage = NullStorage<Self>;
}

/// The blue background sprite.
#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, PrefabData)]
#[prefab(Component)]
pub struct BackgroundTag;

impl Component for BackgroundTag {
    type Storage = NullStorage<Self>;
}
