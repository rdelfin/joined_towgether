use crate::{animation::AnimationId, components::Splash};
use amethyst::{
    animation::AnimationSetPrefab,
    assets::{PrefabData, PrefabLoader, ProgressCounter, RonFormat},
    derive::PrefabData,
    ecs::prelude::Entity,
    error::Error,
    prelude::{Builder, World, WorldExt},
    renderer::sprite::{prefab::SpriteScenePrefab, SpriteRender},
};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, PrefabData)]
pub struct SplashAnimationPrefab {
    sprite_scene: SpriteScenePrefab,
    animation_set: AnimationSetPrefab<AnimationId, SpriteRender>,
    splash: Splash,
}

pub fn load_splash_screen(world: &mut World, progress_counter: &mut ProgressCounter) -> Entity {
    let prefab = world.exec(|loader: PrefabLoader<'_, SplashAnimationPrefab>| {
        loader.load("prefabs/splash.ron", RonFormat, progress_counter)
    });
    world.create_entity().with(prefab).build()
}
