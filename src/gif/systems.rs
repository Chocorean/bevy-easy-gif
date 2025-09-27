use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};

use crate::gif::{Gif, GifAsset, GifDespawn, GifPlayer, events::GifDespawnEvent};

/// Initialize the [Gif]'s [Sprite] with the first image of the sequence.
pub(crate) fn initialize_gifs(
    mut ev_gif: EventReader<AssetEvent<GifAsset>>,
    mut gifs_q: Query<(&mut Sprite, &Gif, &mut GifPlayer)>,
    mut gifs: ResMut<Assets<GifAsset>>,
    asset_server: ResMut<AssetServer>,
) {
    for ev in ev_gif.read() {
        let id = match ev {
            AssetEvent::LoadedWithDependencies { id } => *id,
            _ => continue,
        };
        for (mut sprite, gif, mut player) in gifs_q.iter_mut() {
            if gif.handle.id() != id {
                continue;
            }
            if let Some(GifAsset {
                frames,
                handles,
                times,
            }) = gifs.get_mut(&gif.handle)
            {
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
                sprite.image = handle.clone();
                // just replacing the image allow to not overwrite previously given members (see [brothers example](examples/brothers.rs#spawn_flipped_larger_gif).)

                // initialize timer
                player.current = 0; // first frame
                player.timer = Timer::new(frame.duration, TimerMode::Repeating);
                player.remaining = *times;
            }
        }
    }
}

/// Update the [GifPlayer] of all [Gif]s entities.
/// If the timer expires, we update the player and the [Sprite] image, accordingly to the known config.
pub(crate) fn animate_gifs(
    gifs_q: Query<(&Gif, &mut Sprite, &mut GifPlayer), With<Gif>>,
    gifs: Res<Assets<GifAsset>>,
    time: Res<Time>,
    mut writer: EventWriter<GifDespawnEvent>,
) {
    for (gif, mut sprite, mut player) in gifs_q {
        if let Some(gif_asset) = gifs.get(&gif.handle) {
            player.timer.tick(time.delta());
            if player.timer.finished() {
                // Update timer
                player.current = (player.current + 1) % gif_asset.frames.len();
                let frame = &gif_asset.frames[player.current];
                let new_duration = frame.duration;

                if player.current == 0 {
                    // That means we just ended a loop !
                    if let Some(remaining) = player.remaining {
                        if remaining == 0 {
                            player.timer.pause();
                            writer.write(GifDespawnEvent(gif.handle.clone()));
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
                sprite.image = handle;
            }
        }
    }
}

/// Triggered when a GIF with a finite number of loops reaches its end.
/// Despawn the relevant entity.
pub(crate) fn despawn_gifs(
    mut commands: Commands,
    mut reader: EventReader<GifDespawnEvent>,
    gif_q: Query<(&Gif, Entity), With<GifDespawn>>,
) {
    for GifDespawnEvent(handle) in reader.read() {
        for (gif, entity) in gif_q {
            if gif.handle.id() == handle.id() {
                commands.entity(entity).despawn();
            }
        }
    }
}
