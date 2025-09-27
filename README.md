# bevy-easy-gif 🐸

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
| 0.16 | 0.1.2 |

## Tests

To test the library, run: `cargo test --lib`.
To see an example using wasm, run `bin/wasm`, and [open your browser](http://localhost:8000).