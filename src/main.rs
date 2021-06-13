use amethyst::{
    animation::AnimationBundle,
    assets::PrefabLoaderSystemDesc,
    core::transform::TransformBundle,
    input::InputBundle,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        sprite::SpriteRender,
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
    Application, GameDataBuilder,
};

mod animation;
mod components;
mod input;
mod prefabs;
mod resources;
mod state;
mod systems;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let assets_dir = app_root.join("assets");
    let display_config_path = app_root.join("config").join("display.ron");
    let bindings_path = app_root.join("config").join("bindings.ron");

    let game_data = GameDataBuilder::default()
        .with_system_desc(
            PrefabLoaderSystemDesc::<prefabs::TowerPrefab>::default(),
            "tower_loader",
            &[],
        )
        .with_system_desc(
            PrefabLoaderSystemDesc::<prefabs::BulletPrefab>::default(),
            "bullet_loader",
            &[],
        )
        .with_system_desc(
            PrefabLoaderSystemDesc::<prefabs::SplashAnimationPrefab>::default(),
            "splash_loader",
            &[],
        )
        .with_system_desc(
            PrefabLoaderSystemDesc::<prefabs::BackgroundPrefab>::default(),
            "background_loader",
            &[],
        )
        .with_system_desc(
            PrefabLoaderSystemDesc::<prefabs::PlayerPrefab>::default(),
            "player_loader",
            &[],
        )
        .with_bundle(
            AnimationBundle::<animation::AnimationId, SpriteRender>::new(
                "sprite_animation_control",
                "sprite_sampler_interpolation",
            ),
        )?
        .with_bundle(
            TransformBundle::new()
                .with_dep(&["sprite_animation_control", "sprite_sampler_interpolation"]),
        )?
        .with_bundle(
            InputBundle::<input::GameBindingTypes>::new().with_bindings_from_file(bindings_path)?,
        )?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0., 0., 0., 1.]),
                )
                .with_plugin(RenderFlat2D::default()),
        )?
        .with(systems::PlayerControlSystem, "player_control_system", &[])
        .with(
            systems::ShooterControlSystem::default(),
            "shooter_control_system",
            &[],
        )
        .with(
            systems::TowerDirectionSystem,
            "tower_direction_system",
            &["shooter_control_system"],
        )
        .with(
            systems::BulletSpeedSystem,
            "bullet_speed_system",
            &["shooter_control_system"],
        )
        .with(
            systems::PhysicsSystem,
            "physics_system",
            &["shooter_control_system", "player_control_system"],
        )
        .with(
            systems::CameraFollowSystem,
            "camera_follow_system",
            &["physics_system"],
        );

    let mut game = Application::new(assets_dir, state::Loading::default(), game_data)?;
    game.run();

    Ok(())
}
