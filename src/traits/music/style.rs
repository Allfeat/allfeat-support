use sp_runtime::codec::MaxEncodedLen;
use sp_runtime::DispatchResult;
use sp_std::prelude::*;

pub trait InspectMusicStyles {
    /// Type of a style name.
    type StyleName: MaxEncodedLen;
    /// Type of a register containing all the styles.
    type Styles: MaxEncodedLen;

    /// Get the full register containing styles and sub-styles.
    fn styles() -> Self::Styles;
    /// Get all the parent_styles contained in the register.
    fn parent_styles() -> Vec<Self::StyleName>;
    /// Get all the sub-styles contained in the register.
    fn sub_styles() -> Vec<Self::StyleName>;
    /// Verify that the given style name exist.
    fn exist(style_name: &Self::StyleName) -> bool;
    /// Verify that the given style name exist and is a parent style.
    fn is_parent_style(style_name: &Self::StyleName) -> bool;
    /// Verify that the given style name exist and is a sub-style.
    fn is_sub_style(style_name: &Self::StyleName) -> bool;
    /// Convert the given value into a style name and check if it exist, returning None if not.
    /// Throw an Error if the value can't be converted.
    fn exist_from<T: TryInto<Self::StyleName>>(
        data: T,
    ) -> Result<Option<Self::StyleName>, T::Error> {
        let style_name = data.try_into()?;

        if Self::exist(&style_name) {
            Ok(Some(style_name))
        } else {
            Ok(None)
        }
    }
}

pub trait MutateMusicStyles {
    /// Type of a style name.
    type StyleName;

    /// Add a parent style into the database.
    fn add_parent_style(style_name: Self::StyleName) -> DispatchResult;
    /// Add a sub-style into the database.
    fn add_sub_style(
        sub_style_name: Self::StyleName,
        parent_style: Self::StyleName,
    ) -> DispatchResult;
}
