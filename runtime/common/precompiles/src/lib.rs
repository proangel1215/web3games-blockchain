#![cfg_attr(not(feature = "std"), no_std)]

use codec::Decode;
use evm::{executor::PrecompileOutput, Context, ExitError, ExitSucceed};
use fp_evm::{Precompile, PrecompileSet};
use frame_support::dispatch::{Dispatchable, GetDispatchInfo, PostDispatchInfo};
use pallet_evm_precompile_bn128::{Bn128Add, Bn128Mul, Bn128Pairing};
use pallet_evm_precompile_modexp::Modexp;
use pallet_evm_precompile_simple::{ECRecover, ECRecoverPublicKey, Identity, Ripemd160, Sha256};
use sp_core::H160;
use sp_std::{marker::PhantomData, prelude::*, result, str::FromStr};

pub mod tokens;

pub use tokens::TokensPrecompile;

#[derive(Debug, Clone, Copy)]
pub struct Web3gamesPrecompiles<R>(PhantomData<R>);

impl<R> PrecompileSet for Web3gamesPrecompiles<R>
where
	R::Call: Dispatchable<PostInfo = PostDispatchInfo> + GetDispatchInfo + Decode,
	<R::Call as Dispatchable>::Origin: From<Option<R::AccountId>>,
	R: pallet_evm::Config + pallet_tokens::Config,
	// R::AccountId: From<H160>,
	R::Call: From<pallet_tokens::Call<R>>,
{
	fn execute(
		address: H160,
		input: &[u8],
		target_gas: Option<u64>,
		context: &Context,
	) -> Option<core::result::Result<PrecompileOutput, ExitError>> {
		match address {
			// Ethereum precompiles
			a if a == hash(1) => Some(ECRecover::execute(input, target_gas, context)),
			a if a == hash(2) => Some(Sha256::execute(input, target_gas, context)),
			a if a == hash(3) => Some(Ripemd160::execute(input, target_gas, context)),
			a if a == hash(4) => Some(Identity::execute(input, target_gas, context)),
			a if a == hash(5) => Some(Modexp::execute(input, target_gas, context)),
			a if a == hash(6) => Some(Bn128Add::execute(input, target_gas, context)),
			a if a == hash(7) => Some(Bn128Mul::execute(input, target_gas, context)),
			a if a == hash(8) => Some(Bn128Pairing::execute(input, target_gas, context)),
			// Non-standard precompiles

			// Web3games precompiles
			a if a == hash(10001) => {
				Some(TokensPrecompile::<R>::execute(input, target_gas, context))
			}
			// Not support
			_ => None,
		}
	}
}

fn hash(a: u64) -> H160 {
	H160::from_low_u64_be(a)
}