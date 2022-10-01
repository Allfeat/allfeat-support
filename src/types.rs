use sp_runtime::traits::ConstU32;
use sp_runtime::{BoundedBTreeMap, BoundedVec};

/// How many parent styles a style register can contain.
pub type MAX_PARENT_STYLES = ConstU32<32>;
/// How many sub-styles a parent style can contain.
pub type MAX_SUB_STYLES = ConstU32<32>;

/// A music style name, with a fixed size of maximum 32 characters.
pub type StyleName = BoundedVec<u8, ConstU32<32>>;
/// A list containing a maximum number of style name, representing some sub-styles.
pub type SubStyles = BoundedVec<StyleName, MAX_SUB_STYLES>;
/// A fixed size register containing some style name as parent styles, linked to a list of sub-styles.
pub type Styles = BoundedBTreeMap<StyleName, SubStyles, MAX_PARENT_STYLES>;
