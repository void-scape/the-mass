// Disable console on Windows
#![windows_subsystem = "windows"]

use bevy::prelude::*;

fn main() {
    let mut app = App::default();
    app.add_plugins(
        DefaultPlugins
            .set(AssetPlugin {
                meta_check: bevy::asset::AssetMetaCheck::Never,
                ..default()
            })
            .set(WindowPlugin {
                primary_window: Window {
                    title: "the-mass".to_string(),
                    fit_canvas_to_parent: true,
                    ..default()
                }
                .into(),
                ..default()
            }),
    );
    app.add_plugins((
        bevy_seedling::SeedlingPlugin::default(),
        jamtil::JamtilPlugin,
    ));
    app.add_systems(Startup, spawn_camera);
    #[cfg(feature = "dev")]
    app.add_systems(Update, jamtil::prelude::exit_on_input(KeyCode::Escape));
    app.run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera3d::default());
}
