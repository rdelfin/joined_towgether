use crate::prefabs::{load_tower, TowerPrefab};
use amethyst::{
    assets::{Handle, Prefab, ProgressCounter},
    core::Transform,
    ecs::{Entities, WriteStorage},
    prelude::World,
};
use nalgebra::Point2;
use std::collections::HashMap;

pub struct TowerPlacement {
    pub placing: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum TowerType {
    Standard,
}

#[derive(Default)]
pub struct TowerPrefabSet {
    prefab_handles: HashMap<TowerType, Handle<Prefab<TowerPrefab>>>,
}

impl TowerPrefabSet {
    pub fn new(world: &mut World, progress_counter: &mut ProgressCounter) -> Self {
        // This iterator is only here to ensure that any time new item types are added, the match
        // will cause a compiler error, and this array should get caught more easily
        let prefab_handles = [TowerType::Standard]
            .iter()
            .map(|bullet_type| {
                (
                    *bullet_type,
                    // Remember to also update the array above!
                    match bullet_type {
                        TowerType::Standard => {
                            load_tower(world, "prefabs/tower.ron", progress_counter)
                        }
                    },
                )
            })
            .collect();

        TowerPrefabSet { prefab_handles }
    }

    pub fn get_handle(&self, tower_type: TowerType) -> anyhow::Result<Handle<Prefab<TowerPrefab>>> {
        Ok(self
            .prefab_handles
            .get(&tower_type)
            .ok_or_else(|| {
                anyhow::anyhow!("Prefab for tower type {:?} was not loaded.", tower_type)
            })?
            .clone())
    }

    pub fn add_tower<'s>(
        &self,
        tower_type: TowerType,
        position: Point2<f32>,
        entities: &Entities<'s>,
        tower_prefabs: &mut WriteStorage<'s, Handle<Prefab<TowerPrefab>>>,
        transforms: &mut WriteStorage<'s, Transform>,
    ) -> anyhow::Result<()> {
        let tower_prefab = self.get_handle(tower_type)?;
        let mut transform = Transform::default();
        transform.set_translation_xyz(position.x, position.y, 0.3);
        entities
            .build_entity()
            .with(tower_prefab, tower_prefabs)
            .with(transform, transforms)
            .build();

        Ok(())
    }
}
