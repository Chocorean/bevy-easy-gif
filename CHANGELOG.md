# bevy_easy_gif changelog

## 0.2.1

- 0.2.0 had already been used but I yanked it (lesson learned). Using 0.2.1 to publish it.
- Remove warnings

## 0.2.0

- Upgrade to bevy 0.17

## 0.1.5

- Introduce `Gif3d`, a new component for attaching a Gif to a `MeshMaterial3d<StandardMaterial>`

## 0.1.4

- Fix bug where same GifAsset would get their handles doubled if loaded another time
- Introduce GifNode, a Bevy UI component for displaying GIF.
- Add a UI example

## 0.1.3

- Remove useless prelude
- Improve top level documentation
- Add readme badges

## 0.1.2

- Add `GifDespawn` component, useful for despawning gifs with finite loops after they are done

## 0.1.1

- Fix asset loading bug in wasm environment

## 0.1.0

- Load gif files with asset loader
- Support infinite loops, finite loops