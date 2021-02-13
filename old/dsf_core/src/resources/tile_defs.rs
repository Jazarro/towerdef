use crate::components::Pos;
use crate::resources::{AssetType, SpriteType};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// This resource stores tile definitions. It is used in both the level editor and the actual game.
/// Definitions are loaded from a file. Each tile definition describes the properties of a type of
/// tile or entity that can be encountered in a level. Level files only contain string references to
/// tile definitions. This protects level files from becoming outdated when tile definitions are
/// updated.
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(default)]
#[serde(deny_unknown_fields)]
pub struct TileDefinitions {
    /// Fallback value returned if a requested value cannot be found.
    fallback: TileDefinition,
    pub map: HashMap<String, TileDefinition>,
}

impl Default for TileDefinitions {
    fn default() -> Self {
        TileDefinitions {
            fallback: TileDefinition::fallback(),
            map: HashMap::default(),
        }
    }
}

impl<'a> TileDefinitions {
    pub fn get(&'a self, key: &str) -> &'a TileDefinition {
        self.map
            .get(key)
            .or_else(|| {
                error!("Failed to find tile definition {:?}, using fallback.", key);
                Some(&self.fallback)
            })
            .unwrap_or_else(|| unreachable!("Is unreachable because we use a fallback!"))
    }
}

///TODO:
/// Name of struct. Tile?Block? What do we call the dimens unit? Tile? Meter? Grid space?
/// asset not necessarily the same as the asset in editor. (Mob spawn)
/// this tile map resource that is created, doesn't need to contain ALL tiles.
///
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(default)]
#[serde(deny_unknown_fields)]
pub struct TileDefinition {
    /// Z-layering information. What should be drawn behind this tile and what should be drawn in
    /// front of this tile?
    pub depth: DepthLayer,
    /// How wide and high is the tile?
    pub dimens: Pos,
    /// This tile is unique in the level. Only one tile with this definition can appear in the level.
    /// Examples are the player and the exit door.
    pub unique: bool,
    /// This tile is mandatory for each level. A level cannot be played without at least one tile
    /// with this definition. Examples are the player and the exit door. Note that in combination
    /// with 'unique', a tile can be required to appear EXACTLY once in each level.
    pub mandatory: bool,
    /// Whether you can climb up and down on this block. For ladders, this is true.
    pub climbable: bool,
    /// Collision data for the tile. Is optional, because not all tiles collide.
    pub collision: Option<CollisionDefinition>,
    /// The graphical asset to use for this tile. Is optional, because not all tiles have an asset.
    pub asset: Option<AssetType>,
    /// If the asset is an animation or doesn't exist at all, this must be Some(AssetType::Still()).
    /// Is mainly used in the editor. This is there to guarantee that we can have still-image
    /// representations of any tile, so it can be displayed in the editor GUI.
    pub preview_asset: Option<AssetType>,
    /// Use this if there are any special components or child-entities that should be attached to
    /// this tile.
    pub archetype: Option<Archetype>,
    /// If and under what circumstances this block can be destroyed.
    pub sturdiness: Sturdiness,
}

impl TileDefinition {
    /// Use the fallback if the real TileDefinition could not be found.
    /// This avoids the game having to panic if a level file is slightly corrupted or out of date.
    pub fn fallback() -> Self {
        TileDefinition {
            depth: DepthLayer::Blocks,
            dimens: Pos::new(1, 1),
            unique: false,
            mandatory: false,
            climbable: false,
            collision: None,
            asset: Some(AssetType::Still(SpriteType::NotFound, 0)),
            preview_asset: None,
            archetype: None,
            sturdiness: Sturdiness::Invulnerable,
        }
    }

    /// True if and only if the tile collides at the top.
    /// In other words, if you can stand on top of this tile.
    pub fn provides_platform(&self) -> bool {
        if let Some(collision) = &self.collision {
            collision.collides_top
        } else {
            false
        }
    }

    /// True if and only if the tile collides on the right and left sides.
    pub fn collides_horizontally(&self) -> bool {
        if let Some(collision) = &self.collision {
            collision.collides_side
        } else {
            false
        }
    }

