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
pub struct BackgroundPrefab {
    sprite_scene: SpriteScenePrefab,
}

pub fn load_background(
    world: &mut World,
    progress_counter: &mut ProgressCounter,
) -> Handle<Prefab<BackgroundPrefab>> {
    world.exec(|loader: PrefabLoader<'_, BackgroundPrefab>| {
        loader.load("prefabs/background.ron", RonFormat, progress_counter)
    })
}
