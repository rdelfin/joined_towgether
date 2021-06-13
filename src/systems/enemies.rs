use crate::{
    components::{Guided, Velocity},
    prefabs::EnemyPrefab,
    resources::{EnemyPrefabSet, EnemySpawning, EnemyType},
};
use amethyst::{
    assets::{Handle, Prefab},
    core::Transform,
    derive::SystemDesc,
    ecs::{prelude::*, Entities, Read, ReadStorage, System, WriteStorage},
};
use log::info;
use nalgebra::Point2;
use std::time::{Duration, Instant};

#[derive(Default, SystemDesc)]
pub struct EnemySpawnSystem {
    last_enemy_spawn: Option<Instant>,
}

impl<'s> System<'s> for EnemySpawnSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Handle<Prefab<EnemyPrefab>>>,
        Read<'s, EnemyPrefabSet>,
        Option<Read<'s, EnemySpawning>>,
    );

    fn run(
        &mut self,
        (entities, mut enemy_prefabs, enemy_prefab_set, enemy_spawning): Self::SystemData,
    ) {
        // Only spawn enemies once this exists
        if let Some(_) = enemy_spawning {
            let spawn_time = match self.last_enemy_spawn {
                Some(i) => i.elapsed() > Duration::from_secs_f32(2.0),
                None => true,
            };

            if spawn_time {
                info!("Spawn!");
                enemy_prefab_set
                    .add_enemy(EnemyType::Standard, &entities, &mut enemy_prefabs)
                    .expect("There was an issue spawning an enemy");
                self.last_enemy_spawn = Some(Instant::now());
            }
        }
    }
}

#[derive(SystemDesc)]
pub struct EnemyMovementSystem;

impl<'s> System<'s> for EnemyMovementSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Velocity>,
        WriteStorage<'s, Guided>,
        ReadStorage<'s, Transform>,
    );

    fn run(&mut self, (entities, mut velocities, mut guided, transforms): Self::SystemData) {
        for (entity, velocity, guided, transform) in
            (&entities, &mut velocities, &mut guided, &transforms).join()
        {
            let position = Point2::new(transform.translation().x, transform.translation().y);
            // You are within reasonable distance of the waypoint, switch waypoints
            if (guided.waypoints[guided.curr_waypoint] - position).norm() < guided.speed / 10. {
                guided.curr_waypoint += 1;
            }

            // If you've reached the end, delet the enemy and skip all other operations here
            if guided.curr_waypoint >= guided.waypoints.len() {
                entities.delete(entity).expect("Issue deleting enemy");
                continue;
            }

            // Just in case, if you're on top of the waypoint, just skip this iteration
            // and keep on moving in the direction you were going
            if position == guided.waypoints[guided.curr_waypoint] {
                continue;
            }

            let dir = (guided.waypoints[guided.curr_waypoint] - position).normalize();
            velocity.v = dir * guided.speed;
        }
    }
}
