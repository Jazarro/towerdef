use amethyst::core::ecs::{Read, System, Write};
use amethyst::input::{InputHandler, StringBindings};
use amethyst::shrev::EventChannel;

use dsf_core::resources::{SignalEdge, SignalEdgeDetector};

use crate::resources::{Blueprint, EditorData, LevelEdit};
use crate::systems::RefreshPreviewsEvent;

/// Responsible for placing and removing tiles based on player input.
pub struct PlaceTilesSystem;

impl<'s> System<'s> for PlaceTilesSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Write<'s, EventChannel<RefreshPreviewsEvent>>,
        Write<'s, SignalEdgeDetector>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, EditorData>,
        Write<'s, LevelEdit>,
    );

    fn run(
        &mut self,
        (mut channel, mut sed, input, editor_data, mut level_edit): Self::SystemData,
    ) {
        if let SignalEdge::Rising = sed.edge("place_blocks", &input) {
            let blueprint = Blueprint::from_placing_tiles(&editor_data, &level_edit);
            let lower_bounds = editor_data.selection.lower_bounds();
            blueprint.tiles.iter().for_each(|(relative_pos, tile)| {
                level_edit.put_tile(editor_data.force_place, lower_bounds + *relative_pos, Some(tile.clone()));
            });
            channel.single_write(RefreshPreviewsEvent);
        }
        if let SignalEdge::Rising = sed.edge("delete_blocks", &input) {
            let lower_bounds = editor_data.selection.lower_bounds();
            let selection_dimens = editor_data.selection.dimens();
            (0..selection_dimens.x).for_each(|x| {
                (0..selection_dimens.y).for_each(|y| {
                    level_edit.put_tile(true, lower_bounds.append_xy(x, y), None);
                });
            });
            channel.single_write(RefreshPreviewsEvent);
        }
    }
}
