# bevy-easy-gif 🐸

[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Doc](https://docs.rs/bevy_easy_gif/badge.svg)](https://docs.rs/bevy_easy_gif)
[![Crate](https://img.shields.io/crates/v/bevy_easy_gif.svg)](https://crates.io/crates/bevy_easy_gif)
[![Bevy tracking](https://img.shields.io/badge/Bevy%20tracking-v0.18-lightblue)](https://github.com/bevyengine/bevy/blob/main/docs/plugins_guidelines.md#main-branch-tracking)
[![CI testing](https://github.com/Chocorean/bevy-easy-gif/actions/workflows/test.yml/badge.svg)](https://github.com/Chocorean/bevy-easy-gif/actions/workflows/test.yml)

<img src="assets/frog_large.gif" alt="frog" width="200"/>

A 0-work 0-pain way to display GIF files in your Bevy games.

> [!WARNING]
> This project is still in the early stages of development.

## Usage

This is all it takes in a system to spawn a Sprite with an animated texture from a GIF file:

```rust
// It does require a `Camera2d` and the `GifPlugin`
fn spawn_gif(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    let handle: Handle<GifAsset> = asset_server.load("frog_large.gif");
    commands.spawn(
        Gif { handle }
    );
}
```

There are also [a few examples](./examples/) you can check for customization.

## Features matrix

| Bevy version | 0.16 - 0.18 |
|--------------|-------------|
| Gif3d | :heavy_check_mark: |
| GifDespawn | :heavy_check_mark: |
| GifNode | :heavy_check_mark: |
| Gif | :heavy_check_mark: |

## Why?

GIF files are by default not supported by Bevy's asset loader (only the first frame is loaded).

The current workaround is to export GIF files as atlases, use a [TextureAtlas](https://docs.rs/bevy/latest/bevy/prelude/struct.TextureAtlas.html)
with a Sprite, and then manually animate the sprite by changing the atlas properties.

I wanted a drop-in solution to load GIF files and display them to way they show in any other GIF viewer.
Respecting the timing and the repetitions of the GIF files is supported too.

## How?

This crate leverages [required components](https://docs.rs/bevy/latest/bevy/prelude/trait.Component.html#required-components) introduced in Bevy 0.15.
Spawning a `Gif` component will automatically add a `Sprite` alongside, and the `GifPlugin` will then animate the Sprite to display the frames
of the GIF file.
The `Gif` components also carry a `GifPlayer`, which describe the internal state of the entity: the current frame to display, and how long it should stay displayed.
The `GifPlayer`'s timer is automatically configured, after reading the GIF metadata.

## Bevy support

| bevy | bevy-easy-gif |
|------|---------------|
| 0.18 | 0.2.1 |
| 0.17 | 0.2.0 |
| 0.16 | 0.1.5 |

## Tests

To test the library, run: `cargo test --lib`.
To see an example using wasm, run `bin/wasm`, and [open your browser](http://localhost:8000).
