use amethyst::core::ecs::HashMapStorage;
use amethyst::{
    assets::PrefabData,
    core::transform::Transform,
    derive::PrefabData,
    ecs::{prelude::Entity, Component, DenseVecStorage, WriteStorage},
    error::Error,
};
use serde::{Deserialize, Serialize};
use std::ops::{Add, Sub};

#[derive(Clone, Copy, Component, Debug, Default, Deserialize, Serialize, PrefabData, PartialEq)]
#[prefab(Component)]
#[serde(deny_unknown_fields)]
pub struct Direction2D {
    pub x: Direction1D,
    pub y: Direction1D,
}

impl Direction2D {
    pub fn new(signum_x: f32, signum_y: f32) -> Self {
        Direction2D {
            x: Direction1D::new(signum_x),
            y: Direction1D::new(signum_y),
        }
    }
    pub fn from(x: Direction1D, y: Direction1D) -> Self {
        Direction2D { x, y }
    }

    pub fn is_opposite(&self, other: &Direction2D) -> bool {
        self.x.is_opposite(&other.x) || self.y.is_opposite(&other.y)
    }

    pub fn is_neutral(&self) -> bool {
        self.x == Direction1D::Neutral && self.y == Direction1D::Neutral
    }
}

#[derive(Clone, Copy, Component, Debug, Deserialize, Serialize, PrefabData, PartialEq)]
#[prefab(Component)]
#[serde(deny_unknown_fields)]
pub enum Direction1D {
    Negative,
    Positive,
    Neutral,
}

impl Direction1D {
    pub fn new(signum: f32) -> Self {
        if signum.abs() <= f32::EPSILON {
            Direction1D::Neutral
        } else if signum.is_sign_positive() {
            Direction1D::Positive
        } else {
            Direction1D::Negative
        }
    }
    pub fn is_opposite(&self, other: &Direction1D) -> bool {
        (*self == Direction1D::Negative && *other == Direction1D::Positive)
            || (*self == Direction1D::Positive && *other == Direction1D::Negative)
    }
    pub fn is_positive(&self) -> bool {
        self == &Direction1D::Positive
    }
    pub fn is_negative(&self) -> bool {
        self == &Direction1D::Negative
    }
    pub fn is_neutral(&self) -> bool {
        self == &Direction1D::Neutral
    }

    pub fn aligns_with(&self, direction: f32) -> bool {
        let other = Direction1D::new(direction);
        self != &Direction1D::Neutral && self == &other
    }
    pub fn signum(&self) -> f32 {
        match self {
            Direction1D::Positive => 1.,
            Direction1D::Negative => -1.,
            Direction1D::Neutral => 0.,
        }
    }
    pub fn signum_i(&self) -> i32 {
        match self {
            Direction1D::Positive => 1,
            Direction1D::Negative => -1,
            Direction1D::Neutral => 0,
        }
    }
}

impl Default for Direction1D {
    fn default() -> Self {
        Direction1D::Neutral
    }
}

/// Velocity in meters per second.
#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

impl Component for Velocity {
    type Storage = HashMapStorage<Self>;
}

/// A discrete position in the world, with x and y being integral numbers.
/// Used among other things for positioning tiles, which are always snapped to the grid.
///
/// Not to be confused with Transform, which contains an entity's actual position.
#[derive(
    Clone, Copy, Debug, Default, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[serde(deny_unknown_fields)]
pub struct Pos {
    // TODO: consider whether to replace Pos with with nalgebra's Point2?
    pub x: i32,
    pub y: i32,
}

impl Pos {
    pub fn new(x: i32, y: i32) -> Self {
        Pos { x, y }
    }

    pub fn append_x(&self, x: i32) -> Self {
        Pos::new(self.x + x, self.y)
    }

    pub fn append_y(&self, y: i32) -> Self {
        Pos::new(self.x, self.y + y)
    }

    pub fn append_xy(&self, x: i32, y: i32) -> Self {
        Pos::new(self.x + x, self.y + y)
    }
}

impl Sub for Pos {
    type Output = Pos;

