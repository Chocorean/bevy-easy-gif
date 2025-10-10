//! 3D Gif example.

use std::f32::consts::FRAC_PI_2;

use bevy::prelude::*;
use bevy_easy_gif::*;

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GifPlugin)
        .add_systems(Startup, setup_scene)
        .add_systems(Update, rotate_cubes)
        .run();
}

#[derive(Component)]
pub struct Rotate;

// From https://github.com/bevyengine/bevy/blob/latest/examples/3d/3d_scene.rs [bevy 0.16]
// slitghly changed the cuboid
fn setup_scene(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let handle: Handle<GifAsset> = asset_server.load("frog_large.gif");
    let other: Handle<GifAsset> = asset_server.load("frog_five.gif");

    // circular base
    commands.spawn((
        Gif3d {
            handle: handle.clone(),
        },
        Mesh3d(meshes.add(Circle::new(4.0))),
        Transform::from_rotation(Quat::from_rotation_x(-FRAC_PI_2)),
    ));
    // cubes
    commands.spawn((
        Rotate,
        Gif3d { handle },
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_xyz(-2.5, 1.5, 0.0)
            .with_rotation(Quat::from_axis_angle(Vec3::new(0.2, 0., 0.5), FRAC_PI_2)),
    ));
    commands.spawn((
        Rotate,
        Gif3d { handle: other },
        GifDespawn,
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_xyz(2.5, 1.5, 0.0)
            .with_rotation(Quat::from_axis_angle(Vec3::new(0.2, 0., 0.5), FRAC_PI_2)),
    ));
    // light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            intensity: 10_000_000.,
            ..default()
        },
        Transform::from_xyz(4.0, 12.0, 4.0),
    ));
    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn rotate_cubes(transform: Query<&mut Transform, With<Rotate>>, time: Res<Time>) {
    for mut t in transform {
        t.rotate_y(time.delta_secs() / 2.);
    }
}
