use super::ActorStorage;
use frame_support::traits::PalletError;

pub trait ArtistStorage<T, C, A>: ActorStorage<T> {
    type Err: PalletError;

    /// Return if the specified account is actually a candidate
    fn is_candidate(account_id: &T) -> bool;
    /// Return if the specified account is actually an artist
    fn is_artist(account_id: &T) -> bool;

    /// Retrieve the candidate informations of a specified account
    fn candidate(account_id: &T) -> Result<C, Self::Err>;
    /// Retrieve the candidate informations of a specified account
    fn artist(account_id: &T) -> Result<A, Self::Err>;
}
