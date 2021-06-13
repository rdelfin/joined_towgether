use crate::{prefabs, resources::FollowedObject};
use amethyst::{
    assets::{Handle, Prefab},
    input::{is_close_requested, is_key_down, VirtualKeyCode},
    prelude::{Builder, WorldExt},
    GameData, SimpleState, SimpleTrans, StateData, StateEvent, Trans,
};

pub struct Game {
    pub tower_prefab: Handle<Prefab<prefabs::TowerPrefab>>,
    pub background_prefab: Handle<Prefab<prefabs::BackgroundPrefab>>,
    pub player_prefab: Handle<Prefab<prefabs::PlayerPrefab>>,
}

impl SimpleState for Game {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { world, .. } = data;

        // Add prefabs based on what was loaded in the loading state
        let player_entity = world
            .create_entity()
            .with(self.player_prefab.clone())
            .build();
        world
            .create_entity()
            .with(self.background_prefab.clone())
            .build();

        world.insert(FollowedObject {
            e: player_entity,
            hard_lock: false,
        })
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
}
