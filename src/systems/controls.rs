use crate::{
    animation::AnimationId,
    components::Tower,
    input::{ActionBinding, GameBindingTypes},
    prefabs::BulletPrefab,
    resources::{BulletPrefabSet, BulletType},
};
use amethyst::{
    animation::{get_animation_set, AnimationControlSet},
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
        ReadStorage<'s, Transform>,
        WriteStorage<'s, Tower>,
        WriteStorage<'s, AnimationControlSet<AnimationId, SpriteRender>>,
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
            mut control_sets,
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
            &mut control_sets,
            &cameras,
            &active_camera,
            &screen_dimensions,
        );
        self.fire_routine(
            &entities,
            &input,
            &mut towers,
            &mut control_sets,
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
        transforms: &ReadStorage<'s, Transform>,
        towers: &mut WriteStorage<'s, Tower>,
        control_sets: &mut WriteStorage<'s, AnimationControlSet<AnimationId, SpriteRender>>,
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

        for (entity, tower, transform) in (entities, towers, transforms).join() {
            let tower_position = Point2::new(transform.translation().x, transform.translation().y);
            let dir = (mouse - tower_position).normalize();
            tower.dir = dir;

            let control_set = get_animation_set(control_sets, entity).unwrap();
            let direction = self.get_tower_direction(dir);
            control_set.pause(AnimationId::TowerUp);
            control_set.pause(AnimationId::TowerDown);
            control_set.pause(AnimationId::TowerLeft);
            control_set.pause(AnimationId::TowerRight);
            control_set.set_input(direction, 0.0);
            control_set.start(direction);

            info!("Pointing to {:?} ({:?})", dir, direction);
        }
    }

    fn fire_routine<'s>(
        &mut self,
        entities: &Entities<'s>,
        input: &Read<'s, InputHandler<GameBindingTypes>>,
        towers: &mut WriteStorage<'s, Tower>,
        control_sets: &mut WriteStorage<'s, AnimationControlSet<AnimationId, SpriteRender>>,
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

    fn get_tower_direction(&self, dir: Vector2<f32>) -> AnimationId {
        let angle = dir.y.atan2(dir.x);
        const PI: f32 = std::f32::consts::PI;

        println!("Angle: {}", angle);

        if angle >= PI / 4. && angle < 3. * PI / 4. {
            AnimationId::TowerUp
        } else if angle >= 3. * PI / 4. || angle < -3. * PI / 4. {
            AnimationId::TowerLeft
        } else if angle >= -3. * PI / 4. && angle < -PI / 4. {
            AnimationId::TowerDown
        } else {
            AnimationId::TowerRight
        }
    }
}
