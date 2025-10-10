use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};

use crate::{
    Gif3d, GifNode,
    gif::{Gif, GifAsset, GifDespawn, GifPlayer, messages::GifDespawnMessage},
};

/// Initialize the [Gif]'s [Sprite] / [GifNode]'s [ImageNode] / [Gif3d]'s [MeshMaterial3d] with the first image of the sequence.
pub(crate) fn initialize_gifs(
    mut gifs_q: Query<(
        Option<(&Gif, &mut Sprite)>,
        Option<(&GifNode, &mut ImageNode)>,
        Option<(&Gif3d, &mut MeshMaterial3d<StandardMaterial>)>,
        &mut GifPlayer,
    )>,
    mut gifs: ResMut<Assets<GifAsset>>,
    asset_server: ResMut<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (gif_option, gifnode_option, gif3d_option, mut player) in gifs_q.iter_mut() {
        let handle = if let Some((gif, _)) = gif_option {
            gif.handle.clone()
        } else if let Some((gif_node, _)) = gifnode_option {
            gif_node.handle.clone()
        } else if let Some((gif3d, _)) = gif3d_option {
            gif3d.handle.clone()
        } else {
            panic!("Unexpected error: a GifPlayer was inserted in an unknown entity");
        };

        if let Some(GifAsset {
            frames,
            handles,
            times,
        }) = gifs.get_mut(&handle)
        {
            if handles.len() != 0 {
                // Already loaded, continue
                continue;
            }
            // Build all frames and store them
            for frame in frames.iter() {
                let image = Image::new_fill(
                    Extent3d {
                        width: frame.width,
                        height: frame.height,
                        depth_or_array_layers: 1,
                    },
                    TextureDimension::D2,
                    &frame.rgba,
                    TextureFormat::Rgba8UnormSrgb,
                    RenderAssetUsages::all(),
                );
                let handle = asset_server.add(image);
                handles.push(handle);
            }
            // Get first frame and load it to the sprite
            let frame = frames.first().unwrap();
            let handle = handles.first().unwrap();
            // unwrap()-ing is fine here, because this is called after `asset_server.load()`,
            // which would panic if there is an issue with the GIF file.
            if let Some((_, mut sprite)) = gif_option {
                // just replacing the image allow to not overwrite previously given members (see [brothers example](examples/brothers.rs#spawn_flipped_larger_gif).)
                // same principle for other kinds of gif
                sprite.image = handle.clone();
            }
            if let Some((_, mut image_node)) = gifnode_option {
                image_node.image = handle.clone();
            }
            if let Some((_, mm)) = gif3d_option {
                if let Some(mat) = materials.get_mut(&mm.0) {
                    mat.base_color_texture = Some(handle.clone());
                    mat.alpha_mode = AlphaMode::Blend;
                }
            }

            // initialize timer
            player.current = 0; // first frame
            player.timer = Timer::new(frame.duration, TimerMode::Repeating);
            player.remaining = *times;
        }
    }
}

/// Update the [GifPlayer] of all [Gif]s / [GifNode]s / [Gif3d] entities.
/// If the timer expires, we update the player and the [Sprite] / [ImageNode] image, accordingly to the known config.
/// It updates the [MeshMaterial3d] for 3d objects.
pub(crate) fn animate_gifs(
    gifs_q: Query<(
        Option<(&Gif, &mut Sprite)>,
        Option<(&GifNode, &mut ImageNode)>,
        Option<(&Gif3d, &mut MeshMaterial3d<StandardMaterial>)>,
        &mut GifPlayer,
    )>,
    gifs: Res<Assets<GifAsset>>,
    time: Res<Time>,
    mut writer: MessageWriter<GifDespawnMessage>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (gif_option, gifnode_option, gif3d_option, mut player) in gifs_q {
        let handle = if let Some((gif, _)) = gif_option {
            gif.handle.clone()
        } else if let Some((gif_node, _)) = gifnode_option {
            gif_node.handle.clone()
        } else if let Some((gif3d, _)) = gif3d_option {
            gif3d.handle.clone()
        } else {
            panic!("Unexpected error: a GifPlayer was inserted in an unknown entity");
        };

        if let Some(gif_asset) = gifs.get(&handle) {
            player.timer.tick(time.delta());
            if player.timer.is_finished() {
                // Update timer
                player.current = (player.current + 1) % gif_asset.frames.len();
                let frame = &gif_asset.frames[player.current];
                let new_duration = frame.duration;

                if player.current == 0 {
                    // That means we just ended a loop !
                    if let Some(remaining) = player.remaining {
                        if remaining == 0 {
                            player.timer.pause();
                            writer.write(GifDespawnMessage(handle.clone()));
                        } else {
                            player.remaining = Some(remaining - 1);
                        }
                    }
                    // no else because it means it is an infinite-looping GIF.
                }
                player.timer.set_duration(new_duration);
                player.timer.reset();

                // Update sprite
                let handle = gif_asset.handles[player.current].clone();
                if let Some((_, mut sprite)) = gif_option {
                    sprite.image = handle.clone();
                }
                if let Some((_, mut image_node)) = gifnode_option {
                    image_node.image = handle.clone();
                }
                if let Some((_, mm)) = gif3d_option {
                    if let Some(mat) = materials.get_mut(&mm.0) {
                        mat.base_color_texture = Some(handle.clone());
                        mat.alpha_mode = AlphaMode::Blend;
                    }
                }
            }
        }
    }
}

/// Triggered when a GIF with a finite number of loops reaches its end.
/// Despawn the relevant entity.
pub(crate) fn despawn_gifs(
    mut commands: Commands,
    mut reader: MessageReader<GifDespawnMessage>,
    gif_q: Query<(Option<&Gif>, Option<&GifNode>, Option<&Gif3d>, Entity), With<GifDespawn>>,
) {
    for GifDespawnMessage(handle) in reader.read() {
        for (gif_option, gifnode_option, gif3d_option, entity) in gif_q {
            let gif_handle = if let Some(gif) = gif_option {
                gif.handle.clone()
            } else if let Some(gif_node) = gifnode_option {
                gif_node.handle.clone()
            } else if let Some(gif3d) = gif3d_option {
                gif3d.handle.clone()
            } else {
                panic!("Unexpected error: a GifPlayer was inserted in an unknown entity");
            };
            if gif_handle.id() == handle.id() {
                commands.entity(entity).despawn();
            }
        }
    }
}
