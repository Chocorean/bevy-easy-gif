//! Using `GifDespawn` allows to despwan the `Gif` when it is done iterating through its internal loop.

use bevy::prelude::*;
use bevy_easy_gif::*;

pub fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest()) // sharp zoomed images
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Despawning example - bevy_easy_gif".to_string(),
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

fn spawn_gif(mut commands: Commands) {
    commands.spawn_scene(bsn! {
        Gif { handle: "frog_once.gif" }
        Sprite { custom_size: Vec2::new(32., 32.) }
        GifDespawn
    });
}
