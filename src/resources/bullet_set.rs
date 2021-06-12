use crate::prefabs::{load_bullet, BulletPrefab};
use amethyst::{
    assets::{Handle, Prefab, ProgressCounter},
    prelude::World,
};
use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum BulletType {
    Standard,
}

#[derive(Default)]
pub struct BulletPrefabSet {
    prefab_handles: HashMap<BulletType, Handle<Prefab<BulletPrefab>>>,
}

impl BulletPrefabSet {
    pub fn new(world: &mut World, progress_counter: &mut ProgressCounter) -> Self {
        // This iterator is only here to ensure that any time new item types are added, the match
        // will cause a compiler error, and this array should get caught more easily
        let prefab_handles = [BulletType::Standard]
            .iter()
            .map(|bullet_type| {
                (
                    *bullet_type,
                    // Remember to also update the array above!
                    match bullet_type {
                        BulletType::Standard => {
                            load_bullet(world, "prefabs/bullet.ron", progress_counter)
                        }
                    },
                )
            })
            .collect();

        BulletPrefabSet { prefab_handles }
    }

    pub fn get_handle(
        &self,
        bullet_type: BulletType,
    ) -> anyhow::Result<Handle<Prefab<BulletPrefab>>> {
        Ok(self
            .prefab_handles
            .get(&bullet_type)
            .ok_or_else(|| {
                anyhow::anyhow!("Prefab for object type {:?} was not loaded.", bullet_type)
            })?
            .clone())
    }
}
