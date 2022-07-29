//! Autogenerated weights for pallet_token_multi
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-07-29, STEPS: `50`, REPEAT: 200, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/web3games-node
// benchmark
// pallet
// --chain
// dev
// --execution
// wasm
// --wasm-execution
// compiled
// --pallet
// pallet_token_multi
// --extrinsic
// *
// --steps
// 50
// --repeat
// 200
// --template
// ./.maintain/w3g-weight-template.hbs
// --output
// ./pallets/token-multi/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_token_multi.
pub trait WeightInfo {
    fn create_token() -> Weight;
    fn mint() -> Weight;
    fn mint_batch() -> Weight;
    fn set_approval_for_all() -> Weight;
    fn burn() -> Weight;
    fn burn_batch() -> Weight;
    fn transfer_from() -> Weight;
    fn batch_transfer_from() -> Weight;
}

/// Weights for pallet_token_multi using the Web3Games node and recommended hardware.
pub struct W3GWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for W3GWeight<T> {
    // Storage: TokenMulti Tokens (r:1 w:1)
    // Storage: System Account (r:1 w:1)
    fn create_token() -> Weight {
        (31_210_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().writes(2 as Weight))
    }
    // Storage: TokenMulti Tokens (r:1 w:1)
    // Storage: TokenMulti Balances (r:1 w:1)
    fn mint() -> Weight {
        (24_415_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().writes(2 as Weight))
    }
    // Storage: TokenMulti Tokens (r:1 w:1)
    // Storage: TokenMulti Balances (r:5 w:5)
    fn mint_batch() -> Weight {
        (50_137_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(6 as Weight))
            .saturating_add(T::DbWeight::get().writes(6 as Weight))
    }
    // Storage: TokenMulti Tokens (r:1 w:0)
    // Storage: TokenMulti OperatorApprovals (r:0 w:1)
    fn set_approval_for_all() -> Weight {
        (18_618_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    // Storage: TokenMulti Tokens (r:1 w:1)
    // Storage: TokenMulti Balances (r:1 w:1)
    fn burn() -> Weight {
        (22_537_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().writes(2 as Weight))
    }
    // Storage: TokenMulti Tokens (r:1 w:1)
    // Storage: TokenMulti Balances (r:5 w:5)
    fn burn_batch() -> Weight {
        (54_225_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(6 as Weight))
            .saturating_add(T::DbWeight::get().writes(6 as Weight))
    }
    // Storage: TokenMulti Balances (r:2 w:2)
    fn transfer_from() -> Weight {
        (25_134_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().writes(2 as Weight))
    }
    // Storage: TokenMulti Balances (r:10 w:10)
    fn batch_transfer_from() -> Weight {
        (74_825_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(10 as Weight))
            .saturating_add(T::DbWeight::get().writes(10 as Weight))
    }
}

// For backwards compatibility and tests
impl WeightInfo for () {
    fn create_token() -> Weight {
        (31_210_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(2 as Weight))
            .saturating_add(RocksDbWeight::get().writes(2 as Weight))
    }
    fn mint() -> Weight {
        (24_415_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(2 as Weight))
            .saturating_add(RocksDbWeight::get().writes(2 as Weight))
    }
    fn mint_batch() -> Weight {
        (50_137_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(6 as Weight))
            .saturating_add(RocksDbWeight::get().writes(6 as Weight))
    }
    fn set_approval_for_all() -> Weight {
        (18_618_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(1 as Weight))
            .saturating_add(RocksDbWeight::get().writes(1 as Weight))
    }
    fn burn() -> Weight {
        (22_537_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(2 as Weight))
            .saturating_add(RocksDbWeight::get().writes(2 as Weight))
    }
    fn burn_batch() -> Weight {
        (54_225_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(6 as Weight))
            .saturating_add(RocksDbWeight::get().writes(6 as Weight))
    }
    fn transfer_from() -> Weight {
        (25_134_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(2 as Weight))
            .saturating_add(RocksDbWeight::get().writes(2 as Weight))
    }
    fn batch_transfer_from() -> Weight {
        (74_825_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(10 as Weight))
            .saturating_add(RocksDbWeight::get().writes(10 as Weight))
    }
}