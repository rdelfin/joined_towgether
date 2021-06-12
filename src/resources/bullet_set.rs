use crate::{
    components::Velocity,
    prefabs::{load_bullet, BulletPrefab},
};
use amethyst::{
    assets::{Handle, Prefab, ProgressCounter},
    core::Transform,
    ecs::{Entities, WriteStorage},
    prelude::World,
};
use nalgebra::{Translation3, Unit, UnitQuaternion, Vector2, Vector3};
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

    pub fn add_bullet<'s>(
        &self,
        bullet_type: BulletType,
        dir: Vector2<f32>,
        position: Vector2<f32>,
        entities: &Entities<'s>,
        bullet_prefabs: &mut WriteStorage<'s, Handle<Prefab<BulletPrefab>>>,
        transforms: &mut WriteStorage<'s, Transform>,
        velocities: &mut WriteStorage<'s, Velocity>,
    ) -> anyhow::Result<()> {
        let bullet_prefab = self.get_handle(bullet_type)?;
        entities
            .build_entity()
            .with(bullet_prefab, bullet_prefabs)
            .with(
                Transform::new(
                    Translation3::new(position.x, position.y + 10., 0.2),
                    UnitQuaternion::from_axis_angle(
                        &Unit::new_normalize(Vector3::new(0.0, 0.0, 1.0)),
                        dir.y.atan2(dir.x),
                    ),
                    Vector3::new(1.0, 1.0, 1.0),
                ),
                transforms,
            )
            .with(Velocity { v: dir }, velocities)
            .build();

        Ok(())
    }
}
