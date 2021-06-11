use crate::animation::AnimationId;
use amethyst::{
    animation::AnimationSetPrefab,
    assets::{PrefabData, PrefabLoader, ProgressCounter, RonFormat},
    derive::PrefabData,
    ecs::prelude::Entity,
    error::Error,
    prelude::{Builder, World, WorldExt},
    renderer::sprite::prefab::SpriteScenePrefab,
    renderer::sprite::SpriteRender,
};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, PrefabData)]
pub struct TowerPrefab {
    sprite_scene: SpriteScenePrefab,
    animation_set: AnimationSetPrefab<AnimationId, SpriteRender>,
}

pub fn load_tower(world: &mut World, progress_counter: &mut ProgressCounter) {
    let prefab = world.exec(|loader: PrefabLoader<'_, TowerPrefab>| {
        loader.load("prefabs/tower.ron", RonFormat, progress_counter)
    });
    world.create_entity().with(prefab).build();
}
