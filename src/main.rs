use amethyst::{
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};

// define our game state
pub struct Pong;

impl SimpleState for Pong {}

fn main() -> amethyst::Result<()> {
    // start logging system
    amethyst::start_logger(Default::default());

    // setup config, app, and asset locations
    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("resources").join("display_config.ron");
    let assets_dir = app_root.join("assets");

    // Create game data builder with required systems
    let game_data = GameDataBuilder::default()
        // add rendering
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)
                        .with_clear([0.0, 0.0, 0.0, 1.0]), // RGBA values for display color
                    )
                .with_plugin(RenderFlat2D::default()),
            )?;

    // combine our game state `Pong` with assets and game systems
    let mut game = Application::new(assets_dir, Pong, game_data)?;

    game.run();
    Ok(())
}
