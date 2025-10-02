# bevy_easy_gif changelog

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