use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(default)]
#[serde(deny_unknown_fields)]
pub struct MovementConfig {
    /// The max speed of the player in meters per second.
    pub player_speed: f32,
    /// How many seconds can pass between starting your jump and starting to move sideways for it to
    /// still register. If you start moving sideways later than that, it will not work and the
    /// character will simply jump straight up into the air instead.
    pub jump_allowance: f32,
    /// How many seconds must pass after turning around whilst standing still before the character
    /// starts walking. This gives the player a bit of time to let go of the walking controls if
    /// they just want to turn around, but not want to start walking.
    pub turn_allowance: f32,
    /// When the player first starts pressing down a movement key (e.g. RIGHT), how many seconds
    /// does it take between moving the first step and moving the second step? The first step is
    /// taken instantly, the second step takes a while. This prevents a single key press registering
    /// as more than one step.
    pub map_cursor_move_high_cooldown: f32,
    /// When the player is holding down a movement key (e.g. RIGHT), how many seconds between two
    /// steps? The first step takes longer, that's what the high cooldown is for. Each subsequent
    /// step takes much shorter.
    pub map_cursor_move_low_cooldown: f32,
}
