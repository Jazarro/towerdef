use amethyst::utils::application_root_dir;
use std::fs;
use std::path::PathBuf;

pub fn get_default_settings_dir() -> PathBuf {
    create_if_missing(get_config_dir().join("default_settings/"))
}

pub fn get_config_dir() -> PathBuf {
    create_if_missing(get_assets_dir().join("config/"))
}

pub fn get_adventures_dir() -> PathBuf {
    create_if_missing(get_world_dir().join("adventures/"))
}

pub fn get_levels_dir() -> PathBuf {
    create_if_missing(get_world_dir().join("levels/"))
}

pub fn get_world_dir() -> PathBuf {
    get_assets_dir().join("world/")
}

pub fn get_assets_dir() -> PathBuf {
    get_root_dir().join("assets/")
}

fn get_root_dir() -> PathBuf {
    application_root_dir().expect("Root directory not found!")
}

fn create_if_missing(path: PathBuf) -> PathBuf {
    fs::create_dir_all(&path).unwrap_or_else(|err| {
        panic!(
            "Failed to create directory {:?} because error {:?}",
            &path, err
        )
    });
    path
}

/// This directory contains transient user data. That includes player settings, key bindings,
/// cache files, save files, etc.
/// This directory will not be in git. It will be empty (or not even exist) the first time any
/// player starts up the game.
fn get_user_data_dir() -> PathBuf {
    create_if_missing(get_root_dir().join(".userdata/"))
}

pub fn get_user_cache_file() -> PathBuf {
    get_user_data_dir().join("cache.ron")
}

pub fn get_user_settings_dir() -> PathBuf {
    create_if_missing(get_user_data_dir().join("settings/"))
}
