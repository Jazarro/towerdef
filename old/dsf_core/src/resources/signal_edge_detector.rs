use amethyst::input::{InputHandler, StringBindings};
use std::collections::HashMap;

/// Is used to detect the signal edge from low to high or high to low on user input.
/// Plainly stated: helps detect when the user first presses a key and when they let go of a key.
#[derive(Debug, Default)]
pub struct SignalEdgeDetector {
    map: HashMap<String, bool>,
}

pub enum SignalEdge {
    /// No signal edge. The signal was low the previous time you checked and it is still low now.
    /// For example: the key is in its idle state and is NOT being pressed right now.
    StillLow,
    /// No signal edge. The signal was high the previous time you checked and it is still high now.
    /// For example: the user is holding down the key.
    StillHigh,
    /// Signal rose. It was previously down (no input) and is now up.
    /// For example: the user has started pressing a key.
    Rising,
    /// Signal fell. It was previously up and is now down.
    /// For example: the user has just let go of the key.
    Falling,
}

impl SignalEdgeDetector {
    pub fn new() -> Self {
        SignalEdgeDetector::default()
    }

    /// Call this at most once per frame for each action_key.
    pub fn edge(&mut self, action_key: &str, handler: &InputHandler<StringBindings>) -> SignalEdge {
        if !self.map.contains_key(action_key) {
            self.map.insert(action_key.to_string(), false);
        }
        let old_signal = *self.map.get_mut(action_key).expect("Should not panic.");
        let current_signal = handler.action_is_down(action_key).unwrap_or(false);
        *self.map.get_mut(action_key).expect("Should not panic.") = current_signal;

        if old_signal && !current_signal {
            // println!("detected falling signal for {:?}", action_key);
            SignalEdge::Falling
        } else if !old_signal && current_signal {
            // println!("detected rising signal for {:?}", action_key);
            SignalEdge::Rising
        } else if old_signal {
            // println!("detected high signal for {:?}", action_key);
            SignalEdge::StillHigh
        } else {
            // println!("detected low signal for {:?}", action_key);
            SignalEdge::StillLow
        }
    }
}
