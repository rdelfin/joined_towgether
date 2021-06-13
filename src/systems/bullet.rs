use crate::{
    components::{Bullet, Hitable, Velocity},
    util,
};
use amethyst::{
    core::Transform,
    derive::SystemDesc,
    ecs::{prelude::*, Entities, ReadStorage, System, WriteStorage},
};
use nalgebra::Point2;

#[derive(SystemDesc)]
pub struct BulletSystem;

impl<'s> System<'s> for BulletSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Transform>,
        WriteStorage<'s, Velocity>,
        WriteStorage<'s, Bullet>,
        WriteStorage<'s, Hitable>,
    );

    fn run(
        &mut self,
        (entities, transforms, mut velocities, mut bullets, mut hitable): Self::SystemData,
    ) {
        let hitables: Vec<_> = (&entities, &transforms, &hitable)
            .join()
            .map(|(entity, transform, _)| {
                (
                    entity,
                    Point2::new(transform.translation().x, transform.translation().y),
                )
            })
            .collect();

        for (bullet_entity, transform, velocity, bullet) in
            (&entities, &transforms, &mut velocities, &mut bullets).join()
        {
            let bullet_position = Point2::new(transform.translation().x, transform.translation().y);
            // First, calculate hits, only if velocity.norm() is not 0
            if velocity.v.norm() != 0. {
                if let Some(last_position) = bullet.last_position {
                    // We now have a parametric eqn; b(t) = last_position + t * velocity.v
                    let mut deleted_bullet = false;
                    for (hitable_entity, hitable_position) in &hitables {
                        if util::rectangle_line_intersect(
                            last_position,
                            bullet_position - last_position,
                            util::Rect {
                                x: hitable_position.x - 16.,
                                y: hitable_position.y - 16.,
                                w: 32.,
                                h: 32.,
                            },
                        ) {
                            if let Some(ref mut hitable) = hitable.get_mut(*hitable_entity) {
                                hitable.health -= bullet.hitpoints;
                                entities
                                    .delete(bullet_entity)
                                    .expect("Issue deleting bullet");
                                if hitable.health <= 0. {
                                    entities
                                        .delete(*hitable_entity)
                                        .expect("Issue deleting enemy from bullet");
                                }
                                deleted_bullet = true;
                                break;
                            }
                        }
                    }

                    if deleted_bullet {
                        continue;
                    }
                }
            }

            // Second, move bullet and calculate if we're out of bounds
            velocity.v = velocity.v.normalize() * bullet.speed;

            let position = Point2::new(transform.translation().x, transform.translation().y);

            // At a distance of 1000, we've gone waaaay out of the screen. Delet
            if (position - Point2::new(0., 0.)).norm() > 1000. {
                entities
                    .delete(bullet_entity)
                    .expect("Issue deleting bullet");
            }

            bullet.last_position = Some(position);
        }
    }
}
