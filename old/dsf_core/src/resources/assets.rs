use crate::components::Pos;
use amethyst::audio::SourceHandle;
use amethyst::{
    assets::{Handle, Prefab},
    renderer::SpriteSheet,
};
use dsf_precompile::MyPrefabData;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct Assets {
    stills: HashMap<SpriteType, Handle<SpriteSheet>>,
    animated: HashMap<AnimType, Handle<Prefab<MyPrefabData>>>,
    sounds: HashMap<SoundType, Vec<SourceHandle>>,
}

impl Assets {
    pub fn put_still(mut self, asset_type: SpriteType, asset: Handle<SpriteSheet>) -> Self {
        self.stills.insert(asset_type, asset);
        self
    }

    pub fn put_animated(
        mut self,
        asset_type: AnimType,
        asset: Handle<Prefab<MyPrefabData>>,
    ) -> Self {
        self.animated.insert(asset_type, asset);
        self
    }

    pub fn put_sound(mut self, sound_type: SoundType, asset: SourceHandle) -> Self {
        self.sounds
            .entry(sound_type)
            .or_insert_with(Vec::new)
            .push(asset);
        self
    }

    pub fn get_still(&self, asset_type: &SpriteType) -> Handle<SpriteSheet> {
        (*self
            .stills
            .get(asset_type)
            .or_else(|| {
                error!("Spritesheet asset {:?} is missing!", asset_type);
                self.stills.get(&SpriteType::NotFound)
            })
            .expect("Fallback asset also missing."))
        .clone()
    }

    pub fn get_animated(&self, asset_type: &AnimType) -> Handle<Prefab<MyPrefabData>> {
        (*self
            .animated
            .get(asset_type)
            .or_else(|| {
                error!("Animation asset {:?} is missing!", asset_type);
                self.animated.get(&AnimType::NotFound)
            })
            .expect("Fallback asset also missing!"))
        .clone()
    }

    pub fn get_sound(&self, asset_type: &SoundType) -> Option<SourceHandle> {
        self
            .sounds
            .get(asset_type)
            .or_else(|| {
                error!("There are no sounds of type {:?}. Add them to the LoadingConfig to start using them.", asset_type);
                None
            })
            .map(|sounds_of_that_type| {
                let random_index = rand::thread_rng().gen_range(0, sounds_of_that_type.len());
                (*sounds_of_that_type.get(random_index).expect("Should not panic.")).clone()
            })
    }
}

#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
pub enum AssetType {
    /// A static, non-animated image.
    /// Contains both a handle to the sprite sheet and the number of the sprite on the sheet.
    Still(SpriteType, usize),
    /// An animated image.
    Animated(AnimType),
}

impl Default for AssetType {
    fn default() -> Self {
        AssetType::Still(SpriteType::default(), 0)
    }
}

#[derive(Debug, Copy, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub enum SpriteType {
    /// This is the fallback sprite to use if the desired sprite cannot be found.
    NotFound,
    Ladder,
    Frame,
    Blocks,
    Tools,
    Door,
    Selection,
    LevelSelect,
    EditorUiIcons,
    Miner,
}

impl Default for SpriteType {
    fn default() -> Self {
        SpriteType::NotFound
    }
}

#[derive(Debug, Copy, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub enum AnimType {
    /// The fallback animated asset to use if the desired asset could not be found.
    NotFound,
    Miner,
}

/// Matches a still or animated asset to its dimensions in pixels. Required to calculate the
/// correct scale factor for the entity to make it fit within its in-world bounds.
#[allow(clippy::match_single_binding)]
pub fn get_asset_dimensions(asset: &AssetType) -> Pos {
    match asset {
        AssetType::Still(sprite_type, _) => match sprite_type {
            SpriteType::Frame => Pos::new(50, 50),
            SpriteType::Ladder => Pos::new(128, 64),
            SpriteType::Door => Pos::new(256, 256),
            _ => Pos::new(128, 128),
        },
        AssetType::Animated(anim_type) => match anim_type {
            _ => Pos::new(128, 128),
        },
    }
}

/// Identifies a type of sound effect. Each of these sound types could be represented by any number
/// of sound files that the game will randomly pick from.
#[derive(Debug, Copy, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub enum SoundType {
    /// Sound will be played when the player initiates a jump.
    Jump,
    /// The player's footstep. Sound file must be a single footstep. Sound must not be too loud or
    /// noticeable.
    Step,
    /// One step while climbing on a ladder. Sound file must be just a single footstep. Sound must
    /// not be too loud or noticeable.
    LadderStep,
    /// One step when on the adventure and level selection screen.
    MapStep,
    /// This will be played when the player tries something that is not possible, such as trying to
    /// jump while underneath a 2-high ledge.
    CannotPerformAction,
    /// When the player starts to use a mining tool, ie: a tool that breaks blocks.
    Mining,
    /// Played when the player picks up any tool.
    ToolPickup,
    /// Played when the player picks up a key.
    KeyPickup,
    /// Played when the player completes a level by reaching the exit door after having picked up
    /// all keys.
    Win,
    /// Plays when the player resets the puzzle to the beginning
    /// (probably because they made a mistake).
    LvlReset,
}
