use crate::{
    components::{Tower, TowerDirection, Velocity},
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
    renderer::{sprite::SpriteRender, ActiveCamera, Camera},
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
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Tower>,
        WriteStorage<'s, Velocity>,
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
            mut transforms,
            mut towers,
            mut velocities,
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
        self.fire_routine(
            &entities,
            &input,
            &mut transforms,
            &mut velocities,
            &towers,
            &mut bullet_prefabs,
            &bullet_prefab_set,
        );
    }
}

impl ShooterControlSystem {
    fn point_routine<'s>(
        &mut self,
        entities: &Entities<'s>,
        input: &Read<'s, InputHandler<GameBindingTypes>>,
        transforms: &WriteStorage<'s, Transform>,
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
            tower.sprite_dir = self.get_tower_direction(dir);
        }
    }

    fn fire_routine<'s>(
        &mut self,
        entities: &Entities<'s>,
        input: &Read<'s, InputHandler<GameBindingTypes>>,
        transforms: &mut WriteStorage<'s, Transform>,
        velocities: &mut WriteStorage<'s, Velocity>,
        towers: &WriteStorage<'s, Tower>,
        bullet_prefabs: &mut WriteStorage<'s, Handle<Prefab<BulletPrefab>>>,
        bullet_prefab_set: &Read<'s, BulletPrefabSet>,
    ) {
        let fire_is_pressed = input.action_is_down(&ActionBinding::Fire).unwrap_or(false);

        if !fire_is_pressed && self.fire_was_pressed {
            let mut tower_data: Vec<(Vector2<f32>, Vector2<f32>)> = vec![];
            for (transform, tower) in (&*transforms, towers).join() {
                let translation = transform.translation().clone();
                tower_data.push((
                    tower.dir.clone(),
                    Vector2::new(translation.x, translation.y),
                ));
            }

            for (direction, position) in tower_data {
                bullet_prefab_set
                    .add_bullet(
                        BulletType::Standard,
                        direction,
                        position,
                        entities,
                        bullet_prefabs,
                        transforms,
                        velocities,
                    )
                    .expect("Failed to add bullet");
            }
            info!("PEW");
        }

        self.fire_was_pressed = fire_is_pressed;
    }

    fn get_mouse_projection<'s>(
        &self,
        entities: &Entities<'s>,
        input: &Read<'s, InputHandler<GameBindingTypes>>,
        transforms: &WriteStorage<'s, Transform>,
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

    fn get_tower_direction(&self, dir: Vector2<f32>) -> TowerDirection {
        let angle = dir.y.atan2(dir.x);
        const PI: f32 = std::f32::consts::PI;

        if angle >= PI / 4. && angle < 3. * PI / 4. {
            TowerDirection::N
        } else if angle >= 3. * PI / 4. || angle < -3. * PI / 4. {
            TowerDirection::W
        } else if angle >= -3. * PI / 4. && angle < -PI / 4. {
            TowerDirection::S
        } else {
            TowerDirection::E
        }
    }
}

#[derive(SystemDesc)]
pub struct TowerDirectionSystem;

impl<'s> System<'s> for TowerDirectionSystem {
    type SystemData = (ReadStorage<'s, Tower>, WriteStorage<'s, SpriteRender>);

    fn run(&mut self, (towers, mut sprite_renders): Self::SystemData) {
        for (tower, sprite_render) in (&towers, &mut sprite_renders).join() {
            sprite_render.sprite_number = match tower.sprite_dir {
                TowerDirection::N => 2,
                TowerDirection::S => 3,
                TowerDirection::E => 1,
                TowerDirection::W => 0,
            }
        }
    }
}
