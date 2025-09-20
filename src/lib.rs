mod gif;

pub mod prelude {
    //! A "prelude" for crates using the `bevy_gif` crate.
    //!
    //! This prelude is similar to the standard library's prelude in that you'll
    //! almost always want to import its entire contents, but unlike the
    //! standard library's prelude you'll have to do so manually:
    //!
    //! ```
    //! # #[allow(unused_imports)]
    //! use bevy_gif::prelude::*;
    //! ```
    //!
    //! The prelude may grow over time as additional items see ubiquitous use.

    pub use crate::gif::{Gif, GifAsset, GifPlayer, GifPlugin};
}
