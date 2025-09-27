use bevy::{asset::Handle, ecs::event::Event};

use crate::gif::GifAsset;

#[derive(Event)]
pub(crate) struct GifDespawnEvent(pub Handle<GifAsset>);
