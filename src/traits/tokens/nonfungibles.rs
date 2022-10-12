pub use frame_support::traits::tokens::nonfungibles::{self, Inspect, Mutate};
use sp_runtime::DispatchResult;

/// Trait for providing an interface for sealing a collection of NFT which may be
/// attributes set on them and that we want to be sure that these attributes aren't
/// gonna be modified in the future..
pub trait Seal<AccountId>: Mutate<AccountId> {
    fn seal(collection: &Self::CollectionId) -> DispatchResult;
}

pub trait MutateExtend<AccountId>: Mutate<AccountId> {
    fn set_collection_max_supply(
        collection: &Self::CollectionId,
        max_supply: u32,
    ) -> DispatchResult;
}
