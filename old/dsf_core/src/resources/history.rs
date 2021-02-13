use crate::components::*;

/// Holds the full history of the current game. Used to rewind games to an earlier point.
///
/// NB: Rewinding is currently considered a debug-only feature. It isn't currently ready for use in
/// real game play. In the future, it will either be made an official feature or removed altogether.
#[derive(Debug)]
pub struct History {
    /// If this is true, then a new Frame should be created this tick, even if nothing changed.
    /// This is used at the start of the game to create the initial Frame, and also after rewinding,
    /// to record the state of the game at that point.
    pub force_key_frame: bool,
    /// A stack of Frames. Each frame records some change in game state.
    frame_stack: Vec<Frame>,
}

impl Default for History {
    fn default() -> History {
        History {
            force_key_frame: true,
            frame_stack: vec![],
        }
    }
}

impl History {
    pub fn push_frame(&mut self, frame: Frame) {
        self.frame_stack.push(frame);
    }

    pub fn pop_frame(&mut self) -> Option<Frame> {
        self.frame_stack.pop()
    }
}

#[derive(Debug)]
pub struct Frame {
    pub player_position: Pos,
}

impl Frame {
    pub fn new(player_position: Pos) -> Self {
        Frame { player_position }
    }
}

/// Used to toggle systems on and off. Some systems can only run if the game is running normally.
/// Some systems can only run if the game is rewinding.
#[derive(Debug, PartialEq)]
pub enum CurrentState {
    Running,
    Rewinding,
}

impl Default for CurrentState {
    fn default() -> Self {
        CurrentState::Running
    }
}

/// Helper resource for the rewinding mechanism.
#[derive(Debug, Default)]
pub struct Rewind {
    /// The time in seconds until a new Frame can be popped off the History.
    pub cooldown: f32,
}

impl Rewind {
    pub fn is_ready(&self) -> bool {
        self.cooldown.is_sign_negative()
    }
}
