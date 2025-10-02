//! Build a Bevy UI and display a Gif.

use bevy::prelude::*;
use bevy_easy_gif::*;

pub fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest()) // sharp zoomed images
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "GifNode example - bevy_easy_gif".to_string(),
                        resolution: bevy::window::WindowResolution::new(400.0, 200.0),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(GifPlugin)
        .add_systems(Startup, (setup_camera, spawn_ui))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            ..OrthographicProjection::default_2d()
        }),
    ));
}

fn spawn_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let handle = asset_server.load("frog_large.gif");
    commands.spawn((
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            row_gap: Val::Px(12.0),
            ..default()
        },
        children![Text::new("Hello dear"), GifNode { handle }],
    ));
}
