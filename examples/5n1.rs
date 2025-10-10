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
                        title: "Five times repeat example - bevy_easy_gif".to_string(),
                        resolution: bevy::window::WindowResolution::new(400, 200),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(GifPlugin)
        .add_systems(Startup, (setup_camera, spawn_gifs, spawn_labels))
        .add_systems(Update, update_labels)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn spawn_gifs(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    let handle: Handle<GifAsset> = asset_server.load("frog_five.gif");
    commands.spawn((
        Gif { handle },
        Sprite {
            custom_size: Some(Vec2::new(32., 32.)),
            ..default()
        },
        Transform::from_translation(Vec3::new(-20., 0., 0.)),
        Left,
    ));
    let handle: Handle<GifAsset> = asset_server.load("frog_once.gif");
    commands.spawn((
        Gif { handle },
        Sprite {
            flip_x: true,
            custom_size: Some(Vec2::new(32., 32.)),
            ..default()
        },
        Transform::from_translation(Vec3::new(20., 0., 0.)),
        Right,
    ));
}

#[derive(Component)]
struct First;

#[derive(Component)]
struct Sec;

#[derive(Component)]
struct Left;

#[derive(Component)]
struct Right;

fn spawn_labels(mut commands: Commands) {
    commands
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((Text("Current loop: unknown".to_string()), First, Left));
            parent.spawn((Text("Remaining loops: unknown".to_string()), Sec, Left));
        });
    commands
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::FlexEnd,
            position_type: PositionType::Absolute,
            right: Val::Px(0.),
            bottom: Val::Px(0.),
            width: Val::Auto,
            height: Val::Auto,
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((Text("Current loop: unknown".to_string()), First, Right));
            parent.spawn((Text("Remaining loops: unknown".to_string()), Sec, Right));
        });
}

fn update_labels(
    mut first_l: Single<&mut Text, (With<First>, With<Left>, Without<Sec>, Without<Right>)>,
    mut sec_l: Single<&mut Text, (With<Sec>, With<Left>, Without<First>, Without<Right>)>,
    gif_l: Single<&GifPlayer, With<Left>>,
    mut first_r: Single<&mut Text, (With<First>, With<Right>, Without<Sec>, Without<Left>)>,
    mut sec_r: Single<&mut Text, (With<Sec>, With<Right>, Without<First>, Without<Left>)>,
    gif_r: Single<&GifPlayer, With<Right>>,
) {
    let rem = gif_l.remaining.unwrap_or(0);
    first_l.0 = format!("Current loop: #{}", 5 - rem);
    sec_l.0 = format!("Remaining loops: {}", rem);

    let rem = gif_r.remaining.unwrap_or(0);
    first_r.0 = format!("Current loop: #{}", 1 - rem);
    sec_r.0 = format!("Remaining loops: {}", rem);
}
