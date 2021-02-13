use crate::components::*;
use crate::resources::*;
use crate::systems::SoundEvent;
use amethyst::core::ecs::shrev::EventChannel;
use amethyst::core::{Time, Transform};
use amethyst::ecs::prelude::{Join, Read, ReadStorage, System, Write, WriteStorage};

#[derive(Default)]
pub struct SteeringSystem;

impl<'s> System<'s> for SteeringSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Write<'s, EventChannel<SoundEvent>>,
        WriteStorage<'s, SteeringIntent>,
        ReadStorage<'s, Transform>,
        WriteStorage<'s, Steering>,
        Read<'s, TileMap>,
        Write<'s, History>,
        Read<'s, Time>,
    );

    fn run(
        &mut self,
        (
            mut sound_channel,
            mut steering_intents,
            transforms,
            mut steerings,
            tile_map,
            mut history,
            time,
        ): Self::SystemData,
    ) {
        for (intent, transform, steering) in
            (&mut steering_intents, &transforms, &mut steerings).join()
        {
            let old_pos = steering.pos;
            let (anchored_x, anchored_y) = steering.to_anchor_coords(transform);
            steering.pos = Pos::new(anchored_x.round() as i32, anchored_y.round() as i32);

            if steering.is_mid_air() {
                steering.mode = steering.mode.add_to_duration(time.fixed_seconds());
            }

            if steering.is_grounded() && !intent.face.is_neutral() {
                steering.facing.x = intent.face;
            }

            // The following if-else construction checks if the steering mode should be changed.
            let has_ground_beneath_feet = is_grounded(&steering, &tile_map);
            if steering.is_falling()
                && anchored_y <= steering.pos.y as f32
                && has_ground_beneath_feet
                && on_solid_ground(steering, &tile_map)
            {
                // If falling and you reached the floor, set to grounded.
                steering.mode = SteeringMode::Grounded;
                steering.destination = steering.pos;
            } else if (steering.is_grounded()
                && !has_ground_beneath_feet
                && aligned_with_grid(steering.destination.x as f32, anchored_x, intent.walk))
                || (steering.is_climbing() && intent.jump)
            {
                steering.mode = SteeringMode::Falling {
                    x_movement: Direction1D::Neutral,
                    starting_y_pos: transform.translation().y,
                    duration: 0.,
                };
            } else if steering.is_grounded() && intent.jump {
                if is_underneath_ceiling(steering, &tile_map) {
                    sound_channel.single_write(SoundEvent::new(SoundType::CannotPerformAction));
                } else {
                    sound_channel.single_write(SoundEvent::new(SoundType::Jump));
                    steering.mode = SteeringMode::Jumping {
                        x_movement: intent.face,
                        starting_y_pos: transform.translation().y,
                        duration: 0.,
                    };
                }
            } else if steering.jump_has_peaked() {
                steering.mode = steering.mode.jump_to_fall();
            } else if steering.is_grounded()
                && aligned_with_grid(steering.destination.x as f32, anchored_x, intent.walk)
                && ((intent.climb.is_positive() && can_climb_up(steering, &tile_map))
                    || (intent.climb.is_negative() && can_climb_down(steering, &tile_map)))
            {
                steering.mode = SteeringMode::Climbing;
                if !intent.walk.is_neutral() {
                    intent.walk_invalidated = true;
                }
            } else if steering.is_climbing()
                && aligned_with_grid(steering.destination.y as f32, anchored_y, intent.climb)
                && !intent.walk_invalidated
                && ((intent.walk.is_positive()
                    && !is_against_wall_right(&steering, steering.pos.y as f32, &tile_map))
                    || (intent.walk.is_negative()
                        && !is_against_wall_left(&steering, steering.pos.y as f32, &tile_map)))
            {
                steering.mode = SteeringMode::Grounded;
            }

            // This match will adjust the steering based on the current steering mode.
            match steering.mode {
                SteeringMode::Grounded => {
                    if !intent.walk.is_neutral() {
                        steering.facing = Direction2D::from(intent.walk, Direction1D::Neutral);
                        let offset_from_destination = steering.destination.x as f32 - anchored_x;
                        if offset_from_destination < f32::EPSILON && intent.walk.is_positive() {
                            if !is_against_wall_right(&steering, steering.pos.y as f32, &tile_map) {
                                steering.destination.x = steering.pos.x + 1;
                                sound_channel.single_write(SoundEvent::new(SoundType::Step));
                            }
                        } else if offset_from_destination > -f32::EPSILON
                            && intent.walk.is_negative()
                        {
                            if !is_against_wall_left(&steering, steering.pos.y as f32, &tile_map) {
                                steering.destination.x = steering.pos.x - 1;
                                sound_channel.single_write(SoundEvent::new(SoundType::Step));
                            }
                        } else if !intent
                            .walk
                            .aligns_with((steering.destination.x - steering.pos.x) as f32)
                        {
                            // TODO: Maybe remove, this doesn't seem to do anything.
                            // Player wants to go back where they came from.
                            steering.destination.x = steering.pos.x;
                        }
                    }
                }
                SteeringMode::Climbing => {
                    if !intent.climb.is_neutral() {
                        steering.facing = Direction2D::from(Direction1D::Neutral, intent.climb);
                        let offset_from_discrete_pos = steering.destination.y as f32 - anchored_y;
                        if offset_from_discrete_pos < f32::EPSILON && intent.climb.is_positive() {
                            if can_climb_up(steering, &tile_map) {
                                sound_channel.single_write(SoundEvent::new(SoundType::LadderStep));
                                steering.destination.y = steering.pos.y + 1;
                            } else {
                                steering.mode = SteeringMode::Grounded;
                            }
                        } else if offset_from_discrete_pos > -f32::EPSILON
                            && intent.climb.is_negative()
                        {
                            if can_climb_down(steering, &tile_map) {
                                sound_channel.single_write(SoundEvent::new(SoundType::LadderStep));
                                steering.destination.y = steering.pos.y - 1;
                            } else if above_air(steering, &tile_map) {
                                steering.mode = SteeringMode::Falling {
                                    x_movement: Direction1D::Neutral,
                                    starting_y_pos: transform.translation().y,
                                    duration: 0.,
                                };
                            } else {
                                steering.mode = SteeringMode::Grounded;
                            }
                        } else if !intent
                            .climb
                            .aligns_with((steering.destination.y - steering.pos.y) as f32)
                        {
                            // TODO: Maybe remove, this doesn't seem to do anything.
                            // Player wants to go back where they came from.
                            steering.destination.y = steering.pos.y;
                        }
                    }
                }
                SteeringMode::Falling {
                    x_movement,
                    starting_y_pos,
                    duration,
                } => {
                    if x_movement.is_neutral() {
                        // No horizontal movement.
                        steering.destination.x = steering.pos.x;
                    } else if x_movement.is_positive() {
                        // Moving towards the right.
                        if is_against_wall_right(steering, anchored_y, &tile_map) {
                            steering.mode = SteeringMode::Falling {
                                x_movement: Direction1D::Neutral,
                                starting_y_pos,
                                duration,
                            };
                        } else if aligned_with_grid(
                            steering.destination.x as f32,
                            anchored_x,
                            x_movement,
                        ) {
                            steering.destination.x = steering.pos.x + 1;
                        }
                    } else {
                        // Moving towards the left.
                        if is_against_wall_left(steering, anchored_y, &tile_map) {
                            steering.mode = SteeringMode::Falling {
                                x_movement: Direction1D::Neutral,
                                starting_y_pos,
                                duration,
                            };
                        } else if aligned_with_grid(
                            steering.destination.x as f32,
                            anchored_x,
                            x_movement,
                        ) {
                            steering.destination.x = steering.pos.x - 1;
                        }
                    }
                }
                SteeringMode::Jumping {
                    x_movement,
                    starting_y_pos,
                    duration,
                } => {
                    if !intent.jump_direction.is_neutral() {
                        steering.mode = SteeringMode::Jumping {
                            x_movement: intent.jump_direction,
                            starting_y_pos,
                            duration,
                        };
                        steering.facing =
                            Direction2D::from(intent.jump_direction, Direction1D::Neutral);
                    }
                    if x_movement.is_neutral() {
                        // No horizontal movement.
                        steering.destination.x = steering.pos.x;
                    } else if x_movement.is_positive() {
                        // Moving towards the right.
                        if aligned_with_grid(steering.destination.x as f32, anchored_x, x_movement)
                            && !is_against_wall_right(steering, steering.pos.y as f32, &tile_map)
                        {
                            steering.destination.x = steering.pos.x + 1;
                        }
                    } else {
                        // Moving towards the left.
                        if aligned_with_grid(steering.destination.x as f32, anchored_x, x_movement)
                            && !is_against_wall_left(steering, steering.pos.y as f32, &tile_map)
                        {
                            steering.destination.x = steering.pos.x - 1;
                        }
                    }
                }
            };

            // Push frame on history if player position changed.
            if old_pos != steering.pos || history.force_key_frame {
                history.push_frame(Frame::new(steering.pos));
            }
        }
    }
}

