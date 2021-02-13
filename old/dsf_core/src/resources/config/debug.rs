use crate::utility::files::{get_default_settings_dir, get_user_settings_dir};
use amethyst::prelude::Config;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(default)]
#[serde(deny_unknown_fields)]
pub struct DebugSettings {
    /// An array of values that 'time_scale' can have.
    /// Debug controls will allow switching between these values,
    /// to slow time down or speed it up.
    pub time_scale_presets: Vec<f32>,
    /// How fast the clock is ticking. A value of 1.0 means time is
    /// behaving normally, higher values mean time is sped up and
    /// 0.0 means time is frozen.
    pub time_scale: f32,
    /// Number of seconds to leave between frames when rewinding time.
    pub seconds_per_rewind_frame: f32,
    /// Enable this when debugging, to save time when rapidly iterating.
    /// It saves you from having to navigate the menu every time you start the game.
    /// If true, the game will open in the editor state.
    /// If false, it will open on the main menu.
    pub skip_straight_to_editor: bool,
    /// Whether or not to display debug frames indicating the player's discrete position.
    pub display_debug_frames: bool,
}

impl DebugSettings {
    /// Increase the time scale. Everything in the world will move more quickly.
    /// Return a tuple containing the old scale and the new scale.
    /// If the time is already operating at the fastest speed, the time scale will not change.
    pub fn increase_speed(&mut self) -> (f32, f32) {
        let old_time_scale = self.time_scale;
        let new_time_scale = self
            .time_scale_presets
            .iter()
            .find(|&&scale| scale > self.time_scale);
        if let Some(new_time_scale) = new_time_scale {
            self.time_scale = *new_time_scale;
            (old_time_scale, self.time_scale)
        } else {
            (self.time_scale, self.time_scale)
        }
    }

    /// Decrease the time scale. Everything in the world will move more slowly.
    /// Return a tuple containing the old scale and the new scale.
    /// If the time is already operating at the slowest speed, the time scale will not change.
    pub fn decrease_speed(&mut self) -> (f32, f32) {
        let old_time_scale = self.time_scale;
        let new_time_scale = self
            .time_scale_presets
            .iter()
            .rev()
            .find(|&&scale| scale < self.time_scale);
        if let Some(new_time_scale) = new_time_scale {
            self.time_scale = *new_time_scale;
            (old_time_scale, self.time_scale)
        } else {
            (self.time_scale, self.time_scale)
        }
    }
}

/// Loads the most relevant instance of DebugSettings.
///
/// If the user DebugSettings file exists, tries to load from user settings first. If that fails,
/// log an error and try to load from default settings.
///
/// If the default DebugSettings file fails to load, fall back to the Default trait implementation
/// as a last resort (ie: DebugSettings::default()).
pub fn load_debug_settings() -> DebugSettings {
    let user_settings_file = get_user_settings_dir().join("debug.ron");
    if user_settings_file.exists() {
        load_debug_user_settings(&user_settings_file)
    } else {
        load_debug_default_settings()
    }
}

fn load_debug_user_settings(file_path: &PathBuf) -> DebugSettings {
    DebugSettings::load(&file_path).unwrap_or_else(|error| {
        error!(
            "Failed to load the user-specific debug settings file from {:?}! Falling back to default settings file. Error: {:?}",
            file_path, error
        );
        load_debug_default_settings()
    })
}

fn load_debug_default_settings() -> DebugSettings {
    let file = get_default_settings_dir().join("debug.ron");
    DebugSettings::load(&file).unwrap_or_else(
        |error| {
            error!(
                "Failed to load the default debug settings file from {:?}! Falling back to Default implementation. Error: {:?}",
                file, error
            );
            DebugSettings::default()
        },
    )
}
