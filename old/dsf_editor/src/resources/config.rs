use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(default)]
#[serde(deny_unknown_fields)]
pub struct EditorConfig {
    pub cursor_move_high_cooldown: f32,
    pub cursor_move_low_cooldown: f32,
    /// Time in seconds that the cursor is visible during its blinking animation.
    pub cursor_blink_on_time: f32,
    /// Time in seconds that the cursor is invisible during its blinking animation.
    pub cursor_blink_off_time: f32,
}
