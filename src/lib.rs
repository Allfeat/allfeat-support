use crate::types::{StyleName, Styles, MAX_PARENT_STYLES};
use sp_runtime::codec::{Decode, Encode};
use sp_runtime::BoundedVec;
use std::error::Error;

pub mod types;

pub trait MusicStylesProvider {
    /// Type of a style name.
    type StyleName: Encode + Decode;
    /// Type of a register containing all the styles.
    type Styles: Encode + Decode;

    /// Get the full register containing styles and sub-styles.
    fn styles() -> Self::Styles;
    /// Get all the parent_styles contained in the register.
    fn parent_styles() -> BoundedVec<Self::StyleName, MAX_PARENT_STYLES>;
    fn exist_from<T: TryInto<StyleName>>(data: T) -> Option<Self::StyleName> {
        let styles = Self::styles();
        let bounded_data = data.try_into().map_err(|_| None)?;

        // checking in parent styles
        if styles.contains_key(data) {
            return true;
        }
        // checking in sub styles
        for style in styles.values() {
            if style.contains(data) {
                return true;
            }
        }

        false
    }
}
