//! 3D Gif example.

use std::f32::consts::FRAC_PI_2;

use bevy::prelude::*;
use bevy_easy_gif::*;

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GifPlugin)
        .add_systems(Startup, scene.spawn())
        .add_systems(Update, rotate_cubes)
        .run();
}

#[derive(Component, Clone, Default)]
pub struct Rotate;

// From https://github.com/bevyengine/bevy/blob/v0.19.0-rc.3/examples/3d/3d_scene.rs [bevy 0.19-rc3]
// slitghly changed the cuboid
fn scene() -> impl SceneList {
    bsn_list! [
        (
            #CircularBase
            Mesh3d(asset_value(Circle::new(4.0)))
            Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2))
            Gif3d { handle: "frog_large.gif" }
        ),
        (
            #CubeLeft
            Mesh3d(asset_value(Cuboid::new(1.0, 1.0, 1.0)))
            MeshMaterial3d::<StandardMaterial>(asset_value(Color::srgb_u8(124, 144, 255)))
            template_value(Transform::from_xyz(-2.5, 1.5, 0.0).with_rotation(Quat::from_axis_angle(Vec3::new(0.2, 0., 0.5), FRAC_PI_2)))
            Gif3d { handle: "frog_large.gif" }
            Rotate
        ),
        (
            #CubeRight
            Mesh3d(asset_value(Cuboid::new(1.0, 1.0, 1.0)))
            MeshMaterial3d::<StandardMaterial>(asset_value(Color::srgb_u8(124, 144, 255)))
            template_value(Transform::from_xyz(2.5, 1.5, 0.0).with_rotation(Quat::from_axis_angle(Vec3::new(0.2, 0., 0.5), FRAC_PI_2)))
            Gif3d { handle: "frog_five.gif" }
            GifDespawn
            Rotate
        ),
        (
            PointLight {
                shadow_maps_enabled: true,
                intensity: 10_000_000.,
            }
            Transform::from_xyz(4.0, 12.0, 4.0)
        ),
        (
            Camera3d
            template_value(Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y))
        )
    ]
}

fn rotate_cubes(transform: Query<&mut Transform, With<Rotate>>, time: Res<Time>) {
    for mut t in transform {
        t.rotate_y(time.delta_secs() / 2.);
    }
}
