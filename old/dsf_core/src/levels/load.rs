use amethyst::prelude::WorldExt;

use amethyst::{
    assets::{Handle, Prefab},
    config::ConfigError,
    core::{math::Vector3, transform::Transform, Parent},
    ecs::{prelude::World, Entities, Entity, EntityBuilder, Join, ReadStorage},
    prelude::*,
    renderer::{sprite::SpriteRender, Transparent},
};
use dsf_precompile::MyPrefabData;

use crate::components::*;

use crate::levels::LevelSave;
use crate::resources::*;

use crate::utility::files::get_world_dir;
use amethyst::renderer::resources::Tint;
use std::path::PathBuf;

pub fn load_tile_definitions() -> Result<TileDefinitions, ConfigError> {
    let file = get_world_dir().join("tile_references.ron");
    TileDefinitions::load(file)
}

pub fn load_level(level_file: &PathBuf, world: &mut World) -> Result<(), ConfigError> {
    let mut win_condition = WinCondition::default();
    let display_debug_frames = world.read_resource::<DebugSettings>().display_debug_frames;
    let tile_defs = load_tile_definitions()?;
    let level = LevelSave::load(level_file)?;
    add_background(world, &level.world_bounds);
    level.tiles.iter().for_each(|(pos, tile_def_key)| {
        let tile_def = tile_defs.get(tile_def_key);
        let still_asset = load_still_asset(tile_def, &world.read_resource::<Assets>());
        let anim_asset = load_anim_asset(tile_def, &world.read_resource::<Assets>());
        let transform = if let Some(asset) = &tile_def.asset {
            Some(load_transform(
                &pos,
                &tile_def.depth,
                &tile_def.dimens,
                asset,
            ))
        } else {
            None
        };
        let mut builder = world.create_entity();
        if let Some(still_asset) = still_asset {
            builder = builder.with(still_asset);
        }
        if let Some(anim_asset) = anim_asset {
            builder = builder.with(anim_asset);
        }
        if let Some(transform) = transform {
            builder = builder.with(transform);
        }
        builder = builder.with(Block { pos: *pos });
        match tile_def.archetype {
            Some(Archetype::Player) => {
                let _ = build_player(builder, pos, tile_def);
                if display_debug_frames {
                    build_frames(world, tile_def);
                }
            }
            Some(Archetype::Key) => {
                win_condition.add_key(pos);
                builder.with(Key::new(*pos)).build();
            }
            Some(Archetype::Tool(tool_type)) => {
                if let Some(AssetType::Still(sprite, sprite_nr)) = tile_def.asset {
                    builder
                        .with(Tool::new(tool_type, sprite, sprite_nr))
                        .build();
                } else {
                    error!(
                        "Tool definition {:?} did not have still asset.",
                        tile_def_key
                    );
                }
            }
            Some(Archetype::Door) => {
                builder.with(ExitDoor).build();
            }
            _ => {
                builder.build();
            }
        };
    });
    add_key_displays_to_door(world, &win_condition);
    world.insert(win_condition);
    world.insert(TileMap::for_play(level, tile_defs));
    world.insert(History::default());
    Ok(())
}

fn build_player(builder: EntityBuilder, pos: &Pos, tile_def: &TileDefinition) -> Entity {
    builder
        .with(Transparent)
        .with(Velocity::default())
        .with(SteeringIntent::default())
        .with(Steering::new(*pos, tile_def.dimens))
        .with(Player::default())
        .build()
}

pub fn add_background(world: &mut World, world_bounds: &WorldBounds) {
    let transform = load_transform(
        &world_bounds.pos,
        &DepthLayer::Background,
        &world_bounds.dimens,
        &AssetType::Still(SpriteType::Selection, 1),
    );
    let asset = load_asset_from_world(&SpriteType::Selection, 1, world);
    world
        .create_entity()
        .with(BackgroundTag)
        .with(transform)
        .with(asset)
        .build();
}

fn add_key_displays_to_door(world: &mut World, win_condition: &WinCondition) {
    let door_entity = world.exec(|(doors, entities): (ReadStorage<ExitDoor>, Entities)| {
        (&doors, &entities)
            .join()
            .map(|(_, entity)| (entity))
            .next()
    });
    if let Some(door_entity) = door_entity {
        win_condition
            .keys
            .iter()
            .enumerate()
            .for_each(|(index, key)| {
                // Temporary bit of code to arrange the key displays on the door in a
                // visually pleasing manner. Rewrite this later, when we know exactly what we
                // want to do with the door.
                let i = if index < 2 {
                    index + 5
                } else if index < 4 {
                    index + 7
                } else if index < 5 {
                    index
                } else if index < 7 {
                    index - 5
                } else if index < 9 {
                    index
                } else if index < 11 {
                    index - 7
                } else {
                    index
                };
                let mut transform = Transform::default();
                let x_offset = i % 4;
                let y_offset = i / 4;
                transform.set_translation_x((-1.5 + x_offset as f32) * 64.);
                transform.set_translation_y((-1.5 + y_offset as f32) * 64.);
                transform.set_translation_z(1.); //One higher than parent.
                transform.set_scale(Vector3::new(0.5, 0.5, 1.0));
                let sprite = load_asset_from_world(&SpriteType::Blocks, 3, world);
                world
                    .create_entity()
                    .with(Parent {
                        entity: door_entity,
                    })
                    .with(transform)
                    .with(sprite)
                    .with(KeyDisplay::new(*key))
                    .build();
            });
    }
}

