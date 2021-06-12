use crate::components::{Bullet, Velocity};
use amethyst::{
    derive::SystemDesc,
    ecs::{prelude::*, ReadStorage, System, WriteStorage},
};

#[derive(SystemDesc)]
pub struct BulletSpeedSystem;

impl<'s> System<'s> for BulletSpeedSystem {
    type SystemData = (WriteStorage<'s, Velocity>, ReadStorage<'s, Bullet>);

    fn run(&mut self, (mut velocities, bullets): Self::SystemData) {
        for (velocity, bullet) in (&mut velocities, &bullets).join() {
            velocity.v = velocity.v.normalize() * bullet.speed;
        }
    }
}