    fn sub(self, other: Pos) -> Pos {
        Pos::new(self.x - other.x, self.y - other.y)
    }
}

impl Add for Pos {
    type Output = Pos;

    fn add(self, other: Pos) -> Pos {
        Pos::new(self.x + other.x, self.y + other.y)
    }
}

/// Remembers the direction an entity is moving in. Also keeps a destination as a discrete position.
/// Steering is used to accomplish the snap-to-grid, tile-based movement.
///
/// Any non-particle entity that has movement should have steering.
/// Examples of entities with steering include the Player, enemies and projectiles.
#[derive(Clone, Debug, Default, Deserialize, Serialize, PrefabData)]
#[prefab(Component)]
#[serde(deny_unknown_fields)]
pub struct Steering {
    /// The entity's discrete position. Not to be confused with its Transform, which is where the
    /// entity is actually at. Transform can be between squares, the discrete position is always at
    /// a square. The discrete position has it's coordinate in integral numbers, whereas the
    /// Transform's translation is in floats.
    ///
    /// If an entity is wider than 1 by 1, the pos is the bottom-left most tile in the entity's
    /// body.
    pub pos: Pos,
    /// Width and height of the entity.
    pub dimens: Pos,
    /// Direction the entity is facing along the x-axis and y-axis.
    pub facing: Direction2D,
    pub destination: Pos,
    pub mode: SteeringMode,
}

impl Component for Steering {
    type Storage = HashMapStorage<Self>;
}

/// Specifies how the entity intents to move. For the player, this is mostly informed by the
/// keyboard input. For enemies, this will be set by the AI. For all entities with Steering,
/// the SteeringSystem then actually moves the entity based on this intent.
#[derive(Clone, Debug, Default, Deserialize, Serialize, PrefabData)]
#[prefab(Component)]
#[serde(deny_unknown_fields)]
pub struct SteeringIntent {
    /// If a player is still holding a horizontal movement key (for instance; RIGHT) when they
    /// start climbing, they might move off the ladder after climbing 1 tile. To fix this,
    /// this flag will be set to true when the player starts climbing. To start moving horizontally
    /// at this point, they must let go of the movement key and press RIGHT or LEFT again.
    ///
    /// If they reach the end of the ladder and can climb no further, if they're still holding down
    /// the (invalidated) RIGHT or LEFT button they will start moving horizontally regardless of
    /// this flag.
    ///
    /// This feature exists solely for players, to make movement feel better.
    pub walk_invalidated: bool,
    /// The entity wishes to face this direction. Only for player characters. Note that entities
    /// will still face the direction they are walking in, this value is just for when you
    /// want to face a certain direction without walking.
    pub face: Direction1D,
    /// The entity wishes to walk along the floor in this direction.
    pub walk: Direction1D,
    /// The entity wishes to climb on a ladder in this direction.
    pub climb: Direction1D,
    /// If true; the entity wishes to jump.
    pub jump: bool,
    /// The entity wishes to jump in this direction. This is separate from walk because it is
    /// possible to specify a direction for a limited time after the jump has already started.
    /// That feature exists solely for players, to make movement feel better.
    pub jump_direction: Direction1D,
}

impl Component for SteeringIntent {
    type Storage = HashMapStorage<Self>;
}

/// SteeringMode influences max speeds, ability to jump, ability to move, etc.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum SteeringMode {
    /// Standard mode. There is flat ground beneath the entity and the entity can either move
    /// horizontally or initiate a jump.
    Grounded,
    /// Climbing on a ladder. The entity can either climb up or down.
    Climbing,
    /// The entity is falling straight down.
    Falling {
        /// The x-movement that the entity has while falling. This will remain constant.
        /// It is either a -1 (move to left) 0 (don't move along x-axis) or 1 (move right).
        x_movement: Direction1D,
        /// The y-coordinate that the entity had when it  first started falling.
        starting_y_pos: f32,
        /// The time in seconds since the entity started falling.
        duration: f32,
    },
    /// The entity is jumping. The character may have an x-velocity.
    /// While jumping, the character's y-coordinate describes a certain curve.
    /// This also takes the original y-coordinate and the start time.
    /// These are necessary to be able to interpolate the y-coordinate.
    Jumping {
        /// The x-movement that the entity has while jumping. This will remain constant.
        /// It is either a -1 (move to left) 0 (don't move along x-axis) or 1 (move right).
        x_movement: Direction1D,
        /// The y-coordinate that the entity had when it started the jump.
        starting_y_pos: f32,
        /// The time in seconds since the character started their jump.
        duration: f32,
    },
}

