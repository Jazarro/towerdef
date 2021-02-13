use crate::resources::{EditorStatus, LevelEdit};
use crate::systems::RefreshPreviewsEvent;
use amethyst::core::ecs::shrev::EventChannel;
use amethyst::core::ecs::{Read, System, Write, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};
use amethyst::ui::{UiFinder, UiImage};
use dsf_core::levels::load_sprite_render;
use dsf_core::resources::{AssetType, Assets, SignalEdge, SignalEdgeDetector, SpriteType};

/// Responsible for changing transient configurations for the editor. These settings stay alive
/// as long as the EditorState lives.
///
/// Currently, this system is responsible for:
///
/// - Changing what tile is on the brush.
/// - Toggling the copy-air flag.
/// - Toggling the force-place flag.
///
pub struct ConfigureEditorSystem;

impl<'s> System<'s> for ConfigureEditorSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Write<'s, EventChannel<RefreshPreviewsEvent>>,
        Read<'s, InputHandler<StringBindings>>,
        Write<'s, SignalEdgeDetector>,
        Write<'s, EditorStatus>,
    );

    fn run(&mut self, (mut channel, input, mut sed, mut status): Self::SystemData) {
        if let SignalEdge::Rising = sed.edge("select_previous_brush", &input) {
            let _new_key = status.brush.select_previous();
            channel.single_write(RefreshPreviewsEvent);
        }
        if let SignalEdge::Rising = sed.edge("select_next_brush", &input) {
            let _new_key = status.brush.select_next();
            channel.single_write(RefreshPreviewsEvent);
        }
        if let SignalEdge::Rising = sed.edge("toggle_copy_air", &input) {
            status.copy_air ^= true;
            channel.single_write(RefreshPreviewsEvent);
        }
        if let SignalEdge::Rising = sed.edge("toggle_force_place", &input) {
            status.force_place ^= true;
            channel.single_write(RefreshPreviewsEvent);
        }
    }
}

/// Updates the UI images for the copy-air and force-place flags and for the active brush.
pub struct EditorUiUpdateSystem;

impl<'s> System<'s> for EditorUiUpdateSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        WriteStorage<'s, UiImage>,
        UiFinder<'s>,
        Read<'s, EditorStatus>,
        Read<'s, LevelEdit>,
        Read<'s, Assets>,
    );

    fn run(&mut self, (mut ui_image, finder, status, level_edit, assets): Self::SystemData) {
        let toggle_copy_air = get_image("toggle_copy_air", &finder, &mut ui_image);
        if let Some(toggle_copy_air) = toggle_copy_air {
            let sprite_nr = if status.copy_air { 0 } else { 1 };
            *toggle_copy_air = UiImage::Sprite(load_sprite_render(
                &SpriteType::EditorUiIcons,
                sprite_nr,
                &assets,
            ));
        }
        let toggle_force_place = get_image("toggle_force_place", &finder, &mut ui_image);
        if let Some(toggle_force_place) = toggle_force_place {
            let sprite_nr = 2 + if status.force_place { 0 } else { 1 };
            *toggle_force_place = UiImage::Sprite(load_sprite_render(
                &SpriteType::EditorUiIcons,
                sprite_nr,
                &assets,
            ));
        }
        let brush_preview = get_image("brush_preview", &finder, &mut ui_image);
        if let Some(brush_preview) = brush_preview {
            if let Some(sprite_render) = status
                .brush
                .get_key()
                .as_ref()
                .map(|selected_key| level_edit.get_tile_def(selected_key))
                .map(|tile_def| {
                    if let AssetType::Still(sprite, sprite_nr) = tile_def.get_preview() {
                        Some(load_sprite_render(&sprite, sprite_nr, &assets))
                    } else {
                        None
                    }
                })
                .flatten()
            {
                *brush_preview = UiImage::Sprite(sprite_render);
            } else {
                *brush_preview = UiImage::SolidColor([0.0, 0.0, 0.0, 1.0]);
            }
        }
    }
}

fn get_image<'a>(
    key: &str,
    finder: &UiFinder,
    ui_image: &'a mut WriteStorage<UiImage>,
) -> Option<&'a mut UiImage> {
    let toggle_entity = finder.find(key);
    if let Some(toggle_entity) = toggle_entity {
        ui_image.get_mut(toggle_entity)
    } else {
        None
    }
}
