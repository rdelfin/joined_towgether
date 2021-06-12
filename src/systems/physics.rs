use crate::components::Velocity;
use amethyst::{
    core::{Time, Transform},
    derive::SystemDesc,
    ecs::{prelude::*, ReadStorage, System, WriteStorage},
};

#[derive(SystemDesc)]
pub struct PhysicsSystem;

impl<'s> System<'s> for PhysicsSystem {
    type SystemData = (
        ReadStorage<'s, Velocity>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, (velocities, mut transforms, time): Self::SystemData) {
        let frame_delta_s = time.fixed_time().as_secs_f32();
        for (velocity, transform) in (&velocities, &mut transforms).join() {
            let position = transform.translation_mut();
            let diff = velocity.v * frame_delta_s;
            position.x += diff.x;
            position.y += diff.y;
        }
    }
}
