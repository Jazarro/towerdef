use crate::utility::files::{get_default_settings_dir, get_user_settings_dir};
use amethyst::prelude::Config;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
#[serde(default)]
#[serde(deny_unknown_fields)]
pub struct AudioSettings {
    /// What volume the music should be played at. If this value is None, the music will not be
    /// played at all.
    /// The volume should be a value in the range [0.0, 1.0].
    pub music_volume: Option<f32>,
    /// What volume the sound effects should be played at. If this value is None, the music will
    /// not be played at all.
    /// The volume should be a value in the range [0.0, 1.0].
    pub sound_effects_volume: Option<f32>,
}

/// Set some sensible values for the audio settings fallback.
///
/// These will only be used if the settings cannot be loaded from either the user settings or
/// the default settings files. Under normal circumstances, these values WILL NOT BE USED.
///
/// To change the default settings, check out the assets/config/default_settings/audio.ron file.
impl Default for AudioSettings {
    fn default() -> Self {
        AudioSettings {
            music_volume: Some(0.5),
            sound_effects_volume: Some(0.5),
        }
    }
}

impl AudioSettings {
    /// Add the given delta to the current music volume and write the SoundConfig to a user
    /// settings file.
    pub fn add_to_music_volume(&mut self, delta: f32) {
        self.music_volume = Self::add_volume(self.music_volume, delta);
        self.write(get_user_settings_dir().join("audio.ron"))
            .expect("Failed to write user audio settings file.");
    }

    /// Add the given delta to the current sound effects volume and write the SoundConfig to a user
    /// settings file.
    pub fn add_to_sfx_volume(&mut self, delta: f32) {
        self.sound_effects_volume = Self::add_volume(self.sound_effects_volume, delta);
        self.write(get_user_settings_dir().join("audio.ron"))
            .expect("Failed to write user audio settings file.");
    }

    /// Add the delta to the starting volume. Clamp to range [0, 1].
    /// A value of zero is interpreted as None (sound off).
    fn add_volume(starting_volume: Option<f32>, delta: f32) -> Option<f32> {
        let current_volume = starting_volume.unwrap_or(0.0);
        let new_volume = (current_volume + delta).max(0.0).min(1.0);
        Some(new_volume).and_then(|volume| {
            if volume.abs() < f32::EPSILON {
                None
            } else {
                Some(volume)
            }
        })
    }

    /// Return a pretty printed representation of the music volume.
    pub fn format_music_volume(&self) -> String {
        Self::format_volume(self.music_volume)
    }

    /// Return a pretty printed representation of the sound effects volume.
    pub fn format_sfx_volume(&self) -> String {
        Self::format_volume(self.sound_effects_volume)
    }

    /// Return a pretty printed representation of the given volume value.
    fn format_volume(volume: Option<f32>) -> String {
        match volume {
            Some(volume) => format!("{:.2}", volume),
            None => "Off".to_string(),
        }
    }
}

/// Loads the most relevant instance of AudioSettings.
///
/// If the user AudioSettings file exists, tries to load from user settings first. If that fails,
/// log an error and try to load from default settings.
///
/// If the default AudioSettings file fails to load, fall back to the Default trait implementation
/// as a last resort (ie: AudioSettings::default()).
pub fn load_audio_settings() -> AudioSettings {
    let user_settings_file = get_user_settings_dir().join("audio.ron");
    if user_settings_file.exists() {
        load_audio_user_settings(&user_settings_file)
    } else {
        load_audio_default_settings()
    }
}

fn load_audio_user_settings(file_path: &PathBuf) -> AudioSettings {
    AudioSettings::load(&file_path).unwrap_or_else(|error| {
        error!(
            "Failed to load the user-specific audio settings file from {:?}! Falling back to default settings file. Error: {:?}",
            file_path, error
        );
        load_audio_default_settings()
    })
}

fn load_audio_default_settings() -> AudioSettings {
    let file = get_default_settings_dir().join("audio.ron");
    AudioSettings::load(&file).unwrap_or_else(
        |error| {
            error!(
                "Failed to load the default audio settings file from {:?}! Falling back to Default implementation. Error: {:?}",
                file, error
            );
            AudioSettings::default()
        },
    )
}