impl Default for SteeringMode {
    fn default() -> Self {
        SteeringMode::Grounded
    }
}

impl SteeringMode {
    /// Calculate the y offset from the initial y-position at the time this movement began.
    /// This method is only valid for SteeringMode::Falling and SteeringMode::Jumping. It will
    /// return 0. otherwise.
    pub fn calc_delta_y(&self, duration: f32) -> f32 {
        match self {
            SteeringMode::Jumping { .. } => -50. * (duration - 0.209).powf(2.) + 2.2,
            SteeringMode::Falling { .. } => duration * -15.,
            _ => 0.,
        }
    }

    pub fn jump_to_fall(&self) -> Self {
        if let SteeringMode::Jumping {
            x_movement,
            starting_y_pos,
            duration,
        } = *self
        {
            SteeringMode::Falling {
                x_movement,
                starting_y_pos: starting_y_pos + self.calc_delta_y(0.209),
                duration: duration - 0.209,
            }
        } else {
            panic!("Not allowed.");
        }
    }

    pub fn add_to_duration(&self, delta_time: f32) -> Self {
        match *self {
            SteeringMode::Jumping {
                x_movement,
                starting_y_pos,
                duration,
            } => SteeringMode::Jumping {
                x_movement,
                starting_y_pos,
                duration: duration + delta_time,
            },
            SteeringMode::Falling {
                x_movement,
                starting_y_pos,
                duration,
            } => SteeringMode::Falling {
                x_movement,
                starting_y_pos,
                duration: duration + delta_time,
            },
            _ => panic!("Not allowed to call this on SteeringMode that is not Falling or Jumping."),
        }
    }
}

impl Steering {
    pub fn new(pos: Pos, dimens: Pos) -> Steering {
        Steering {
            pos,
            dimens,
            facing: Direction2D::new(1., 0.),
            destination: pos,
            mode: SteeringMode::Grounded,
        }
    }

    pub fn is_grounded(&self) -> bool {
        self.mode == SteeringMode::Grounded
    }

    pub fn is_mid_air(&self) -> bool {
        match self.mode {
            SteeringMode::Falling { .. } => true,
            SteeringMode::Jumping { .. } => true,
            _ => false,
        }
    }

    pub fn is_jumping(&self) -> bool {
        matches!(self.mode, SteeringMode::Jumping {..})
    }

    pub fn jump_has_peaked(&self) -> bool {
        if let SteeringMode::Jumping { duration, .. } = self.mode {
            duration > 0.209
        } else {
            false
        }
    }

    pub fn is_falling(&self) -> bool {
        matches!(self.mode, SteeringMode::Falling {..})
    }

    pub fn is_climbing(&self) -> bool {
        self.mode == SteeringMode::Climbing
    }

    /// Converts the given discrete position to a translation, taking into account the dimensions
    /// of the entity.
    ///
    /// The discrete position is the bottom-left corner of the entity, a translation is the
    /// center point of the entity.
    pub fn to_centered_coords(&self, pos: Pos) -> (f32, f32) {
        (
            pos.x as f32 + 0.5 * self.dimens.x as f32,
            pos.y as f32 + 0.5 * self.dimens.y as f32,
        )
    }

    /// Converts the given translation, which is the center-point of the entity, into a pair of
    /// anchored coordinates, describing the bottom-left corner of the entity.
    ///
    /// Note that this does NOT return a discrete position: output is not rounded or floored.
    pub fn to_anchor_coords(&self, transform: &Transform) -> (f32, f32) {
        (
            transform.translation().x - 0.5 * self.dimens.x as f32,
            transform.translation().y - 0.5 * self.dimens.y as f32,
        )
    }
}
