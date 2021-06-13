use crate::{
    input::{self, ActionBinding, GameBindingTypes},
    prefabs::TowerPrefab,
    resources::{TowerPlacement, TowerPrefabSet, TowerType},
};
use amethyst::{
    assets::{Handle, Prefab},
    core::Transform,
    derive::SystemDesc,
    ecs::{Entities, Read, ReadExpect, ReadStorage, System, SystemData, Write, WriteStorage},
    input::InputHandler,
    renderer::{ActiveCamera, Camera},
    shrev::{EventChannel, ReaderId},
    ui::{UiEvent, UiEventType, UiFinder},
    window::ScreenDimensions,
};

const BUTTON_TOWER: &str = "button";

#[derive(SystemDesc)]
#[system_desc(name(UiEventHandlerSystemDesc))]
pub struct UiEventHandlerSystem {
    #[system_desc(event_channel_reader)]
    reader_id: ReaderId<UiEvent>,
}

impl UiEventHandlerSystem {
    pub fn new(reader_id: ReaderId<UiEvent>) -> Self {
        Self { reader_id }
    }
}

impl<'s> System<'s> for UiEventHandlerSystem {
    type SystemData = (
        Write<'s, EventChannel<UiEvent>>,
        UiFinder<'s>,
        Option<Write<'s, TowerPlacement>>,
    );

    fn run(&mut self, (events, ui_finder, tower_placement): Self::SystemData) {
        match tower_placement {
            Some(mut tower_placement) => {
                for ev in events.read(&mut self.reader_id) {
                    // Look for tower clicks
                    if ev.event_type == UiEventType::Click {
                        let button_entity = ui_finder.find(BUTTON_TOWER);

                        // If the entity pressed is the tower button, start placing
                        if let Some(button_entity) = button_entity {
                            if button_entity == ev.target {
                                tower_placement.placing = true;
                            }
                        }
                    }
                }
            }
            None => {
                // Read until emptied out. We want to ignore old events
                for _ in events.read(&mut self.reader_id) {}
            }
        }
    }
}

#[derive(Default, SystemDesc)]
pub struct PlacementSystem {
    place_was_pressed: bool,
}

impl<'s> System<'s> for PlacementSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Camera>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Handle<Prefab<TowerPrefab>>>,
        Option<Write<'s, TowerPlacement>>,
        Read<'s, TowerPrefabSet>,
        Read<'s, InputHandler<GameBindingTypes>>,
        Read<'s, ActiveCamera>,
        ReadExpect<'s, ScreenDimensions>,
    );

    fn run(
        &mut self,
        (
            entities,
            cameras,
            mut transforms,
            mut tower_prefabs,
            mut tower_placement,
            tower_prefab_set,
            input_handler,
            active_camera,
            screen_dimensions,
        ): Self::SystemData,
    ) {
        if let Some(ref mut tower_placement) = tower_placement {
            // Do not track button presses unless we're placing
            if tower_placement.placing {
                let place_is_pressed = input_handler
                    .action_is_down(&ActionBinding::Place)
                    .unwrap_or(false);
                if !place_is_pressed && self.place_was_pressed {
                    let position = match input::get_mouse_projection(
                        &entities,
                        &input_handler,
                        &transforms,
                        &cameras,
                        &active_camera,
                        &screen_dimensions,
                    ) {
                        Some(p) => p,
                        None => {
                            return;
                        }
                    };

                    tower_prefab_set
                        .add_tower(
                            TowerType::Standard,
                            position,
                            &entities,
                            &mut tower_prefabs,
                            &mut transforms,
                        )
                        .expect("Failed to add tower");
                    tower_placement.placing = false;
                }

                self.place_was_pressed = place_is_pressed;
            }
        }
    }
}
