use std::time::Duration;

use bevy::{
    asset::{AssetLoader, LoadContext, io::Reader},
    prelude::*,
};
use gif::{ColorOutput, DecodeOptions, Repeat};
use thiserror::Error;

/// Entity used to spawn a [Sprite] with an animated texture.
/// This is the main and might be the only struct you will use from this crate.
///
/// ```no_run
/// commands.spawn(Gif { handle: asset_server.load("frog.gif") })
/// ```
#[derive(Component, Debug, Clone)]
#[require(Sprite, GifPlayer)]
pub struct Gif {
    pub handle: Handle<GifAsset>,
}

/// Internal state of a [Gif]. Store the current frame index, its associated timer,
/// and the number of remaining repetitions, minus the one currently running.
///
/// That means a [GifPlayer] with `remaining` being 0 and its `timer` not paused will
/// still update the [Sprite] for a last rotation.
///
/// Also: `remaining` == None is different from `remaining` == Some(0)
/// The former means: Repeat indefinitely.
/// The latter: Do not repeat _anymore_.
/// Ultimately, `remaining` == Some(n: n!= 0) means: Repeat n more time(s).
#[derive(Component, Debug, Clone)]
pub struct GifPlayer {
    pub current: usize,
    pub timer: Timer,
    pub remaining: Option<u16>,
}

impl Default for GifPlayer {
    fn default() -> Self {
        Self {
            current: 0,
            timer: Timer::new(Duration::from_millis(100), TimerMode::Repeating),
            remaining: None,
        }
    }
}

/// Contains the data of one frame of a GIF
///
/// What really distinguish this from using a [TextureAtlas] is the unique [Duration] of each frame,
/// stored within the asset.
#[derive(Debug, Clone)]
pub struct GifFrame {
    pub width: u32,
    pub height: u32,
    pub rgba: Vec<u8>,
    pub duration: Duration,
}

/// Contains the data of a GIF
///
/// Careful: `times` represents the raw value of the GIF repeat metadata, which can
/// be interpreted as "how many times will I _repeat_", with an emphasis on _repeat_.
/// For a GIF that plays a total of 5 loops, this value is going to be 4.
#[derive(Asset, TypePath, Debug, Clone)]
pub struct GifAsset {
    pub frames: Vec<GifFrame>,
    pub handles: Vec<Handle<Image>>,
    pub times: Option<u16>,
}

#[derive(Error, Debug)]
pub(crate) enum GifLoaderError {
    /// An [IO](std::io) Error
    #[error("Could not load asset: {0}")]
    Io(#[from] std::io::Error),
    /// A [gif](gif) DecodingError
    #[error("Could not decode asset: {0}")]
    Decode(#[from] gif::DecodingError),
    /// A data error
    #[error("Decoded gif frame size mismatch: {0} != {1}")]
    SizeMismatch(usize, usize),
}

/// Allow to load GIF files properly with the AssetServer
#[derive(Default, TypePath)]
pub(crate) struct GifLoader;

impl AssetLoader for GifLoader {
    type Asset = GifAsset;
    type Settings = bool;
    type Error = GifLoaderError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &Self::Settings,
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;

        let mut decoder = DecodeOptions::new();
        decoder.set_color_output(ColorOutput::RGBA);
        let mut decoder = decoder.read_info(std::io::Cursor::new(bytes))?;

        let mut frames = Vec::new();
        while let Some(frame) = decoder.read_next_frame()? {
            let width = frame.width as u32;
            let height = frame.height as u32;
            let rgba = frame.buffer.to_vec();

            // Make sure data is not truncated or smth
            if rgba.len() != (width as usize) * (height as usize) * 4 {
                return Err(Self::Error::SizeMismatch(
                    rgba.len(),
                    (width as usize) * (height as usize) * 4,
                ));
            }

            // frame.delay is in 1/100th of a second, per [GIF spec](https://docs.rs/gif/latest/gif/struct.Frame.html#structfield.delay)
            let ms = (frame.delay as u64).saturating_mul(10);
            let duration = Duration::from_millis(ms.max(1)); // avoid 0 ms frames

            frames.push(GifFrame {
                width,
                height,
                rgba,
                duration,
            });
        }

        let times = match decoder.repeat() {
            Repeat::Infinite => None,
            Repeat::Finite(n) => Some(n),
        };

        // Create the GifAsset and set it as the default loaded asset
        let asset = GifAsset {
            frames,
            handles: vec![], // will be loaded in `initialize_gifs`
            times,
        };
        Ok(asset)
    }

    fn extensions(&self) -> &[&str] {
        &["gif"]
    }
}

/// Insert this component next to a non-infinite [Gif] to despawn the
/// entity when its loops are over.
///
/// See [despawn example](examples/despawn.rs)
///
/// It has no effect on infinite-looping GIF files.
#[derive(Component)]
pub struct GifDespawn;

/// Ui component to display a gif file.
///
/// Works the same than [Gif]
#[derive(Component, Debug, Clone)]
#[require(ImageNode, GifPlayer)]
pub struct GifNode {
    pub handle: Handle<GifAsset>,
}

/// 3d component to display a gif file on a 3d object.
///
/// It needs to be spawned alongside a [Mesh3d].
///
/// Works almost the same than [Gif]
#[derive(Component, Debug, Clone)]
#[require(MeshMaterial3d<StandardMaterial>, GifPlayer)]
pub struct Gif3d {
    pub handle: Handle<GifAsset>,
}
