use crate::resources::TowerPlacement;
use amethyst::{
    derive::SystemDesc,
    ecs::{System, SystemData, Write},
    shrev::{EventChannel, ReaderId},
    ui::{UiEvent, UiEventType, UiFinder},
};
use log::info;

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
                                info!("PLACING!");
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
