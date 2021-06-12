use crate::components::{ControlledCharacter, Velocity};
use amethyst::{
    assets::{Handle, Prefab, PrefabData, PrefabLoader, ProgressCounter, RonFormat},
    derive::PrefabData,
    ecs::prelude::Entity,
    error::Error,
    prelude::World,
    renderer::sprite::prefab::SpriteScenePrefab,
};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, PrefabData)]
pub struct PlayerPrefab {
    sprite_scene: SpriteScenePrefab,
    velocity: Velocity,
    controlled_character: ControlledCharacter,
}

pub fn load_player(
    world: &mut World,
    progress_counter: &mut ProgressCounter,
) -> Handle<Prefab<PlayerPrefab>> {
    world.exec(|loader: PrefabLoader<'_, PlayerPrefab>| {
        loader.load("prefabs/player.ron", RonFormat, progress_counter)
    })
}
