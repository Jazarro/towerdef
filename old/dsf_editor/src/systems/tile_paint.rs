use amethyst::{
    assets::{Handle, Prefab},
    core::transform::Transform,
    ecs::prelude::{Entities, Join, Read, ReadStorage, System, WriteStorage},
    renderer::sprite::SpriteRender,
};

use dsf_core::components::BackgroundTag;
use dsf_core::levels::{load_anim_asset, load_still_asset, load_transform};
use dsf_core::resources::Assets;
use dsf_precompile::MyPrefabData;

use crate::components::*;
use crate::resources::*;
use amethyst::core::ecs::Write;
use amethyst::renderer::palette::Srgba;
use amethyst::renderer::resources::Tint;

/// Clears all dirty tiles, then adds all dirty tiles back in.
/// At the end of this system's execution, no tiles should be left dirty.
pub struct TilePaintSystem;

impl<'s> System<'s> for TilePaintSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        ReadStorage<'s, BackgroundTag>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, SpriteRender>,
        WriteStorage<'s, Handle<Prefab<MyPrefabData>>>,
        WriteStorage<'s, PaintedTile>,
        WriteStorage<'s, Tint>,
        Read<'s, Assets>,
        Write<'s, LevelEdit>,
        Entities<'s>,
    );

    fn run(
        &mut self,
        (
            _backgrounds,
            mut transforms,
            mut sprite_renders,
            mut anims,
            mut painted_tiles,
            mut tints,
            assets,
            mut level_edit,
            entities,
        ): Self::SystemData,
    ) {
        // First delete all dirty entities:
        for (_, entity) in (&painted_tiles, &entities)
            .join()
            .filter(|(painted_tile, _)| level_edit.dirty.contains(&painted_tile.pos))
        {
            entities
                .delete(entity)
                .expect("Failed to delete tile sprite.");
        }

        // Then create new entities for all dirty positions.
        // These are the entities that were changed or newly added.
        level_edit
            .drain_dirty()
            .drain(..)
            // Do not create entities for dummy tiles:
            .filter(|pos| level_edit.tile_map.is_tile_def_key(&pos))
            .map(|dirty_pos| {
                let tile_def = level_edit.tile_map.get_tile(&dirty_pos)
                    .expect("Cannot panic, we previously checked that there is a proper tile in this location.");
                (dirty_pos, tile_def)
            })
            .for_each(|(pos, tile_def)| {
                let still_asset = load_still_asset(tile_def, &assets);
                let anim_asset = load_anim_asset(tile_def, &assets);
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
                let mut builder = entities.build_entity();
                if let Some(still_asset) = still_asset {
                    builder = builder.with(still_asset, &mut sprite_renders);
                }
                if let Some(anim_asset) = anim_asset {
                    builder = builder.with(anim_asset, &mut anims);
                }
                if let Some(transform) = transform {
                    builder = builder.with(transform, &mut transforms);
                }
                builder
                    .with(Tint(Srgba::new(1., 1., 1., 1.)), &mut tints)
                    .with(PaintedTile::new(pos), &mut painted_tiles)
                    .build();
            });
    }
}
