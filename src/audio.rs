use amethyst::{
    assets::{AssetStorage, Loader, ProgressCounter},
    audio::{output::Output, OggFormat, Source, SourceHandle},
    ecs::{World, WorldExt},
};

pub struct Sounds {
    pub music: SourceHandle,
    pub intro_buzz: SourceHandle,
}

/// Loads an ogg audio track.
fn load_audio_track(
    loader: &Loader,
    world: &World,
    progress_counter: &mut ProgressCounter,
    file: &str,
) -> SourceHandle {
    loader.load(file, OggFormat, progress_counter, &world.read_resource())
}

/// Initialise audio in the world. This will eventually include
/// the background tracks as well as the sound effects, but for now
/// we'll just work on sound effects.
pub fn initialise_audio(world: &mut World, progress_counter: &mut ProgressCounter) {
    let sound_effects = {
        let loader = world.read_resource::<Loader>();
        Sounds {
            music: load_audio_track(&loader, &world, progress_counter, "audio/blippy-trance.ogg"),
            intro_buzz: load_audio_track(&loader, &world, progress_counter, "audio/logo_buzz.ogg"),
        }
    };

    // Add sound effects to the world. We have to do this in another scope because
    // world won't let us insert new resources as long as `Loader` is borrowed.
    world.insert(sound_effects);
}

pub fn play_music(sounds: &Sounds, storage: &AssetStorage<Source>, output: Option<&Output>) {
    if let Some(ref output) = output.as_ref() {
        if let Some(sound) = storage.get(&sounds.music) {
            output.play_n_times(sound, 1.0, 20);
        }
    }
}

pub fn play_buzz(sounds: &Sounds, storage: &AssetStorage<Source>, output: Option<&Output>) {
    if let Some(ref output) = output.as_ref() {
        if let Some(sound) = storage.get(&sounds.intro_buzz) {
            output.play_once(sound, 1.0);
        }
    }
}
