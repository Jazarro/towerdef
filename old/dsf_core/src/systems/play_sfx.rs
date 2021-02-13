use crate::resources::{Assets, AudioSettings, SoundType};
use amethyst::assets::AssetStorage;
use amethyst::audio::output::Output;
use amethyst::audio::Source;
use amethyst::core::ecs::shrev::EventChannel;
use amethyst::core::ecs::{Read, ReaderId, System};
use amethyst::core::shred::SystemData;

use amethyst::prelude::World;

/// Elsewhere in the application, you can broadcast SoundEvents. The PlaySfxSystem below listens
/// for such events and actually plays the sound effect that was requested.
#[derive(Debug, Clone)]
pub struct SoundEvent {
    sound_type: SoundType,
}

impl SoundEvent {
    pub fn new(sound_type: SoundType) -> Self {
        SoundEvent { sound_type }
    }
}

/// This system is responsible for playing non-location-dependent sound effects.
/// To play any sound effect, just broadcast a SoundEvent in the corresponding event channel.
/// This system will take care of the rest.
#[derive(Default)]
pub struct PlaySfxSystem {
    reader_id: Option<ReaderId<SoundEvent>>,
}

impl<'s> System<'s> for PlaySfxSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Read<'s, AudioSettings>,
        Read<'s, EventChannel<SoundEvent>>,
        Read<'s, Assets>,
        Read<'s, AssetStorage<Source>>,
        Read<'s, Output>,
    );

    fn run(&mut self, (config, sound_events, assets, sources, output): Self::SystemData) {
        let reader_id = self
            .reader_id
            .as_mut()
            .expect("`PlaySfxSystem::setup` was not called before `PlaySfxSystem::run`");

        for event in sound_events.read(reader_id) {
            if let Some(volume) = config.sound_effects_volume {
                let source = assets
                    .get_sound(&event.sound_type)
                    .map(|source_handle| sources.get(&source_handle))
                    .flatten();
                if let Some(source) = source {
                    output.play_once(source, volume);
                }
            }
        }
    }

    fn setup(&mut self, world: &mut World) {
        <Self as System<'_>>::SystemData::setup(world);
        self.reader_id = Some(
            world
                .fetch_mut::<EventChannel<SoundEvent>>()
                .register_reader(),
        )
    }
}
