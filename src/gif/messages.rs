use bevy::{asset::Handle, ecs::message::Message};

use crate::gif::GifAsset;

#[derive(Message)]
pub(crate) struct GifDespawnMessage(pub Handle<GifAsset>);
