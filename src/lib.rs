//! [![Frog](https://github.com/Chocorean/bevy-easy-gif/blob/main/assets/frog_large.gif?raw=true)](https://github.com/Chocorean/bevy-easy-gif)
//!
//! `.gif` files asset loading for the bevy game engine.
//!
//! `bevy-easy-gif` provides a super easy way to load GIF files using Bevy's [AssetLoader](https://docs.rs/bevy/latest/bevy/asset/trait.AssetLoader.html).
//! Under the hood, each frame is extracted and loaded by the asset loader as an [Image](https://docs.rs/bevy/latest/bevy/image/struct.Image.html).
//! The GIF metadata is also partially saved. The duration of each frame is obviously very important, but how the GIF should loop matters as well.
//! If provided, `bevy-easy-gif` will read the `repeat` metadata provided by the [gif crate](https://crates.io/crates/gif), and respect it. That means
//! there is almost zero configuration to do code-wise if your GIF files are properly build (yes, I'm looking at you Aseprite).
//!
//! # Example
//!
//! Add the gif plugin to your app:
//!
//! ```no_run
//! use bevy::prelude::*;
//! use bevy_easy_gif::*;
//!
//! App::default()
//!     .add_plugins(DefaultPlugins)
//!     .add_plugins(GifPlugin)
//!     .run();
//! ```
//!
//! Spawn a [`Gif`]:
//!
//! ```no_run
//! let handle = asset_loader.load("animated.gif") // located in ./assets/
//! commands.spawn(Gif { handle });
//! ```
//!
//! [Gif] leverages the [required components](https://docs.rs/bevy/latest/bevy/ecs/component/struct.RequiredComponents.html)
//! feature of [Bevy 0.15](https://bevy.org/news/bevy-0-15/#required-components) and automagically spawns alongside itself
//! a [Sprite](https://docs.rs/bevy/latest/bevy/sprite/index.html) and a [GifPlayer]. The trick is to change the `image`
//! field of the sprite at very specific timings, mimicking the behavior of a GIF file. [GifPlayer] is an internal state
//! which controls what to show, when to show it, and when to stop.
//!
//! The [examples](https://github.com/Chocorean/bevy-easy-gif/tree/main/examples) cover pretty much all there is to know.

mod gif;
pub use crate::gif::{Gif, GifAsset, GifDespawn, GifPlayer, GifPlugin};
