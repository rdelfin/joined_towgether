use crate::components::Tower;
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
pub struct TowerPrefab {
    sprite_scene: SpriteScenePrefab,
    tower: Tower,
}

pub fn load_tower(
    world: &mut World,
    path: &str,
    progress_counter: &mut ProgressCounter,
) -> Handle<Prefab<TowerPrefab>> {
    world.exec(|loader: PrefabLoader<'_, TowerPrefab>| {
        loader.load(path, RonFormat, progress_counter)
    })
}
