use amethyst::audio::SourceHandle;
use rand::prelude::SliceRandom;
use rand::thread_rng;

use std::iter::Cycle;
use std::vec::IntoIter;

/// This is a Music resource that is passed to Amethyst's DJSystem. It simply loops through a
/// shuffled list of music tracks.
/// In the (far) future, it should intelligently play music based on the level's atmosphere etc.
pub struct Music {
    pub music: Cycle<IntoIter<SourceHandle>>,
}

impl Music {
    pub fn new(mut tracks: Vec<SourceHandle>) -> Self {
        tracks.shuffle(&mut thread_rng());
        let music = tracks.into_iter().cycle();
        Music { music }
    }
}
