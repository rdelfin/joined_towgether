use crate::components::{Guided, Hitable, Velocity};
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
pub struct EnemyPrefab {
    sprite_scene: SpriteScenePrefab,
    velocity: Velocity,
    guided: Guided,
    hitable: Hitable,
}

pub fn load_enemy(
    world: &mut World,
    path: &str,
    progress_counter: &mut ProgressCounter,
) -> Handle<Prefab<EnemyPrefab>> {
    world.exec(|loader: PrefabLoader<'_, EnemyPrefab>| {
        loader.load(path, RonFormat, progress_counter)
    })
}
