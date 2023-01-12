use sp_runtime::codec::{Decode, Encode, MaxEncodedLen};
use sp_runtime::scale_info::TypeInfo;

pub mod artist;

#[cfg(feature = "std")]
/// Something defined as an Actor on the network.
pub trait Actor:
    Clone + Encode + Decode + Eq + PartialEq + std::fmt::Debug + MaxEncodedLen + TypeInfo
{
}

#[cfg(not(feature = "std"))]
/// Something defined as an Actor on the network.
pub trait Actor: Clone + Encode + Decode + Eq + PartialEq + MaxEncodedLen + TypeInfo {}

/// Something defined as a data storage of an actor on the blockchain.
pub trait ActorStorage<T> {
    fn is_actor(account_id: &T) -> bool;
}
