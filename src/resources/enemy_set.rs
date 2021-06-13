use crate::prefabs::{load_enemy, EnemyPrefab};
use amethyst::{
    assets::{Handle, Prefab, ProgressCounter},
    ecs::{Entities, WriteStorage},
    prelude::World,
};
use std::collections::HashMap;

#[derive(Default)]
pub struct EnemySpawning;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum EnemyType {
    Standard,
}

#[derive(Default)]
pub struct EnemyPrefabSet {
    prefab_handles: HashMap<EnemyType, Handle<Prefab<EnemyPrefab>>>,
}

impl EnemyPrefabSet {
    pub fn new(world: &mut World, progress_counter: &mut ProgressCounter) -> Self {
        // This iterator is only here to ensure that any time new item types are added, the match
        // will cause a compiler error, and this array should get caught more easily
        let prefab_handles = [EnemyType::Standard]
            .iter()
            .map(|enemy_type| {
                (
                    *enemy_type,
                    // Remember to also update the array above!
                    match enemy_type {
                        EnemyType::Standard => {
                            load_enemy(world, "prefabs/enemy.ron", progress_counter)
                        }
                    },
                )
            })
            .collect();

        EnemyPrefabSet { prefab_handles }
    }

    pub fn get_handle(&self, enemy_type: EnemyType) -> anyhow::Result<Handle<Prefab<EnemyPrefab>>> {
        Ok(self
            .prefab_handles
            .get(&enemy_type)
            .ok_or_else(|| {
                anyhow::anyhow!("Prefab for enemy type {:?} was not loaded.", enemy_type)
            })?
            .clone())
    }

    pub fn add_enemy<'s>(
        &self,
        enemy_type: EnemyType,
        entities: &Entities<'s>,
        enemy_prefabs: &mut WriteStorage<'s, Handle<Prefab<EnemyPrefab>>>,
    ) -> anyhow::Result<()> {
        let enemy_prefab = self.get_handle(enemy_type)?;
        entities
            .build_entity()
            .with(enemy_prefab, enemy_prefabs)
            .build();

        Ok(())
    }
}
