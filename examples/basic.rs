//! Display frog [Gif]s as a proof of concept!

use bevy::prelude::*;
use bevy_easy_gif::*;

pub fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest()) // sharp zoomed images
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Basic example - bevy_easy_gif".to_string(),
                        resolution: bevy::window::WindowResolution::new(400, 200),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(GifPlugin)
        .add_systems(Startup, (setup_camera, spawn_gif))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn spawn_gif(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    let handle: Handle<GifAsset> = asset_server.load("frog_large.gif");
    commands.spawn(Gif { handle });
}
