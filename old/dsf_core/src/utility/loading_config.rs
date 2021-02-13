use crate::resources::{AnimType, SoundType, SpriteType, UiType};
use serde::{Deserialize, Serialize};

/// This specifies all assets that must be loaded by the LoadingState.
#[derive(Debug, Deserialize, Serialize)]
#[serde(default)]
#[serde(deny_unknown_fields)]
pub struct LoadingConfig {
    pub uis: Vec<(UiType, String)>,
    pub animations: Vec<(AnimType, String)>,
    pub stills: Vec<(SpriteType, String, String)>,
    pub sound_effects: Vec<(SoundType, String)>,
    pub music_tracks: Vec<String>,
}

impl Default for LoadingConfig {
    fn default() -> Self {
        LoadingConfig {
            uis: vec![],
            animations: vec![(AnimType::NotFound, "prefab/anim_not_found.ron".to_string())],
            stills: vec![(
                SpriteType::NotFound,
                "textures/not_found.png".to_string(),
                "prefab/still_not_found.ron".to_string(),
            )],
            sound_effects: vec![],
            music_tracks: vec![],
        }
    }
}
