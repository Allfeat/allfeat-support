use crate::traits::actors::Actor;
use allfeat_primitives::BlockNumber;
use sp_runtime::codec::{Decode, Encode, MaxEncodedLen};
use sp_runtime::scale_info::TypeInfo;
use sp_runtime::traits::ConstU32;
use sp_runtime::{BoundedVec, RuntimeDebug};

/// Type of an Artist that live on the Allfeat chain.
pub type AllfeatArtist = ArtistData<BoundedName, BlockNumber>;
/// Type of a Candidate that live on the Allfeat chain.
pub type AllfeatCandidate = CandidateData<BoundedName, BlockNumber>;

/// Represent a bounded artist/candidate name on the Allfeat chain
pub type BoundedName = BoundedVec<u8, ConstU32<128>>;

/// Structure that holds the artist information that will be stored on-chain
#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, MaxEncodedLen, TypeInfo)]
pub struct ArtistData<BoundedString, BlockNumber> {
    /// The name of the artist.
    pub name: BoundedString,
    /// The block number when the artist was created
    pub created_at: BlockNumber,
}

/// Structure that holds the candidate information that will be stored on-chain
#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, MaxEncodedLen, TypeInfo)]
pub struct CandidateData<BoundedString, BlockNumber> {
    /// The name of the artist.
    pub name: BoundedString,
    /// The block number when the artist was created
    pub created_at: BlockNumber,
}

impl Actor for AllfeatArtist {}
impl Actor for AllfeatCandidate {}
