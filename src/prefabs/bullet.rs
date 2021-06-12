use crate::components::Velocity;
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
pub struct BulletPrefab {
    sprite_scene: SpriteScenePrefab,
    velocity: Velocity,
}

pub fn load_bullet(
    world: &mut World,
    path: &str,
    progress_counter: &mut ProgressCounter,
) -> Handle<Prefab<BulletPrefab>> {
    world.exec(|loader: PrefabLoader<'_, BulletPrefab>| {
        loader.load(path, RonFormat, progress_counter)
    })
}
