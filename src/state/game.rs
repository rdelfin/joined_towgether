use crate::{
    audio, prefabs,
    resources::{FollowedObject, TowerPlacement},
};
use amethyst::{
    assets::{AssetStorage, Handle, Prefab},
    audio::{output::Output, Source},
    ecs::{Entity, Read, ReadExpect},
    input::{is_close_requested, is_key_down, VirtualKeyCode},
    prelude::{Builder, WorldExt},
    ui::UiCreator,
    GameData, SimpleState, SimpleTrans, StateData, StateEvent, Trans,
};

pub struct Game {
    pub background_prefab: Handle<Prefab<prefabs::BackgroundPrefab>>,
    pub player_prefab: Handle<Prefab<prefabs::PlayerPrefab>>,
    pub ui_root: Option<Entity>,
}

impl SimpleState for Game {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { world, .. } = data;

        // Setup UI
        self.ui_root =
            Some(world.exec(|mut creator: UiCreator<'_>| creator.create("ui/hud.ron", ())));

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
        });
        world.insert(TowerPlacement { placing: false });

        // Start the music
        world.exec(
            |(sounds, storage, audio_output): (
                ReadExpect<'_, audio::Sounds>,
                Read<'_, AssetStorage<Source>>,
                Option<Read<'_, Output>>,
            )| { audio::play_music(&*sounds, &storage, audio_output.as_deref()) },
        );
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
