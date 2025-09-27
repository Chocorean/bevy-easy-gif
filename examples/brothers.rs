//! Display 2 frogs with some parameters.

use bevy::prelude::*;
use bevy_easy_gif::*;

pub fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest()) // sharp zoomed images
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Brothers example - bevy_easy_gif".to_string(),
                        resolution: bevy::window::WindowResolution::new(400.0, 200.0),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(GifPlugin)
        .add_systems(Startup, (setup_camera, spawn_gif, spawn_flipped_larger_gif))
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
    let handle: Handle<GifAsset> = asset_server.load("frog_infinite.gif");
    commands.spawn((
        Gif { handle },
        Transform::from_translation(Vec3::new(-10., 0., 0.)),
        Sprite {
            custom_size: Some(Vec2::new(24., 24.)),
            ..default()
        },
    ));
}

fn spawn_flipped_larger_gif(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    let handle: Handle<GifAsset> = asset_server.load("frog_infinite.gif");
    commands.spawn((
        Gif { handle },
        Transform::from_translation(Vec3::new(40., 6., 0.)),
        Sprite {
            flip_x: true, // won't be overwritten
            custom_size: Some(Vec2::new(40., 40.)),
            ..default()
        },
    ));
}
