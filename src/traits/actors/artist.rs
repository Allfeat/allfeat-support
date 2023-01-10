use super::ActorStorage;

pub trait ArtistStorage<T, C, A>: ActorStorage<T> {
    /// Return if the specified account is actually a candidate
    fn is_candidate(account_id: &T) -> bool;
    /// Return if the specified account is actually an artist
    fn is_artist(account_id: &T) -> bool;

    /// Retrieve the candidate informations of a specified account
    fn candidate(account_id: &T) -> Option<C>;
    /// Retrieve the candidate informations of a specified account
    fn artist(account_id: &T) -> Option<A>;
}
