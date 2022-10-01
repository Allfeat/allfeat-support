use sp_runtime::traits::ConstU32;
use sp_runtime::{BoundedBTreeMap, BoundedVec};

/// How many parent styles a style register can contain.
pub type MaxParentStyles = ConstU32<32>;
/// How many sub-styles a parent style can contain.
pub type MaxSubStyles = ConstU32<32>;
/// The maximum length of a style name.
pub type MaxNameLength = ConstU32<32>;

/// A music style name, with a fixed size of maximum 32 characters.
pub type StyleName = BoundedVec<u8, MaxNameLength>;
/// A list containing a maximum number of style name, representing some sub-styles.
pub type SubStyles = BoundedVec<StyleName, MaxSubStyles>;
/// A fixed size register containing some style name as parent styles, linked to a list of sub-styles.
pub type Styles = BoundedBTreeMap<StyleName, SubStyles, MaxParentStyles>;
