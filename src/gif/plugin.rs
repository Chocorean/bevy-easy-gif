use bevy::prelude::*;

use crate::gif::{GifAsset, animate_gifs, components::GifLoader, initialize_gifs};

pub struct GifPlugin;

impl Plugin for GifPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<GifAsset>();
        app.init_asset_loader::<GifLoader>();
        app.add_systems(Update, (initialize_gifs, animate_gifs));
    }
}
