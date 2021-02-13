use dsf_core::resources::TileDefinitions;

#[derive(Debug, Default)]
pub struct Brush {
    palette: Vec<Option<String>>,
    palette_index: usize,
}

impl Brush {
    pub fn set_palette(&mut self, defs: &TileDefinitions) {
        self.palette.clear();
        self.palette.push(None);
        defs.map.keys().for_each(|key| {
            self.palette.push(Some(key.clone()));
        });
        self.palette.sort();
    }
    pub fn select_previous(&mut self) -> Option<String> {
        self.select(-1)
    }
    pub fn select_next(&mut self) -> Option<String> {
        self.select(1)
    }

    fn select(&mut self, offset: i32) -> Option<String> {
        self.palette_index =
            (self.palette_index as i32 + offset).rem_euclid(self.palette.len() as i32) as usize;
        let new_key = self.get_key();
        info!("Selected brush: {:?}", new_key);
        new_key.clone()
    }

    pub fn get_key(&self) -> &Option<String> {
        self.palette
            .get(self.palette_index)
            .expect("Should not panic.")
    }
}