/// Returns true iff the player is aligned with the grid.
/// This function can be used for both horizontal and vertical coordinates.
fn aligned_with_grid(destination_pos: f32, actual_pos: f32, input: Direction1D) -> bool {
    let offset = actual_pos - destination_pos;
    // Actual pos equal or greater than destination. Moving towards the positive.
    (offset > -f32::EPSILON && input.is_positive())
        // Actual pos equal or smaller than destination. Moving towards the negative.
        || (offset < f32::EPSILON && input.is_negative())
        // Actual pos basically equal to the destination.
        || (offset.abs() < f32::EPSILON)
}

/// You cannot jump onto the middle of a ladder, so use this function to check if you
/// should set steering to Grounded.
/// Returns true iff entity is on solid ground; meaning the very top of a ladder or a proper,
/// solid block that is not climbable.
///
/// This definition excludes the middle of a ladder. While the middle of a ladder can be walked on,
/// it cannot be landed on from a jump or fall.
fn on_solid_ground(steering: &Steering, tile_map: &TileMap) -> bool {
    (0..steering.dimens.x).any(|i| {
        let tile = tile_map.get_tile(&Pos::new(steering.pos.x + i, steering.pos.y - 1));
        let tile_above = tile_map.get_tile(&Pos::new(steering.pos.x + i, steering.pos.y));
        tile.map(|tile| {
            tile.provides_platform()
                && (!tile.climbable
                    || !tile_above
                        .map(|tile_above| tile_above.climbable)
                        .unwrap_or(false))
        })
        .unwrap_or(false)
    })
}

