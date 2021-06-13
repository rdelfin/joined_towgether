use amethyst::{
    assets::PrefabData,
    derive::PrefabData,
    ecs::{storage::DenseVecStorage, Component, Entity, WriteStorage},
    Error,
};
use nalgebra::Vector2;
use serde::{Deserialize, Serialize};

#[derive(Clone, Component, Debug, Deserialize, Serialize, PrefabData)]
#[prefab(Component)]
#[storage(DenseVecStorage)]
#[serde(deny_unknown_fields)]
pub struct Guided {
    pub speed: f32,
    // Waypoints are all supposed to be destinations, so the initial position should
    // not be included
    pub waypoints: Vec<Vector2<f32>>,
    pub curr_waypoint: usize,
}
