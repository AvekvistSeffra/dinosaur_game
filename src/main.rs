use amethyst::{
    prelude::*,
    renderer::{
        plugins::{ RenderFlat2D, RenderToWindow },
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};

mod dinosaur;
use dinosaur::Dinosaur;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let display_config_path = app_root.join("config").join("display.ron");

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([ 0.0, 0.0, 0.0, 1.0 ]),
                )
                .with_plugin(RenderFlat2D::default()),
        )?;

    let assets_dir = app_root.join("assets");
    let mut game = Application::new(assets_dir, Dinosaur, game_data)?;

    game.run();

    Ok(())
}
