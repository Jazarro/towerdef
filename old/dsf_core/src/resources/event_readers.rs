use amethyst::core::ecs::shrev::EventChannel;
use amethyst::core::ecs::{Read, ReaderId, World};
use amethyst::input::{InputEvent, StringBindings};
use std::collections::HashMap;

/// This is a resource that holds a collection of ReaderIds for state-bound systems.
/// When a system that belongs to a State-specific dispatcher needs a ReaderId it should get it
/// from this resource.
///
/// While the State is active (IE: on top of the stack), the system reads from
/// the event channel every frame.
///
/// While the State is not active, (IE: not on top of the stack),
/// the State should read from the event channel in its shadow update method.
/// This ensures that the channel is always read from on every frame and events thus cannot
/// bunch up.
#[derive(Debug, Default)]
pub struct EventReaders {
    // TODO: Only works for one type of events. Either make generic or find different solution?
    reader_ids: HashMap<String, ReaderId<InputEvent<StringBindings>>>,
}

impl EventReaders {
    pub fn add_reader(mut self, system_name: String, world: &mut World) -> Self {
        let reader_id = world
            .fetch_mut::<EventChannel<InputEvent<StringBindings>>>()
            .register_reader();
        self.reader_ids.insert(system_name, reader_id);
        self
    }

    pub fn get_reader_id(
        &mut self,
        system_name: &str,
    ) -> Option<&mut ReaderId<InputEvent<StringBindings>>> {
        self.reader_ids.get_mut(system_name)
    }

    /// The State should call this method every frame if it is not active.
    pub fn drain_event_channel(&mut self, channel: Read<EventChannel<InputEvent<StringBindings>>>) {
        self.reader_ids.values_mut().for_each(|mut reader_id| {
            channel.read(&mut reader_id).for_each(|_| ());
        });
    }
}