    /// True if and only if the tile collides on the bottom.
    pub fn collides_bottom(&self) -> bool {
        if let Some(collision) = &self.collision {
            collision.collides_bottom
        } else {
            false
        }
    }

    pub fn is_breakable(&self) -> bool {
        self.sturdiness == Sturdiness::Breakable
    }

    /// This returns the preview asset if it exists and the standard asset otherwise.
    /// Mainly used by the editor.
    pub fn get_preview(&self) -> AssetType {
        self.preview_asset.or(self.asset).unwrap_or_default()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum DepthLayer {
    Background,
    DebugLines,
    // Leave room to expand background into multiple parallax layers here
    Blocks,
    Selection,
    Cursor,
    FloatingBlocks,
    Enemies,
    Player,
    Particles,
    /// Any UI elements that exist in world space. If we'd want a health bar above an enemy's head,
    /// this is the z-layer we'd use.
    /// Currently, it is used for some debugging elements, such as the player frames.
    /// In the editor, it is used for the cursor and selection.
    UiElements,
    Camera,
}

impl Default for DepthLayer {
    fn default() -> Self {
        DepthLayer::Blocks
    }
}

impl DepthLayer {
    pub fn z(&self) -> f32 {
        match self {
            DepthLayer::Background => 0.,
            DepthLayer::DebugLines => 1.,
            DepthLayer::Blocks => 100.,
            DepthLayer::Selection => 101.,
            DepthLayer::Cursor => 102.,
            DepthLayer::FloatingBlocks => 110.,
            DepthLayer::Enemies => 120.,
            DepthLayer::Player => 130.,
            DepthLayer::Particles => 140.,
            DepthLayer::UiElements => 200.,
            DepthLayer::Camera => 300.,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
/// If there are any special rules that apply to this tile, the archetype signals this.
/// For example: a tile with the Archetype Player will be targeted by player input, etc.
///
/// Use a different archetype to attach special components or sub-entities to an entity.
pub enum Archetype {
    /// This tile is the spawn location for the player character.
    /// A special Player component must be attached to this entity.
    /// For debug purposes, some child entities may be attached to the player.
    Player,
    /// Level key. The objective is to collect them all. Each level should contain at least one.
    Key,
    /// After collecting all keys, finish level by reaching this door.
    Door,
    /// An item that can be picked up and equipped by the player.
    Tool(ToolType),
}

/// What it takes to break this block.
/// This enum has two varieties now (breakable or not breakable) but further nuances could be added later.
/// For example: more/less resistant to explosions, etc.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum Sturdiness {
    /// Cannot be targeted for demolition.
    Invulnerable,
    /// Can be broken by tools.
    Breakable,
}

impl Default for Sturdiness {
    fn default() -> Self {
        Sturdiness::Invulnerable
    }
}

#[derive(Debug, Copy, Clone, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub enum ToolType {
    /// This tool will break the blocks that the player is facing, n layers deep.
    /// The integer argument is how many layers of blocks will be broken.
    ///
    /// If the player is facing right and occupies blocks (0, 0) to (1, 1) inclusive, the blocks
    /// that are targeted are: (2, 0) to (1 + depth, 1) inclusive.
    BreakBlocksHorizontally(u8),
    /// This tool will break the blocks below the player, in the direction the player is facing,
    /// n layers deep. The integer argument is how many layers of blocks will be broken.
    ///
    /// If the player is facing right and occupies blocks (0, 0) to (1, 1) inclusive, the blocks
    /// that are targeted are: (1, -1) to (2, -depth) inclusive.
    BreakBlocksBelow(u8),
}

impl Default for ToolType {
    fn default() -> Self {
        ToolType::BreakBlocksHorizontally(0)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CollisionDefinition {
    /// Player can stand on these tiles. Examples include regular blocks and ladders.
    pub collides_top: bool,
    /// Player cannot move through these tiles horizontally. Examples include blocks.
    pub collides_side: bool,
    /// When standing underneath a two-high ledge of these tiles, the player cannot jump.
    pub collides_bottom: bool,
}
