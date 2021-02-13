use crate::components::Pos;
use crate::resources::WorldBounds;
use serde::{Deserialize, Serialize, Serializer};
use std::collections::{BTreeMap, HashMap};

/// Describes a complete level. This is the format that the level is stored in.
/// Contains a map of positions, mapped to tile definitions.
/// This struct can be loaded from a level file and used to start a game.
#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(default)]
#[serde(deny_unknown_fields)]
pub struct LevelSave {
    /// The level's exterior borders. In this game, the world wraps at the borders.
    pub world_bounds: WorldBounds,
    /// Mapping of (x,y) position in the world to a TileDefinition key.
    /// These keys can be used to look up the corresponding TileDefinition.
    #[serde(serialize_with = "ordered_map")]
    pub tiles: HashMap<Pos, String>,
}

/// A function used by serde to serialise the tile map in a deterministic way.
/// This will prevent the output being different each time the level is saved, which will
/// prevent lots of unnecessarily large diffs in the git commits.
fn ordered_map<S>(value: &HashMap<Pos, String>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let ordered: BTreeMap<_, _> = value.iter().collect();
    ordered.serialize(serializer)
}
