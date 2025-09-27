//! Welcome to `bevy-easy-gif` documentation!
//!
//! Everything is in the prelude.

mod gif;

pub mod prelude {
    //! A "prelude" for projects the `bevy_gif` crate.
    //!
    //! ```no_run
    //! use bevy_gif::prelude::*;
    //! ```

    pub use crate::gif::{Gif, GifAsset, GifDespawn, GifPlayer, GifPlugin};
}
