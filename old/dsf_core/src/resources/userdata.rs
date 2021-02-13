use crate::components::Pos;
use crate::utility::files::get_user_cache_file;
use amethyst::config::Config;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// These are some transient values to improve user experience.
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct UserCache {
    /// This keeps track of the player's map cursor position for every adventure.
    /// This maps the adventure file name (e.g. "default.ron") to the last position the
    /// player's cursor was at.
    pub adventure_map_pos: HashMap<String, Pos>,
}

impl UserCache {
    pub fn save_adventure_map_pos(&mut self, adventure_file_name: String, pos: Pos) {
        self.adventure_map_pos.insert(adventure_file_name, pos);
        self.write(get_user_cache_file()).unwrap_or_else(|err| {
            error!("Failed to save {:?} because error: {:?}", self, err);
        });
    }

    pub fn get_initial_cursor_pos(&self, adventure_file_name: &str) -> Pos {
        self.adventure_map_pos
            .get(adventure_file_name)
            .copied()
            .unwrap_or_else(Pos::default)
    }
}
