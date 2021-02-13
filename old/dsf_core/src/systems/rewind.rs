use crate::components::*;
use crate::resources::*;
use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    ecs::prelude::{Join, Read, ReadStorage, System, Write, WriteStorage},
    input::{InputHandler, StringBindings},
};

pub struct RewindControlSystem;

impl<'s> System<'s> for RewindControlSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Write<'s, CurrentState>,
        Write<'s, Rewind>,
        Write<'s, History>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Time>,
        Read<'s, DebugSettings>,
    );

    fn run(
        &mut self,
        (mut current_state, mut rewind, mut history, input, time, config): Self::SystemData,
    ) {
        history.force_key_frame = false;
        if input.action_is_down("shift").unwrap_or(false) {
            rewind.cooldown = match *current_state {
                CurrentState::Running => config.seconds_per_rewind_frame,
                CurrentState::Rewinding => {
                    if rewind.is_ready() {
                        rewind.cooldown + config.seconds_per_rewind_frame
                    } else {
                        rewind.cooldown - time.fixed_seconds()
                    }
                }
            };
            *current_state = CurrentState::Rewinding;
        } else {
            if CurrentState::Rewinding == *current_state {
                history.force_key_frame = true;
            }
            *current_state = CurrentState::Running;
        }
    }
}

pub struct RewindSystem;

impl<'s> System<'s> for RewindSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Steering>,
        ReadStorage<'s, Player>,
        Read<'s, Rewind>,
        Write<'s, History>,
    );

    fn run(
        &mut self,
        (mut transforms, mut steerings, player_tags, rewind, mut history): Self::SystemData,
    ) {
        if rewind.is_ready() {
            if let Some(frame) = history.pop_frame() {
                info!("Rewinding player to {:?}", frame);
                for (_, transform, steering) in
                    (&player_tags, &mut transforms, &mut steerings).join()
                {
                    let (centered_x, centered_y) =
                        steering.to_centered_coords(frame.player_position);
                    transform.set_translation_x(centered_x);
                    transform.set_translation_y(centered_y);
                    steering.pos = frame.player_position;
                    steering.destination = frame.player_position;
                }
            }
        }
    }
}
