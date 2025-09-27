use bevy::{asset::Handle, ecs::event::Event};

use crate::gif::GifAsset;

#[derive(Event)]
pub struct GifDespawnEvent(pub Handle<GifAsset>);