fn is_grounded(steering: &Steering, tile_map: &TileMap) -> bool {
    (0..steering.dimens.x).any(|i| {
        let tile = tile_map.get_tile(&Pos::new(steering.pos.x + i, steering.pos.y - 1));
        tile.map(|tile| tile.provides_platform()).unwrap_or(false)
    })
}

/// The player cannot jump when underneath a 2-high ceiling.
/// This function returns true iff the player is underneath a 2-high ceiling.
fn is_underneath_ceiling(steering: &Steering, tile_map: &TileMap) -> bool {
    (0..steering.dimens.x).any(|i| {
        let tile = tile_map.get_tile(&Pos::new(
            steering.pos.x + i,
            steering.pos.y + steering.dimens.y,
        ));
        tile.map(|tile| tile.collides_bottom()).unwrap_or(false)
    })
}

fn is_against_wall_left(steering: &Steering, anchored_y: f32, tile_map: &TileMap) -> bool {
    is_against_wall(&steering, anchored_y, &tile_map, -1, 0)
}

fn is_against_wall_right(steering: &Steering, anchored_y: f32, tile_map: &TileMap) -> bool {
    is_against_wall(
        &steering,
        anchored_y,
        &tile_map,
        steering.dimens.x,
        steering.dimens.x - 1,
    )
}

fn is_against_wall(
    steering: &Steering,
    anchored_y: f32,
    tile_map: &TileMap,
    x_offset: i32,
    x_offset_for_tile_in_front: i32,
) -> bool {
    let floored_y = anchored_y.floor();
    let nr_blocks_to_check = if (floored_y - anchored_y).abs() > f32::EPSILON {
        steering.dimens.y + 1
    } else {
        steering.dimens.y
    };
    (0..nr_blocks_to_check).any(|i| {
        let tile = tile_map.get_tile(&Pos::new(steering.pos.x + x_offset, floored_y as i32 + i));
        let tile_in_front = tile_map.get_tile(&Pos::new(
            steering.pos.x + x_offset_for_tile_in_front,
            floored_y as i32 + i,
        ));
        tile.map(|tile| {
            tile.collides_horizontally()
                && tile_in_front
                    .map(|tile_in_front| !tile_in_front.collides_horizontally())
                    .unwrap_or(true)
        })
        .unwrap_or(false)
    })
}

fn can_climb_up(steering: &Steering, tile_map: &TileMap) -> bool {
    can_climb(steering, tile_map, (0, 1)) && !is_underneath_ceiling(steering, &tile_map)
}

fn can_climb_down(steering: &Steering, tile_map: &TileMap) -> bool {
    can_climb(steering, tile_map, (-1, 0))
}

fn can_climb(steering: &Steering, tile_map: &TileMap, y_range: (i32, i32)) -> bool {
    (0..steering.dimens.x).all(|x_offset| {
        (y_range.0..y_range.1).all(|y_offset| {
            let tile = tile_map.get_tile(&Pos::new(
                steering.pos.x + x_offset,
                steering.pos.y + y_offset,
            ));
            tile.map(|tile| tile.climbable).unwrap_or(false)
        })
    })
}

fn above_air(steering: &Steering, tile_map: &TileMap) -> bool {
    (0..steering.dimens.x).all(|x_offset| {
        let tile = tile_map.get_tile(&Pos::new(steering.pos.x + x_offset, steering.pos.y - 1));
        tile.map(|tile| !tile.provides_platform()).unwrap_or(true)
    })
}
