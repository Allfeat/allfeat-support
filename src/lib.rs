#![cfg_attr(not(feature = "std"), no_std)]

pub mod traits;
pub mod types;

pub mod prelude {
    pub use crate::traits::music::style::InspectMusicStyles;
    pub use crate::types::music::style::{MusicStyleDB, MusicStyleName};
}
