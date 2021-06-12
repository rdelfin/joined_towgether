use crate::{
    components::Tower,
    input::{ActionBinding, GameBindingTypes},
    prefabs::BulletPrefab,
    resources::{BulletPrefabSet, BulletType},
};
use amethyst::{
    assets::{Handle, Prefab},
    core::{geometry::Plane, Transform},
    derive::SystemDesc,
    ecs::{prelude::*, Entities, Read, System, WriteStorage},
    input::InputHandler,
    renderer::{ActiveCamera, Camera},
    window::ScreenDimensions,
};
use log::info;
use nalgebra::{Point2, Vector2};

#[derive(Default, SystemDesc)]
pub struct ShooterControlSystem {
    fire_was_pressed: bool,
}

impl<'s> System<'s> for ShooterControlSystem {
    type SystemData = (
        Entities<'s>,
        Read<'s, InputHandler<GameBindingTypes>>,
        ReadStorage<'s, Transform>,
        WriteStorage<'s, Tower>,
        WriteStorage<'s, Handle<Prefab<BulletPrefab>>>,
        Read<'s, BulletPrefabSet>,
        ReadStorage<'s, Camera>,
        Read<'s, ActiveCamera>,
        ReadExpect<'s, ScreenDimensions>,
    );

    fn run(
        &mut self,
        (
            entities,
            input,
            transforms,
            mut towers,
            mut bullet_prefabs,
            bullet_prefab_set,
            cameras,
            active_camera,
            screen_dimensions,
        ): Self::SystemData,
    ) {
        self.point_routine(
            &entities,
            &input,
            &transforms,
            &mut towers,
            &cameras,
            &active_camera,
            &screen_dimensions,
        );
        self.fire_routine(&entities, &input, &mut bullet_prefabs, &bullet_prefab_set);
    }
}

impl ShooterControlSystem {
    fn point_routine<'s>(
        &mut self,
        entities: &Entities<'s>,
        input: &Read<'s, InputHandler<GameBindingTypes>>,
        transforms: &ReadStorage<'s, Transform>,
        towers: &mut WriteStorage<'s, Tower>,
        cameras: &ReadStorage<'s, Camera>,
        active_camera: &Read<'s, ActiveCamera>,
        screen_dimensions: &ReadExpect<'s, ScreenDimensions>,
    ) {
        let mouse = match self.get_mouse_projection(
            entities,
            input,
            transforms,
            cameras,
            active_camera,
            screen_dimensions,
        ) {
            Some(m) => m,
            None => return,
        };

        for (tower, transform) in (towers, transforms).join() {
            let tower_position = Point2::new(transform.translation().x, transform.translation().y);
            let dir = (mouse - tower_position).normalize();
            tower.dir = dir;
            info!("Pointing to {:?}", dir);
        }
    }

    fn fire_routine<'s>(
        &mut self,
        entities: &Entities<'s>,
        input: &Read<'s, InputHandler<GameBindingTypes>>,
        bullet_prefabs: &mut WriteStorage<'s, Handle<Prefab<BulletPrefab>>>,
        bullet_prefab_set: &Read<'s, BulletPrefabSet>,
    ) {
        let fire_is_pressed = input.action_is_down(&ActionBinding::Fire).unwrap_or(false);

        if !fire_is_pressed && self.fire_was_pressed {
            bullet_prefab_set
                .add_bullet(BulletType::Standard, entities, bullet_prefabs)
                .expect("Failed to add bullet");
            info!("PEW");
        }

        self.fire_was_pressed = fire_is_pressed;
    }

    fn get_mouse_projection<'s>(
        &self,
        entities: &Entities<'s>,
        input: &Read<'s, InputHandler<GameBindingTypes>>,
        transforms: &ReadStorage<'s, Transform>,
        cameras: &ReadStorage<'s, Camera>,
        active_camera: &Read<'s, ActiveCamera>,
        screen_dimensions: &ReadExpect<'s, ScreenDimensions>,
    ) -> Option<Point2<f32>> {
        let mouse = match input.mouse_position() {
            Some((x, y)) => Point2::new(x, y),
            None => Point2::new(0.0, 0.0),
        };
        let mut camera_join = (cameras, transforms).join();

        match active_camera
            .entity
            .and_then(|a| camera_join.get(a, &entities))
            .or_else(|| camera_join.next())
        {
            Some((camera, camera_transform)) => {
                let ray = camera.screen_ray(
                    mouse,
                    Vector2::new(screen_dimensions.width(), screen_dimensions.height()),
                    camera_transform,
                );
                let distance = ray.intersect_plane(&Plane::with_z(0.0)).unwrap();
                let point_intersection = ray.at_distance(distance);
                Some(Point2::new(point_intersection.x, point_intersection.y))
            }
            None => None,
        }
    }
}
