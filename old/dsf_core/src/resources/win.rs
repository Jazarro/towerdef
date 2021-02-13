use std::collections::HashSet;

use crate::components::*;

/// Maintains some information related to winning the level.
/// In any given level, the player must collect all keys. Once all keys are collected, the exit door
/// opens. When the player then reaches the door, they complete the level.
#[derive(Debug, Default)]
pub struct WinCondition {
    /// The set of positions of keys that are left in the level. If this collection is empty, then
    /// the player has collected all keys and is free to finish the level by reaching the exit door.
    pub keys: HashSet<Pos>,
    /// This is set to true when the player has collected all keys and then subsequently reached
    /// the exit door. If this is true, the player has completed the level.
    pub reached_open_door: bool,
}

impl WinCondition {
    /// Add a key. Only to be used when loading a level.
    pub fn add_key(&mut self, pos: &Pos) {
        self.keys.insert(*pos);
    }
    /// How many keys are left uncollected in the level.
    pub fn nr_keys_left(&self) -> usize {
        self.keys.len()
    }
    /// Sets the key at the given position as collected.
    pub fn set_key_collected(&mut self, pos: &Pos) {
        self.keys.remove(pos);
    }
    /// Whether or not the player has collected all keys.
    /// If this returns true, the door is open and once the player reaches it they win the level.
    pub fn all_keys_collected(&self) -> bool {
        self.keys.is_empty()
    }
}
