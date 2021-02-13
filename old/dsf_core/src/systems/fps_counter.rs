use amethyst::{
    core::Time,
    ecs::prelude::{Entity, Read, System, WriteStorage},
    ui::{UiFinder, UiText},
    utils::fps_counter::FpsCounter,
};

#[derive(Default)]
pub struct FpsCounterUiSystem {
    maybe_fps_entity: Option<Entity>,
}

impl<'a> System<'a> for FpsCounterUiSystem {
    type SystemData = (
        Read<'a, Time>,
        WriteStorage<'a, UiText>,
        Read<'a, FpsCounter>,
        UiFinder<'a>,
    );

    fn run(&mut self, (time, mut ui_text, fps_counter, finder): Self::SystemData) {
        if time.frame_number() % 20 != 0 {
            // This system should only be executed every 20 frames.
            return;
        }
        // Grab the UiText component from the fps ui entity.
        // If that entity doesn't exist or is expired, try to obtain an up-to-date handle.
        let fps_text = {
            if self.maybe_fps_entity.is_none() {
                self.maybe_fps_entity = finder.find("fps_text");
            }
            if let Some(fps_entity) = self.maybe_fps_entity {
                let maybe_component = ui_text.get_mut(fps_entity);
                if maybe_component.is_none() {
                    self.maybe_fps_entity = finder.find("fps_text");
                }
                maybe_component
            } else {
                None
            }
        };
        if let Some(mut fps_text) = fps_text {
            let fps = fps_counter.sampled_fps();
            fps_text.text = format!("FPS: {:.*}", 2, fps);
        }
    }
}
