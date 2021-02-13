#![forbid(unsafe_code)]

//! Inspired by the user Cart on the Amethyst forums:
//! https://community.amethyst.rs/t/better-compile-times/1264
//!
//! This crate bundles some code that takes a tong time to compile but that rarely changes during
//! development of the game. That way it doesn't need to be recompiled every time.

pub use self::bundles::{PrecompiledDefaultsBundle, PrecompiledRenderBundle};
pub use self::startup::*;
pub use self::structs::{AnimationId, MyPrefabData};

mod bundles;
mod startup;
mod structs;
