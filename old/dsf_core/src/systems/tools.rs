use amethyst::core::ecs::{Entities, LazyUpdate};
use amethyst::{
    core::math::Vector2,
    core::transform::Transform,
    ecs::prelude::{Join, Read, ReadStorage, System, Write, WriteStorage},
    input::{InputHandler, StringBindings},
};

use crate::components::*;
use crate::levels::*;
use crate::resources::*;
use crate::systems::SoundEvent;
use amethyst::core::ecs::shrev::EventChannel;
use amethyst::core::Parent;
use amethyst::prelude::{Builder, WorldExt};

/// Tool width and height, hardcoded for now.
/// TODO: Don't hardcode.
const TOOL_WIDTH: f32 = 2.;
const TOOL_HEIGHT: f32 = 2.;

/// Checks if the player intersects any tools.
/// If so, the tool will equipped by the player and will be removed from the game.
#[derive(Default)]
pub struct PickupSystem;

impl<'s> System<'s> for PickupSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Write<'s, EventChannel<SoundEvent>>,
        WriteStorage<'s, Player>,
        ReadStorage<'s, Steering>,
        ReadStorage<'s, Tool>,
        ReadStorage<'s, Transform>,
        Read<'s, LazyUpdate>,
        Entities<'s>,
    );

    fn run(
        &mut self,
        (mut sound_channel, mut players, steerings, tools, transforms, lazy, entities): Self::SystemData,
    ) {
        let player = (&mut players, &entities, &steerings, &transforms)
            .join()
            .map(|(player, entity, steering, transform)| {
                (
                    player,
                    entity,
                    Vector2::new(transform.translation().x, transform.translation().y),
                    Vector2::new(steering.dimens.x as f32, steering.dimens.y as f32),
                )
            })
            .next();
        if let Some((mut player, player_entity, pos, dimens)) = player {
            if player.equipped.is_some() {
                return;
            }
            // Find the first tool that intersects with the player:
            let tool_opt = (&tools, &transforms, &entities)
                .join()
                .find(|(_, transform, _)| {
                    let key_x = transform.translation().x;
                    let key_y = transform.translation().y;
                    pos.x - dimens.x / 2. < key_x + TOOL_WIDTH / 3.
                        && pos.x + dimens.x / 2. > key_x - TOOL_WIDTH / 3.
                        && pos.y - dimens.y / 2. < key_y + TOOL_HEIGHT / 3.
                        && pos.y + dimens.y / 2. > key_y - TOOL_HEIGHT / 3.
                });
            if let Some((tool, _, tool_entity)) = tool_opt {
                sound_channel.single_write(SoundEvent::new(SoundType::ToolPickup));
                player.equipped = Some(tool.tool_type);
                let (sprite, sprite_nr) = (tool.sprite, tool.sprite_nr);
                lazy.exec_mut(move |world| {
                    world
                        .delete_entity(tool_entity)
                        .expect("Tried to delete tool, but failed.");
                    let render = load_asset_from_world(&sprite, sprite_nr, world);
                    world
                        .create_entity()
                        .with(EquippedTag)
                        .with(Transform::default())
                        .with(Parent {
                            entity: player_entity,
                        })
                        .with(render)
                        .build();
                });
            }
        }
    }
}

#[derive(Default)]
pub struct UseToolSystem;

impl<'s> System<'s> for UseToolSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Write<'s, EventChannel<SoundEvent>>,
        WriteStorage<'s, Player>,
        ReadStorage<'s, Steering>,
        ReadStorage<'s, EquippedTag>,
        ReadStorage<'s, Block>,
        Read<'s, InputHandler<StringBindings>>,
        Write<'s, TileMap>,
        Entities<'s>,
    );

    fn run(
        &mut self,
        (
            mut sound_channel,
            mut players,
            steerings,
            equipped_tags,
            blocks,
            input,
            mut tile_map,
            entities,
        ): Self::SystemData,
    ) {
        let wants_to_use_tool = input.action_is_down("jump").unwrap_or(false);
        if !wants_to_use_tool {
            return;
        }
        for (player, steering) in (&mut players, &steerings).join() {
            if !steering.is_grounded() {
                return;
            }
            let targeted_blocks = match player.equipped {
                Some(ToolType::BreakBlocksHorizontally(depth)) => {
                    let player_is_not_too_far_away_from_wall =
                        at_least_one_is_breakable(&tiles_to_side(1, steering), &tile_map);
                    if player_is_not_too_far_away_from_wall {
                        Some(tiles_to_side(depth, steering))
                    } else {
                        None
                    }
                }
                Some(ToolType::BreakBlocksBelow(depth)) => Some(tiles_below(depth, steering)),
                _ => None,
            };
            if let Some(targeted_blocks) = targeted_blocks {
                let at_least_one_is_breakable =
                    at_least_one_is_breakable(&targeted_blocks, &tile_map);
                let none_are_unbreakable = none_are_unbreakable(&targeted_blocks, &tile_map);
                if at_least_one_is_breakable && none_are_unbreakable {
                    sound_channel.single_write(SoundEvent::new(SoundType::Mining));
                    player.equipped = None;
                    targeted_blocks.iter().for_each(|pos| {
                        tile_map.remove_tile(pos);
                    });
                    for (_, entity) in (&equipped_tags, &entities).join() {
                        entities
                            .delete(entity)
                            .expect("Failed to delete equipped tool sprite.");
                    }
                    for (block, entity) in (&blocks, &entities).join() {
                        if targeted_blocks.contains(&block.pos) {
                            entities.delete(entity).expect("Failed to delete block!");
                        }
                    }
                }
            }
        }
    }
}

fn at_least_one_is_breakable(blocks: &[Pos], tile_map: &TileMap) -> bool {
    blocks.iter().any(|pos| {
        tile_map
            .get_tile(pos)
            .map(|block| block.is_breakable())
            .unwrap_or(false)
    })
}

fn none_are_unbreakable(blocks: &[Pos], tile_map: &TileMap) -> bool {
    blocks.iter().all(|pos| {
        tile_map
            .get_tile(pos)
            .map(|block| block.is_breakable())
            .unwrap_or(true)
    })
}

fn tiles_to_side(depth: u8, steering: &Steering) -> Vec<Pos> {
    let facing_offset = if steering.facing.x.is_positive() {
        steering.dimens.x
    } else {
        -1
    };
    (0..(depth as i32))
        .flat_map(|x| {
            (0..steering.dimens.y).map(move |y| (x, y)) //???
        })
        .map(|(x_offset, y_offset)| {
            Pos::new(
                steering.pos.x + facing_offset + x_offset * steering.facing.x.signum_i(),
                steering.pos.y + y_offset,
            )
        })
        .collect()
}

fn tiles_below(depth: u8, steering: &Steering) -> Vec<Pos> {
    let facing_offset = if steering.facing.x.is_positive() {
        steering.dimens.x - 1
    } else {
        0
    };
    (0..steering.dimens.x)
        .flat_map(|x| (0..(depth as i32)).map(move |y| (x, y)))
        .map(|(x_offset, y_offset)| {
            Pos::new(
                steering.pos.x + facing_offset + x_offset * steering.facing.x.signum_i(),
                steering.pos.y - 1 - y_offset,
            )
        })
        .collect()
}
