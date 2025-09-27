//! Basically the same as [basic](basic.rs), however one animation loop occurs 5 times, and the other only once.
//! This is configured within the GIF file, and can be checked by inspecting the metadata,
//! with tools such as EXIF for instance.

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
                        resolution: bevy::window::WindowResolution::new(400.0, 200.0),
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
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: bevy::render::camera::ScalingMode::WindowSize,
            ..OrthographicProjection::default_2d()
        }),
    ));
}

fn spawn_gif(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    let handle: Handle<GifAsset> = asset_server.load("frog_once.gif");
    commands.spawn((
        Gif { handle },
        Sprite {
            custom_size: Some(Vec2::new(32., 32.)),
            ..default()
        },
        GifDespawn,
    ));
}
