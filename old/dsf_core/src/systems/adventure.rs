use crate::components::{Direction2D, MapCursor};
use crate::resources::{
    Adventure, AdventureNode, MapElement, MovementConfig, NodeDetails, PositionOnMap, SoundType,
};
use crate::systems::SoundEvent;
use amethyst::core::ecs::{Join, Read, System, Write, WriteStorage};
use amethyst::core::{Time, Transform};
use amethyst::input::{InputHandler, StringBindings};
use amethyst::shrev::EventChannel;
use amethyst::ui::{UiFinder, UiText};

/// Responsible for moving the map cursor in the adventure and level selection.
pub struct MapCursorSystem;

impl<'s> System<'s> for MapCursorSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Write<'s, EventChannel<SoundEvent>>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, MapCursor>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Time>,
        Read<'s, MovementConfig>,
        Read<'s, Adventure>,
        Write<'s, PositionOnMap>,
    );

    fn run(
        &mut self,
        (
            mut sound_channel,
            mut transforms,
            mut cursors,
            input,
            time,
            config,
            adventure,
            mut pos_on_map,
        ): Self::SystemData,
    ) {
        for (cursor, transform) in (&mut cursors, &mut transforms).join() {
            let input_x = input.axis_value("move_x").unwrap_or(0.0);
            let input_y = input.axis_value("move_y").unwrap_or(0.0);
            let new_direction = Direction2D::new(input_x, input_y);
            if cursor.last_direction.is_neutral() && !new_direction.is_neutral() {
                // Start movement now. Move once, then set cooldown to High.
                move_cursor(
                    &new_direction,
                    &mut pos_on_map,
                    transform,
                    &adventure,
                    &mut sound_channel,
                );
                cursor.cooldown = config.map_cursor_move_high_cooldown;
            } else if cursor.last_direction.is_opposite(&new_direction) {
                // Reset movement. Set cooldown to high.
                cursor.cooldown = config.map_cursor_move_high_cooldown;
            } else if !new_direction.is_neutral() {
                // continue movement. Tick down cooldown.
                // If cooldown is due, move once and reset cooldown to Low.
                cursor.cooldown -= time.delta_seconds();
                if cursor.cooldown.is_sign_negative() {
                    cursor.cooldown = config.map_cursor_move_low_cooldown;
                    move_cursor(
                        &new_direction,
                        &mut pos_on_map,
                        transform,
                        &adventure,
                        &mut sound_channel,
                    );
                }
            }
            cursor.last_direction = new_direction;
        }
    }
}

/// Move on both x and y directions if possible. If the target position is not available, move
/// on just the x-axis. If that position is not available either, move on just the y-axis.
fn move_cursor(
    direction: &Direction2D,
    pos_on_map: &mut PositionOnMap,
    transform: &mut Transform,
    adventure: &Adventure,
    sound_channel: &mut EventChannel<SoundEvent>,
) {
    let target_pos = if !direction.x.is_neutral() {
        pos_on_map.pos.append_x(direction.x.signum_i())
    } else {
        pos_on_map.pos.append_y(direction.y.signum_i())
    };

    if adventure.nodes.contains_key(&target_pos) {
        pos_on_map.pos = target_pos;
        transform.set_translation_x(pos_on_map.pos.x as f32 + 0.5);
        transform.set_translation_y(pos_on_map.pos.y as f32 + 0.5);
        sound_channel.single_write(SoundEvent::new(SoundType::MapStep));
    }
}

/// Updates the UI label on the adventure and level select screen. The label must always display the
/// name of the currently selected node.
pub struct LevelSelectUiUpdateSystem;

impl<'s> System<'s> for LevelSelectUiUpdateSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        WriteStorage<'s, UiText>,
        UiFinder<'s>,
        Read<'s, Adventure>,
        Read<'s, PositionOnMap>,
    );

    fn run(&mut self, (mut ui_text, finder, adventure, pos_on_map): Self::SystemData) {
        let label_title = {
            let label_title_entity = finder.find("label_node_title");
            if let Some(fps_entity) = label_title_entity {
                ui_text.get_mut(fps_entity)
            } else {
                None
            }
        };
        if let Some(mut label_title) = label_title {
            let selected = adventure.nodes.get(&pos_on_map.pos);
            let selected_title = match selected {
                Some(MapElement::Node(AdventureNode {
                    details: NodeDetails::Level(file_name),
                    ..
                })) => file_name,
                _ => "Nothing",
            };
            label_title.text = format!("Selected: {:?}", selected_title);
        }
    }
}
