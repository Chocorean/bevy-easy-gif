# bevy-easy-gif 🐸

<img src="assets/frog_large.gif" alt="frog" width="200"/>

A 0-work 0-pain way to display GIF files in your Bevy games.

> [!WARNING]
> This project is still in the early stages of development.

## Why?

GIF files are by default not supported by Bevy's asset loader (only the first frame is loaded).

The current workaround is to export GIF files as atlases, use a [TextureAtlas](https://docs.rs/bevy/latest/bevy/prelude/struct.TextureAtlas.html)
with a Sprite, and then manually animate the sprite by changing the atlas properties.

I wanted a drop-in solution to load GIF files and display them to way they show in any other GIF viewer.
Respecting the timing and the repetitions of the GIF files is supported too.

## Usage

Dead-simple:

```rust
fn spawn_gif(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    let handle: Handle<GifAsset> = asset_server.load("frog_large.gif");
    commands.spawn(
        Gif { handle }
    );
}
```

There are also [a few examples](./examples/) you can check for customization.

## Bevy support

| bevy | bevy-easy-gif |
|------|---------------|
| 0.16 | 0.1 |