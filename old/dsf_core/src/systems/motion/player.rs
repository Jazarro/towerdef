use crate::components::{Direction1D, Player, Steering, SteeringIntent};
use crate::resources::MovementConfig;
use amethyst::core::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::core::Time;
use amethyst::input::{InputHandler, StringBindings};

/// Sets the player intention to move.
#[derive(Default)]
pub struct PlayerSystem;

impl<'s> System<'s> for PlayerSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        WriteStorage<'s, Player>,
        ReadStorage<'s, Steering>,
        WriteStorage<'s, SteeringIntent>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, MovementConfig>,
        Read<'s, Time>,
    );

    fn run(
        &mut self,
        (mut players, steerings, mut steering_intents, input, config, time): Self::SystemData,
    ) {
        let input_x = input.axis_value("move_x").unwrap_or(0.0);
        let input_y = input.axis_value("move_y").unwrap_or(0.0);
        let jump_down = input.action_is_down("jump").unwrap_or(false);
        for (player, intent, steering) in (&mut players, &mut steering_intents, &steerings).join() {
            let initiate_jump = jump_down && !player.pressing_jump;
            player.pressing_jump = jump_down;
            player.jump_grace_timer = if initiate_jump {
                Some(0.)
            } else if let Some(time_passed) = player.jump_grace_timer {
                let time_passed = time_passed + time.fixed_seconds();
                if time_passed < config.jump_allowance {
                    Some(time_passed)
                } else {
                    None
                }
            } else {
                None
            };
            let old_walk = intent.walk;
            let new_walk = Direction1D::new(input_x);
            let turn_around = steering.is_grounded()
                && steering.facing.x.is_opposite(&new_walk)
                && old_walk.is_neutral();
            player.turn_around_timer = if turn_around {
                // Player wants to turn around, initialise turn-around timer.
                Some(0.)
            } else if new_walk.is_neutral() {
                // Player has let go of controls, forcefully reset timer.
                None
            } else if let Some(time_passed) = player.turn_around_timer {
                let time_passed = time_passed + time.fixed_seconds();
                if time_passed < config.turn_allowance {
                    Some(time_passed)
                } else {
                    None
                }
            } else {
                None
            };

            if player.turn_around_timer.is_none() {
                intent.walk = new_walk;
            }
            intent.face = new_walk;
            if intent.walk_invalidated && old_walk != intent.walk {
                intent.walk_invalidated = false;
            }
            intent.climb = Direction1D::new(input_y);
            intent.jump = player.equipped.is_none() && initiate_jump;
            intent.jump_direction = if player.jump_grace_timer.is_some() {
                intent.walk
            } else {
                Direction1D::Neutral
            };
        }
    }
}
