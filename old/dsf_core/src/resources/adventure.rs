use std::fs;

use std::path::PathBuf;

use amethyst::prelude::*;

use serde::{Deserialize, Serialize};

use crate::components::*;
use crate::levels::{load_asset_from_world, load_transform, LevelSave};
use crate::resources::{AssetType, DepthLayer, SpriteType, UserCache};
use crate::utility::files::{get_adventures_dir, get_levels_dir};
use amethyst::config::ConfigError;
use amethyst::renderer::palette::Srgba;
use amethyst::renderer::resources::Tint;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct PositionOnMap {
    pub pos: Pos,
}

impl PositionOnMap {
    pub fn new(pos: Pos) -> Self {
        PositionOnMap { pos }
    }
}

/// All adventures must start at position (0, 0).
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Adventure {
    pub(crate) nodes: HashMap<Pos, MapElement>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum MapElement {
    Road,
    Node(AdventureNode),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AdventureNode {
    pub name: String,
    // pub description: String,
    pub details: NodeDetails,
    // If true, the player must defeat this node before they can move further.
    // If false, nodes behind this node are reachable and playable even if this node was never
    // entered.
    // pub blocking: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum NodeDetails {
    /// This node is an adventure: a collection of levels.
    /// Opening this node will push a new LevelSelectState for this adventure.
    Adventure(String),
    /// This node is a level. Opening this node will open the level in the PlayState.
    Level(String),
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Road {
    pub start_id: u16,
    pub end_id: u16,
}

/// Creates a new adventure that gives access to every single level.
/// This is useful while there aren't too many levels yet.
pub fn create_default_adventure() {
    let mut adventure = Adventure::default();
    level_files()
        .iter()
        .map(|level_name| {
            let level_file = get_levels_dir().join(level_name);
            (level_name, LevelSave::load(level_file))
        })
        .filter(|(level_name, result)| {
            result
                .as_ref()
                .map_err(|err| {
                    error!("Failed to load level {:?}: {:?}", level_name, err);
                    err
                })
                .is_ok()
        })
        .map(|(level_name, result)| (level_name, result.expect("Should never panic.")))
        .enumerate()
        .for_each(|(index, (level_name, _level))| {
            adventure.nodes.insert(
                Pos::new((index * 2) as i32, 0),
                MapElement::Node(AdventureNode {
                    name: level_name.clone(),
                    details: NodeDetails::Level(level_name.clone()),
                }),
            );
            if index > 0 {
                adventure
                    .nodes
                    .insert(Pos::new((index * 2 - 1) as i32, 0), MapElement::Road);
            }
        });

    adventure
        .write(get_adventures_dir().join("default.ron"))
        .expect("Failed to create default adventure that contains all levels.");
}

fn level_files() -> Vec<String> {
    fs::read_dir(get_levels_dir())
        .expect("Failed to read contents of the levels directory.")
        .map(|file| {
            if let Ok(file) = file {
                if file.path().is_file() {
                    Some(
                        file.path()
                            .file_name()
                            .expect("This should not happen.")
                            .to_str()
                            .expect("Music file name did not contain valid unicode.")
                            .to_string(),
                    )
                } else {
                    None
                }
            } else {
                None
            }
        })
        .filter(|option| option.is_some())
        .map(|option| option.unwrap())
        .collect()
}

pub fn load_adventure(path: &PathBuf, world: &mut World) -> Result<(), ConfigError> {
    let adventure = Adventure::load(path)?;
    for (pos, map_element) in &adventure.nodes {
        match map_element {
            MapElement::Road => load_road(pos, world),
            MapElement::Node(node) => load_node(pos, node, world),
        }
    }
    let initial_cursor_pos = {
        let last_known_pos = cursor_position(path, world);
        if adventure.nodes.contains_key(&last_known_pos) {
            last_known_pos
        } else {
            Pos::default()
        }
    };
    load_cursor(world, &initial_cursor_pos);
    world.insert(adventure);
    world.insert(PositionOnMap::new(initial_cursor_pos));
    Ok(())
}

fn cursor_position(path: &PathBuf, world: &mut World) -> Pos {
    world.read_resource::<UserCache>().get_initial_cursor_pos(
        path.file_name()
            .expect("This should not happen.")
            .to_str()
            .expect("Adventure file name did not contain valid unicode."),
    )
}

fn load_cursor(world: &mut World, pos: &Pos) {
    let sprite_render = load_asset_from_world(&SpriteType::LevelSelect, 3, world);
    let transform = load_transform(
        &pos,
        &DepthLayer::Player,
        &Pos::new(1, 1),
        &AssetType::Still(SpriteType::LevelSelect, 3),
    );
    world
        .create_entity()
        .with(MapCursor::default())
        .with(Tint(Srgba::new(0.5, 0., 0., 1.)))
        .with(transform)
        .with(sprite_render)
        .build();
}

fn load_road(pos: &Pos, world: &mut World) {
    let sprite_render_road = load_asset_from_world(&SpriteType::LevelSelect, 1, world);
    let transform = load_transform(
        pos,
        &DepthLayer::Blocks,
        &Pos::new(1, 1),
        &AssetType::Still(SpriteType::LevelSelect, 1),
    );
    world
        .create_entity()
        .with(transform)
        .with(sprite_render_road)
        .build();
}

fn load_node(pos: &Pos, _node: &AdventureNode, world: &mut World) {
    let sprite_render_node = load_asset_from_world(&SpriteType::LevelSelect, 0, world);
    let transform = load_transform(
        pos,
        &DepthLayer::Blocks,
        &Pos::new(1, 1),
        &AssetType::Still(SpriteType::LevelSelect, 0),
    );
    world
        .create_entity()
        .with(transform)
        .with(sprite_render_node)
        .build();
}
