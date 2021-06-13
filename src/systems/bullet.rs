use crate::components::{Bullet, Velocity};
use amethyst::{
    core::Transform,
    derive::SystemDesc,
    ecs::{prelude::*, Entities, ReadStorage, System, WriteStorage},
};
use nalgebra::Point2;

#[derive(SystemDesc)]
pub struct BulletSpeedSystem;

impl<'s> System<'s> for BulletSpeedSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Transform>,
        WriteStorage<'s, Velocity>,
        ReadStorage<'s, Bullet>,
    );

    fn run(&mut self, (entities, transforms, mut velocities, bullets): Self::SystemData) {
        for (entity, transform, velocity, bullet) in
            (&entities, &transforms, &mut velocities, &bullets).join()
        {
            velocity.v = velocity.v.normalize() * bullet.speed;

            let position = Point2::new(transform.translation().x, transform.translation().y);

            // At a distance of 1000, we've gone waaaay out of the screen. Delet
            if (position - Point2::new(0., 0.)).norm() > 1000. {
                entities.delete(entity).expect("Issue deleting bullet");
            }
        }
    }
}