fn build_frames(world: &mut World, tile_def: &TileDefinition) {
    let frame = world
        .read_resource::<Assets>()
        .get_still(&SpriteType::Frame);
    let sprite_render = SpriteRender {
        sprite_sheet: frame,
        sprite_number: 0, // First sprite
    };

    let steering_ghost_transform = load_transform(
        &Pos::default(),
        &DepthLayer::UiElements,
        &tile_def.dimens,
        &AssetType::Still(SpriteType::Frame, 0),
    );
    world
        .create_entity()
        .with(sprite_render.clone())
        .with(steering_ghost_transform)
        .with(DebugSteeringGhostTag)
        .build();
    let pos_ghost_transform = load_transform(
        &Pos::default(),
        &DepthLayer::UiElements,
        &Pos::new(1, 1),
        &AssetType::Still(SpriteType::Frame, 0),
    );
    world
        .create_entity()
        .with(sprite_render)
        .with(pos_ghost_transform)
        .with(DebugPosGhostTag)
        .build();
}

pub fn load_transform(pos: &Pos, depth: &DepthLayer, dimens: &Pos, asset: &AssetType) -> Transform {
    let asset_dimensions = get_asset_dimensions(&asset);
    let mut transform = Transform::default();
    transform.set_translation_xyz(
        pos.x as f32 + dimens.x as f32 * 0.5,
        pos.y as f32 + dimens.y as f32 * 0.5,
        depth.z(),
    );
    transform.set_scale(Vector3::new(
        dimens.x as f32 / asset_dimensions.x as f32,
        dimens.y as f32 / asset_dimensions.y as f32,
        1.0,
    ));
    transform
}

pub fn load_still_asset(tile: &TileDefinition, assets: &Assets) -> Option<SpriteRender> {
    match &tile.asset? {
        AssetType::Animated(..) => None,
        AssetType::Still(spritesheet, sprite_nr) => {
            let handle = assets.get_still(&spritesheet);
            Some(SpriteRender {
                sprite_sheet: handle,
                sprite_number: *sprite_nr,
            })
        }
    }
}

pub fn load_asset_from_world(
    sprite: &SpriteType,
    sprite_nr: usize,
    world: &mut World,
) -> SpriteRender {
    let assets = world.read_resource::<Assets>();
    SpriteRender {
        sprite_sheet: assets.get_still(sprite),
        sprite_number: sprite_nr,
    }
}

pub fn load_sprite_render(sprite: &SpriteType, sprite_nr: usize, assets: &Assets) -> SpriteRender {
    SpriteRender {
        sprite_sheet: assets.get_still(sprite),
        sprite_number: sprite_nr,
    }
}

pub fn load_anim_asset(
    tile: &TileDefinition,
    assets: &Assets,
) -> Option<Handle<Prefab<MyPrefabData>>> {
    match &tile.asset? {
        AssetType::Animated(anim) => {
            let handle = assets.get_animated(&anim);
            Some(handle)
        }
        AssetType::Still(..) => None,
    }
}

pub fn attach_graphics(
    world: &mut World,
    entity: Entity,
    asset: &AssetType,
    dimens: &Pos,
    tint: Option<Tint>,
) -> Entity {
    let still_asset = if let AssetType::Still(sprite, sprite_nr) = *asset {
        Some(load_asset_from_world(&sprite, sprite_nr, world))
    } else {
        None
    };

    let mut builder = world.create_entity();
    if let Some(still_asset) = still_asset {
        builder = builder.with(still_asset);
    }
    // if let Some(anim_asset) = anim_asset {
    //     builder = builder.with(anim_asset);
    // }
    if let Some(tint) = tint {
        builder = builder.with(tint);
    }
    builder
        .with(Transparent) // TODO: only attach this when needed.
        .with(Parent { entity })
        .with(transform_scale(dimens, asset))
        .build()
}

fn transform_scale(dimens: &Pos, asset: &AssetType) -> Transform {
    let asset_dimensions = get_asset_dimensions(&asset);
    let mut transform = Transform::default();
    transform.set_scale(Vector3::new(
        dimens.x as f32 / asset_dimensions.x as f32,
        dimens.y as f32 / asset_dimensions.y as f32,
        1.0,
    ));
    transform
}
