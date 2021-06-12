use crate::{animation::AnimationId, components::Splash, prefabs, resources, state::Game};
use amethyst::{
    animation::{
        get_animation_set, AnimationCommand, AnimationControlSet, AnimationSet, EndControl,
    },
    assets::{Handle, Prefab, ProgressCounter},
    core::transform::Transform,
    ecs::{Entities, Entity, Join, ReadStorage, WriteStorage},
    input::{is_close_requested, is_key_down, VirtualKeyCode},
    prelude::{Builder, World, WorldExt},
    renderer::{camera::Camera, sprite::SpriteRender},
    window::ScreenDimensions,
    GameData, SimpleState, SimpleTrans, StateData, StateEvent, Trans,
};
use log::info;
use std::time::{Duration, Instant};

#[derive(Default)]
pub struct Loading {
    loading_progress_counter: Option<ProgressCounter>,
    main_progress_counter: Option<ProgressCounter>,
    items_done_last: Option<usize>,
    tower_prefab: Option<Handle<Prefab<prefabs::TowerPrefab>>>,
    background_prefab: Option<Handle<Prefab<prefabs::BackgroundPrefab>>>,
    player_prefab: Option<Handle<Prefab<prefabs::PlayerPrefab>>>,
    counter_end: Option<Instant>,
    animation_entity: Option<Entity>,
}

impl SimpleState for Loading {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { world, .. } = data;
        self.loading_progress_counter = Some(ProgressCounter::new());
        self.animation_entity = Some(prefabs::load_splash_screen(
            world,
            self.loading_progress_counter.as_mut().unwrap(),
        ));
        // Creates a new camera (needed for splash screen)
        initialise_camera(world);
    }

    fn handle_event(
        &mut self,
        mut _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Quit;
            }
        }
        Trans::None
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        let StateData { world, .. } = data;

        // Checks if we are still loading data
        if let Some(ref loading_progress_counter) = self.loading_progress_counter {
            // Checks progress
            if loading_progress_counter.is_complete() {
                info!("Splash screen loaded");

                // Load and start splash screen animations
                self.start_animation(world);

                self.loading_progress_counter = None;
                self.main_progress_counter = Some(ProgressCounter::new());

                // Start up all the important resource loading
                let bullet_prefab_set = resources::BulletPrefabSet::new(
                    world,
                    self.main_progress_counter.as_mut().unwrap(),
                );
                world.insert(bullet_prefab_set);
                self.tower_prefab = Some(prefabs::load_tower(
                    world,
                    self.main_progress_counter.as_mut().unwrap(),
                ));
                self.background_prefab = Some(prefabs::load_background(
                    world,
                    self.main_progress_counter.as_mut().unwrap(),
                ));
                self.player_prefab = Some(prefabs::load_player(
                    world,
                    self.main_progress_counter.as_mut().unwrap(),
                ));
            }
        } else if let Some(ref main_progress_counter) = self.main_progress_counter {
            // Checks progress
            if main_progress_counter.is_complete() {
                info!("LOADED");

                // All data loaded
                self.main_progress_counter = None;
            } else {
                let errors = main_progress_counter.errors();
                if !errors.is_empty() {
                    println!("ERRORS: {:?}", errors);
                }

                let num_finished = main_progress_counter.num_finished();
                let print = match self.items_done_last {
                    Some(l) => num_finished != l,
                    None => true,
                };
                if print {
                    self.items_done_last = Some(num_finished);
                    let completion_pct =
                        100. * num_finished as f64 / main_progress_counter.num_assets() as f64;
                    info!("{:.2}% DONE", completion_pct,);
                }
            }
        } else {
            // If animation ended after loading, exit
            if let Some(counter_end) = self.counter_end {
                if counter_end < Instant::now() {
                    if let Some(animation_entity) = self.animation_entity {
                        world
                            .delete_entity(animation_entity)
                            .expect("Failed to delete splash screen");
                    }
                    return Trans::Replace(Box::new(Game {
                        tower_prefab: self.tower_prefab.as_ref().unwrap().clone(),
                        background_prefab: self.background_prefab.as_ref().unwrap().clone(),
                        player_prefab: self.player_prefab.as_ref().unwrap().clone(),
                    }));
                }
            }
        }

        Trans::None
    }
}

impl Loading {
    fn start_animation(&mut self, world: &mut World) {
        // Execute a pass similar to a system
        world.exec(
            #[allow(clippy::type_complexity)]
            |(entities, animation_sets, splashes, mut control_sets): (
                Entities,
                ReadStorage<AnimationSet<AnimationId, SpriteRender>>,
                ReadStorage<Splash>,
                WriteStorage<AnimationControlSet<AnimationId, SpriteRender>>,
            )| {
                // Should only be one entity (splash screen)
                for (entity, animation_set, _) in (&entities, &animation_sets, &splashes).join() {
                    info!("AAAAA");
                    // Creates a new AnimationControlSet for the entity
                    let control_set = get_animation_set(&mut control_sets, entity).unwrap();
                    control_set.add_animation(
                        AnimationId::Splash,
                        &animation_set.get(&AnimationId::Splash).unwrap(),
                        EndControl::Stay,
                        1.0,
                        AnimationCommand::Start,
                    );
                }
            },
        );

        // Animation lasts ~1.54s, so let's round up to 2s
        self.counter_end = Some(Instant::now() + Duration::from_secs_f32(4.0));
    }
}

fn initialise_camera(world: &mut World) {
    info!("Initialising camera");
    let (width, height) = {
        let dim = world.read_resource::<ScreenDimensions>();
        (dim.width(), dim.height())
    };

    let mut camera_transform = Transform::default();
    camera_transform.set_translation_z(1.0);

    world
        .create_entity()
        .with(camera_transform)
        .with(Camera::standard_2d(width / 4., height / 4.))
        .build();
}
