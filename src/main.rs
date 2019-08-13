use amethyst::{
    core::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{RenderUi, UiBundle},
    utils::application_root_dir,
};

mod pong;
use crate::pong::Pong;

mod systems;

fn main() -> amethyst::Result<()> {
    // start logging system
    amethyst::start_logger(Default::default());

    // setup config, app, and asset locations
    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("resources").join("display_config.ron");
    let assets_dir = app_root.join("assets");
    let binding_path = app_root.join("resources").join("bindings.ron");

    // Create game data builder with required systems
    let game_data = GameDataBuilder::default()
        // add rendering and window opening
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)
                        .with_clear([0.0, 0.0, 0.0, 1.0]), // RGBA values for display color
                )
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderUi::default()),
        )?
        // transform bundle to handle entity positions
        .with_bundle(
            TransformBundle::new() 
        )?
        .with_bundle(
            InputBundle::<StringBindings>::new()
                .with_bindings_from_file(binding_path)?
        )?
        .with_bundle(UiBundle::<StringBindings>::new())?
        // pass system, name of system, and list of dependencies to run before our system
        .with(systems::PaddleSystem, "paddle_system", &["input_system"])
        .with(systems::MoveBallsSystem, "ball_system", &[])
        .with(
            systems::BounceSystem,
            "collision_system",
            &["paddle_system", "ball_system"],
        )
        .with(systems::WinnerSystem, "winner_system", &["ball_system"]);

    // combine our game state `Pong` with assets and game systems
    let mut game = Application::new(assets_dir, Pong::default(), game_data)?;

    game.run();
    Ok(())
}
