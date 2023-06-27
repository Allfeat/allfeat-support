use codec::{Decode, Encode, MaxEncodedLen};
use core::marker::PhantomData;
use frame_support::traits::EnsureOrigin;
use scale_info::TypeInfo;

/// Origin for the chain extension calls.
#[derive(PartialEq, Eq, Clone, Encode, Decode, TypeInfo, MaxEncodedLen)]
#[codec(mel_bound(AccountId: MaxEncodedLen))]
pub enum RawChainExtOrigin<AccountId> {
    /// It has been condoned by the contract caller.
    Caller(AccountId),
    /// It has been condoned by the contract itself.
    Contract(AccountId),
}

/// Ensure that the origin is from the contract of a contract call
pub struct EnsureContract<AccountId>(PhantomData<AccountId>);
impl<
        O: Into<Result<RawChainExtOrigin<AccountId>, O>> + From<RawChainExtOrigin<AccountId>>,
        AccountId: Decode,
    > EnsureOrigin<O> for EnsureContract<AccountId>
{
    type Success = AccountId;

    fn try_origin(o: O) -> Result<Self::Success, O> {
        o.into().and_then(|o| match o {
            RawChainExtOrigin::Contract(id) => Ok(id),
            _ => Err(O::from(o)),
        })
    }

    #[cfg(feature = "runtime-benchmarks")]
    fn try_successful_origin() -> Result<O, ()> {
        let zero_account_id =
            AccountId::decode(&mut sp_runtime::traits::TrailingZeroInput::zeroes())
                .expect("infinite length input; no invalid inputs for type; qed");
        Ok(O::from(RawChainExtOrigin::Contract(zero_account_id)))
    }
}

/// Ensure that the origin is from the caller of a contract call
pub struct EnsureContractCaller<AccountId>(PhantomData<AccountId>);
impl<
        O: Into<Result<RawChainExtOrigin<AccountId>, O>> + From<RawChainExtOrigin<AccountId>>,
        AccountId: Decode,
    > EnsureOrigin<O> for EnsureContractCaller<AccountId>
{
    type Success = AccountId;

    fn try_origin(o: O) -> Result<Self::Success, O> {
        o.into().and_then(|o| match o {
            RawChainExtOrigin::Caller(id) => Ok(id),
            _ => Err(O::from(o)),
        })
    }

    #[cfg(feature = "runtime-benchmarks")]
    fn try_successful_origin() -> Result<O, ()> {
        let zero_account_id =
            AccountId::decode(&mut sp_runtime::traits::TrailingZeroInput::zeroes())
                .expect("infinite length input; no invalid inputs for type; qed");
        Ok(O::from(RawChainExtOrigin::Caller(zero_account_id)))
    }
}
