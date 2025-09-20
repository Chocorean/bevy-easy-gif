mod gif;

pub mod prelude {
    //! A "prelude" for crates using the `bevy_gif` crate.
    //!
    //! ```no_run
    //! # #[allow(unused_imports)]
    //! use bevy_gif::prelude::*;
    //! ```

    pub use crate::gif::{Gif, GifAsset, GifPlayer, GifPlugin};
}
