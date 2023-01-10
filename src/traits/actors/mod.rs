pub mod artist;

pub trait Actor {}

/// Something defined as a data storage of an actor on the blockchain.
pub trait ActorStorage<T> {
    fn is_actor(id: &T) -> bool;
}
