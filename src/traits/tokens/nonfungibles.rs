pub use frame_support::traits::tokens::nonfungibles::{Inspect, Mutate};
use sp_runtime::DispatchResult;

/// Trait for providing an interface for sealing a collection of NFT which may be
/// attributes set on them and that we want to be sure that these attributes aren't
/// gonna be modified in the future..
pub trait Seal<AccountId>: Mutate<AccountId> {
    fn seal(
        collection: &Self::CollectionId,
        maybe_check_owner: Option<AccountId>,
    ) -> DispatchResult;
}
