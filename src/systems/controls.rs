use crate::{
    components::{ControlledCharacter, Tower, TowerDirection, Velocity},
    input::{self, ActionBinding, AxisBinding, GameBindingTypes},
    prefabs::BulletPrefab,
    resources::{BulletPrefabSet, BulletType, FollowedObject},
};
use amethyst::{
    assets::{Handle, Prefab},
    core::Transform,
    derive::SystemDesc,
    ecs::{prelude::*, Entities, Read, ReadExpect, ReadStorage, System, WriteStorage},
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
        let mouse = match input::get_mouse_projection(
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
            if tower.active {
                let tower_position =
                    Point2::new(transform.translation().x, transform.translation().y);
                let dir = (mouse - tower_position).normalize();
                tower.dir = dir;
                tower.sprite_dir = self.get_tower_direction(dir);
            } else {
                tower.dir = Vector2::new(-1.0, 0.0);
                tower.sprite_dir = TowerDirection::W;
            }
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
                if tower.active {
                    let translation = transform.translation().clone();
                    tower_data.push((
                        tower.dir.clone(),
                        Vector2::new(translation.x, translation.y),
                    ));
                }
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

struct TowerData {
    pos: Point2<f32>,
    entity: Entity,
    active: bool,
}

#[derive(Default, SystemDesc)]
pub struct PlayerControlSystem {
    activate_was_pressed: bool,
}

impl<'s> System<'s> for PlayerControlSystem {
    type SystemData = (
        Entities<'s>,
        Read<'s, InputHandler<GameBindingTypes>>,
        WriteStorage<'s, ControlledCharacter>,
        WriteStorage<'s, Tower>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Velocity>,
        Option<Write<'s, FollowedObject>>,
    );

    fn run(
        &mut self,
        (
            entities,
            input,
            mut controlled_characters,
            mut towers,
            mut transforms,
            mut velocities,
            mut followed_object,
        ): Self::SystemData,
    ) {
        // Move according to this frame
        for (controlled_character, velocity) in (&controlled_characters, &mut velocities).join() {
            if !controlled_character.locked {
                let forwards = input.axis_value(&AxisBinding::Forwards).unwrap_or(0.0);
                let sideways = input.axis_value(&AxisBinding::Sideways).unwrap_or(0.0);
                let mut direction = Vector2::new(sideways, forwards);
                if direction.norm() != 0.0 {
                    direction = direction.normalize();
                }

                velocity.v = direction * controlled_character.speed;
            }
        }

        let activate_is_pressed = input
            .action_is_down(&ActionBinding::Activate)
            .unwrap_or(false);

        // Try to enter if requested
        if !activate_is_pressed && self.activate_was_pressed {
            // Gets all the data we need to work with the tower
            let tower_data: Vec<_> = (&entities, &towers, &transforms)
                .join()
                .map(|(entity, tower, transform)| TowerData {
                    pos: Point2::new(transform.translation().x, transform.translation().y),
                    entity: entity,
                    active: tower.active,
                })
                .collect();

            for (entity, controlled_character, velocity, transform) in (
                &entities,
                &mut controlled_characters,
                &mut velocities,
                &mut transforms,
            )
                .join()
            {
                let character_position =
                    Point2::new(transform.translation().x, transform.translation().y);
                // If the character is locked, find the corresponding active tower and disable +
                // exit
                if controlled_character.locked {
                    // Check within 1 unit as we should be right on top of it
                    let closest_tower = match tower_data
                        .iter()
                        .filter(|td| td.active && (td.pos - character_position).norm() <= 1.)
                        .min_by(|a, b| {
                            let a_dist = (a.pos - character_position).norm();
                            let b_dist = (b.pos - character_position).norm();
                            a_dist.partial_cmp(&b_dist).expect("Tried to compare a NaN")
                        }) {
                        Some(t) => t,
                        None => {
                            continue;
                        }
                    };

                    towers
                        .get_mut(closest_tower.entity)
                        .expect("Tower dissapeared")
                        .active = false;
                    controlled_character.locked = false;
                    transform.set_translation_xyz(closest_tower.pos.x, closest_tower.pos.y, 0.6);
                }
                // If the character is not locked, check if there's any inactive nearby towers
                // (within (within 60. units) and enter them
                else {
                    let closest_tower = match tower_data
                        .iter()
                        .filter(|td| !td.active && (td.pos - character_position).norm() <= 60.)
                        .min_by(|a, b| {
                            let a_dist = (a.pos - character_position).norm();
                            let b_dist = (b.pos - character_position).norm();
                            a_dist.partial_cmp(&b_dist).expect("Tried to compare a NaN")
                        }) {
                        Some(t) => t,
                        None => {
                            continue;
                        }
                    };

                    towers
                        .get_mut(closest_tower.entity)
                        .expect("Tower dissapeared")
                        .active = true;
                    controlled_character.locked = true;
                    // Send behind the background so we don't see the character
                    transform.set_translation_xyz(closest_tower.pos.x, closest_tower.pos.y, -10.);
                    velocity.v = Vector2::new(0.0, 0.0);
                    // Lock in the camera if it's a followed object
                    if let Some(ref mut followed_object) = followed_object {
                        if followed_object.e == entity {
                            followed_object.hard_lock = true;
                        }
                    }
                }
            }
        }

        self.activate_was_pressed = activate_is_pressed;
    }
}
