use sp_runtime::codec::{Decode, Encode};

pub mod types;

pub trait MusicStylesProvider {
    /// Type of a style name.
    type StyleName: Encode + Decode;
    /// Type of a register containing all the styles.
    type Styles: Encode + Decode;

    /// Get the full register containing styles and sub-styles.
    fn styles() -> Self::Styles;
    /// Get all the parent_styles contained in the register.
    fn parent_styles() -> Vec<Self::StyleName>;
    /// Verify that the given style name exist.
    fn exist(style_name: &Self::StyleName) -> bool;
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
