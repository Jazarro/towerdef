use amethyst::{
    assets::PrefabData,
    derive::PrefabData,
    ecs::{prelude::Entity, Component, NullStorage, WriteStorage},
    error::Error,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, PrefabData)]
#[prefab(Component)]
#[serde(deny_unknown_fields)]
pub struct SelectionTag;

impl Component for SelectionTag {
    type Storage = NullStorage<Self>;
}
