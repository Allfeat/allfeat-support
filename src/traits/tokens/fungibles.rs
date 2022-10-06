pub use frame_support::traits::fungibles::{Inspect, InspectMetadata, Mutate};
use sp_runtime::DispatchResult;

/// Trait for providing the ability to create new Allfeat fungible assets.
pub trait Create<AccountId>: Inspect<AccountId> {
    // Create a new fungible asset.
    fn create(
        id: Self::AssetId,
        admin: AccountId,
        is_sufficient: bool,
        min_balance: Self::Balance,
        max_supply: Option<Self::Balance>,
    ) -> DispatchResult;
}
