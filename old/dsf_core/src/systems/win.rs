use amethyst::core::ecs::{Entities, LazyUpdate};
use amethyst::{
    core::math::Vector2,
    core::transform::Transform,
    ecs::prelude::{Join, Read, ReadStorage, System, Write},
};

use crate::components::*;
use crate::resources::*;
use crate::systems::SoundEvent;
use amethyst::core::ecs::shrev::EventChannel;

/// Key width and height, hardcoded for now.
/// TODO: Get rid of these hardcoded constants.
const KEY_WIDTH: f32 = 2.;
const KEY_HEIGHT: f32 = 2.;
const DOOR_WIDTH: f32 = 4.;
const DOOR_HEIGHT: f32 = 4.;

/// Checks if the player intersects any keys.
/// If so, the key will collected by the player and will be removed from the game.
#[derive(Default)]
pub struct KeyCollectionSystem;

impl<'s> System<'s> for KeyCollectionSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Write<'s, EventChannel<SoundEvent>>,
        ReadStorage<'s, Player>,
        ReadStorage<'s, Steering>,
        ReadStorage<'s, Key>,
        ReadStorage<'s, KeyDisplay>,
        ReadStorage<'s, Transform>,
        Write<'s, WinCondition>,
        Entities<'s>,
    );

    fn run(
        &mut self,
        (
            mut sound_channel,
            player_tags,
            steerings,
            keys,
            key_displays,
            transforms,
            mut win,
            entities,
        ): Self::SystemData,
    ) {
        let player_collider = (&player_tags, &steerings, &transforms)
            .join()
            .map(|(_, steering, transform)| {
                (
                    Vector2::new(transform.translation().x, transform.translation().y),
                    Vector2::new(steering.dimens.x as f32, steering.dimens.y as f32),
                )
            })
            .next();
        if let Some((pos, dimens)) = player_collider {
            let collected_key = (&keys, &transforms, &entities)
                .join()
                .filter(|(_, transform, _)| {
                    let key_x = transform.translation().x;
                    let key_y = transform.translation().y;
                    pos.x - dimens.x / 2. < key_x + KEY_WIDTH / 3.
                        && pos.x + dimens.x / 2. > key_x - KEY_WIDTH / 3.
                        && pos.y - dimens.y / 2. < key_y + KEY_HEIGHT / 3.
                        && pos.y + dimens.y / 2. > key_y - KEY_HEIGHT / 3.
                })
                .map(|(key, _, entity)| (key, entity))
                .next();
            if let Some((key, key_entity)) = collected_key {
                sound_channel.single_write(SoundEvent::new(SoundType::KeyPickup));
                win.set_key_collected(&key.pos);
                entities.delete(key_entity).expect("Failed to delete key.");
                for (key_display, display_entity) in (&key_displays, &entities).join() {
                    if key_display.pos == key.pos {
                        entities
                            .delete(display_entity)
                            .expect("Failed to delete key display!");
                    }
                }
            }
        }
    }
}

/// Checks if the player has finished the level.
/// The player finishes the level when they collect all keys and then reach the exit door.
#[derive(Default)]
pub struct WinSystem;

impl<'s> System<'s> for WinSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Write<'s, EventChannel<SoundEvent>>,
        ReadStorage<'s, Player>,
        ReadStorage<'s, Steering>,
        ReadStorage<'s, ExitDoor>,
        ReadStorage<'s, Transform>,
        Write<'s, WinCondition>,
        Read<'s, LazyUpdate>,
    );

    fn run(
        &mut self,
        (mut sound_channel, players, steerings, doors, transforms, mut win, lazy): Self::SystemData,
    ) {
        if win.reached_open_door || !win.all_keys_collected() {
            return;
        }
        let player_collider = (&players, &steerings, &transforms)
            .join()
            .map(|(_, steering, transform)| {
                (
                    Vector2::new(transform.translation().x, transform.translation().y),
                    Vector2::new(steering.dimens.x as f32, steering.dimens.y as f32),
                )
            })
            .next();
        if let Some((pos, dimens)) = player_collider {
            for (_, door_transform) in (&doors, &transforms).join() {
                let door_x = door_transform.translation().x;
                let door_y = door_transform.translation().y;
                if pos.x - dimens.x / 2. < door_x + DOOR_WIDTH / 3.
                    && pos.x + dimens.x / 2. > door_x - DOOR_WIDTH / 3.
                    && pos.y - dimens.y / 2. < door_y + DOOR_HEIGHT / 3.
                    && pos.y + dimens.y / 2. > door_y - DOOR_HEIGHT / 3.
                {
                    sound_channel.single_write(SoundEvent::new(SoundType::Win));
                    win.reached_open_door = true;
                    lazy.exec_mut(move |world| {
                        UiHandles::add_ui(&UiType::WinMessage, world);
                    });
                }
            }
        }
    }
}
