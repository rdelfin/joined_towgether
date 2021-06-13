use amethyst::{
    assets::PrefabData,
    derive::PrefabData,
    ecs::{storage::DenseVecStorage, Component, Entity, WriteStorage},
    Error,
};
use nalgebra::Vector2;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum TowerDirection {
    N,
    S,
    E,
    W,
}

#[derive(Clone, Component, Debug, Deserialize, Serialize, PrefabData)]
#[prefab(Component)]
#[storage(DenseVecStorage)]
#[serde(deny_unknown_fields)]
pub struct Tower {
    pub dir: Vector2<f32>,
    pub sprite_dir: TowerDirection,
    pub active: bool,
}

#[derive(Clone, Component, Debug, Deserialize, Serialize, PrefabData)]
#[prefab(Component)]
#[storage(DenseVecStorage)]
#[serde(deny_unknown_fields)]
pub struct Bullet {
    pub speed: f32,
    pub hitpoints: f32,
}
