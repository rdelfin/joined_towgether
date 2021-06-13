use crate::{resources::FollowedObject, util};
use amethyst::{
    core::Transform,
    derive::SystemDesc,
    ecs::{prelude::*, Entities, ReadStorage, System, WriteStorage},
    renderer::{ActiveCamera, Camera},
};
use nalgebra::{Point2, Vector2};

#[derive(SystemDesc)]
pub struct CameraFollowSystem;

impl<'s> System<'s> for CameraFollowSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Camera>,
        Option<Read<'s, FollowedObject>>,
        Read<'s, ActiveCamera>,
    );

    fn run(
        &mut self,
        (entities, mut transforms, cameras, followed_object, active_camera): Self::SystemData,
    ) {
        // Only continue if we have an active camera and a followed object with a transform
        let followed_object = match followed_object {
            Some(f) => f,
            None => {
                return;
            }
        };
        let followed_position = {
            let translation = match transforms.get(followed_object.e) {
                Some(t) => t,
                None => {
                    return;
                }
            }
            .translation();
            Point2::new(translation.x, translation.y)
        };
        let mut camera_join = (&cameras, &mut transforms).join();
        let camera_transform = match active_camera
            .entity
            .and_then(|a| camera_join.get(a, &entities))
            .or_else(|| camera_join.next())
        {
            Some((_, t)) => t,
            None => {
                return;
            }
        };

        // Extract transforms out and calculate where to move the camera to
        let mut_camera_transform = camera_transform.translation_mut();
        let mut camera_position = Point2::new(mut_camera_transform.x, mut_camera_transform.y);
        camera_position = self.closest_ideal_position(
            camera_position,
            followed_position,
            followed_object.hard_lock,
        );
        mut_camera_transform.x = camera_position.x;
        mut_camera_transform.y = camera_position.y;
    }
}

impl CameraFollowSystem {
    fn closest_ideal_position(
        &self,
        camera_position: Point2<f32>,
        followed_position: Point2<f32>,
        hard_lock: bool,
    ) -> Point2<f32> {
        if hard_lock {
            followed_position
        } else {
            // If you're over the camera, we'd find no direction vector. Keep as is
            if camera_position == followed_position {
                return camera_position;
            }

            let dir = camera_position - followed_position;
            // There's 4 lines our segment can intersect with:
            // y = followed_position.y + 68
            // y = followed_position.y - 68
            // x = followed_position.x + 120
            // x = followed_position.x - 120
            //
            // We're looking for the line with the intersection closest to the player, in the
            // direction of the camera, only including ones between the camera and the person (to
            // avoid moving the camera away from the player). If none is found, we're already in a
            // suitable position and we can keep the camera as is
            [
                (followed_position.y + 68., false),
                (followed_position.y - 68., false),
                (followed_position.x + 120., true),
                (followed_position.x - 120., true),
            ]
            .iter()
            .filter_map(|(c, vert)| util::intersect(followed_position, dir, *c, *vert))
            .filter(|(t, _)| *t >= 0. && *t <= 1.)
            .min_by(|(a, _), (b, _)| a.partial_cmp(b).expect("Tried to compare a NaN"))
            .map(|(_, intersection)| intersection)
            .unwrap_or(camera_position)
        }
    }
}
