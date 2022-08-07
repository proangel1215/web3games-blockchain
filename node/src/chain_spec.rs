// This file is part of Web3Games.

// Copyright (C) 2021-2022 Web3Games https://web3games.org
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use frame_benchmarking::account;
use hex_literal::hex;
use pallet_evm::{AddressMapping, HashedAddressMapping};
use sc_service::ChainType;
use sc_telemetry::TelemetryEndpoints;
use serde_json::json;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_core::{crypto::UncheckedInto, sr25519, Pair, Public, H160};
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::traits::{BlakeTwo256, IdentifyAccount, Verify};
use std::str::FromStr;
use web3games_runtime::{
	AccountId, AuraConfig, Balance, BalancesConfig, BaseFeeConfig, EVMConfig, EthereumConfig,
	GenesisConfig, GrandpaConfig, Permill, Precompiles, Signature, SudoConfig, SystemConfig,
	DOLLARS, GIGAWEI, WASM_BINARY,
};

// The URL for the telemetry server.
const TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

type AccountPublic = <Signature as Verify>::Signer;

/// Generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Generate an Aura authority key.
pub fn authority_keys_from_seed(s: &str) -> (AuraId, GrandpaId) {
	(get_from_seed::<AuraId>(s), get_from_seed::<GrandpaId>(s))
}

pub fn get_account_id_from_evm_address(address: &str) -> AccountId {
	HashedAddressMapping::<BlakeTwo256>::into_account_id(
		H160::from_str(address).expect("invalid evm address"),
	)
}

pub fn development_config() -> Result<ChainSpec, String> {
	Ok(ChainSpec::from_genesis(
		// Name
		"Web3Games Development Testnet",
		// ID
		"web3games_dev",
		ChainType::Development,
		move || {
			testnet_genesis(
				// Initial PoA authorities
				vec![authority_keys_from_seed("Alice")],
				// Sudo account
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// Pre-funded accounts
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
					get_account_id_from_evm_address("6be02d1d3665660d22ff9624b7be0551ee1ac91b"),
					get_account_id_from_evm_address("573394b77fC17F91E9E67F147A9ECe24d67C5073"),
					account("alice", 0, 0),
					account("bob", 0, 0),
					account("charlie", 0, 0),
				],
				true,
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		None,
		// Properties
		Some(
			json!({
				"tokenDecimals": 18,
				"tokenSymbol": "W3G"
			})
			.as_object()
			.expect("Provided valid json map")
			.clone(),
		),
		// Extensions
		None,
	))
}

pub fn staging_testnet_config() -> Result<ChainSpec, String> {
	Ok(ChainSpec::from_genesis(
		// Name
		"Web3Games Testnet",
		// ID
		"web3games_testnet",
		ChainType::Live,
		move || {
			testnet_genesis(
				// Initial PoA authorities
				vec![
					(
						// 5D9idfnat7BcmSPLYP3YCgvUVz3gAMniZGuk6ztbf29rLdXs
						hex!["300f128295a3d0b3cc605965357173112d4c1d1c5b3cadf179e9586d92ba2e02"]
							.unchecked_into(),
						hex!["300f128295a3d0b3cc605965357173112d4c1d1c5b3cadf179e9586d92ba2e02"]
							.unchecked_into(),
					),
					(
						// 5HfyQpxaw5jeKLHVbqq6NojZNHC7b54TqPEnF5eDJnLgFGYv
						hex!["f8131e0eb898448b630516acfd69be6dbf8ab37a218fcc692d3a91e4f9a63b62"]
							.unchecked_into(),
						hex!["f8131e0eb898448b630516acfd69be6dbf8ab37a218fcc692d3a91e4f9a63b62"]
							.unchecked_into(),
					),
					(
						// 5CSj6yzHxZKwFNAmZFQ3kVTT5XeZZmn3QCpQwNyWeTqeJ89K
						hex!["10cb989431c4a409f25864824e3239fb03f13e3fb4c761d51525c16259093e69"]
							.unchecked_into(),
						hex!["10cb989431c4a409f25864824e3239fb03f13e3fb4c761d51525c16259093e69"]
							.unchecked_into(),
					),
				],
				// Sudo account
				hex![
					// 5CXy43qEuaCtCdLG572vWvfY9wJFCRRiswts69QVPyKYjjCq
					"14cac7ed1b42d80ddef944c866cf18675039ba3fe313f5ef45cfcc5e56e2cc05"
				]
				.into(),
				// Pre-funded accounts
				vec![
					// 5CXy43qEuaCtCdLG572vWvfY9wJFCRRiswts69QVPyKYjjCq
					hex!["14cac7ed1b42d80ddef944c866cf18675039ba3fe313f5ef45cfcc5e56e2cc05"].into(),
				],
				true,
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		TelemetryEndpoints::new(vec![(TELEMETRY_URL.into(), 0)]).ok(),
		// Protocol ID
		None,
		None,
		// Properties
		Some(
			json!({
				"tokenDecimals": 18,
				"tokenSymbol": "W3G"
			})
			.as_object()
			.expect("Provided valid json map")
			.clone(),
		),
		// Extensions
		None,
	))
}

/// Configure initial storage state for FRAME modules.
fn testnet_genesis(
	initial_authorities: Vec<(AuraId, GrandpaId)>,
	root_key: AccountId,
	endowed_accounts: Vec<AccountId>,
	_enable_println: bool,
) -> GenesisConfig {
	// This is the simplest bytecode to revert without returning any data.
	// We will pre-deploy it under all of our precompiles to ensure they can be called from
	// within contracts.
	// (PUSH1 0x00 PUSH1 0x00 REVERT)
	let revert_bytecode = vec![0x60, 0x00, 0x60, 0x00, 0xFD];

	const ENDOWMENT: Balance = 1_000_000_000 * DOLLARS;

	GenesisConfig {
		system: SystemConfig {
			// Add Wasm runtime to storage.
			code: WASM_BINARY.expect("WASM binary is not available.").to_vec(),
		},
		balances: BalancesConfig {
			// Configure endowed accounts with initial balance of 1 << 60.
			balances: endowed_accounts.iter().cloned().map(|k| (k, ENDOWMENT)).collect(),
		},
		aura: AuraConfig {
			authorities: initial_authorities.iter().map(|x| (x.0.clone())).collect(),
		},
		grandpa: GrandpaConfig {
			authorities: initial_authorities.iter().map(|x| (x.1.clone(), 1)).collect(),
		},
		sudo: SudoConfig { key: Some(root_key) },
		transaction_payment: Default::default(),
		evm: EVMConfig {
			// We need _some_ code inserted at the precompile address so that
			// the evm will actually call the address.
			accounts: Precompiles::used_addresses()
				.iter()
				.map(|addr| {
					(
						addr.clone(),
						fp_evm::GenesisAccount {
							nonce: Default::default(),
							balance: Default::default(),
							storage: Default::default(),
							code: revert_bytecode.clone(),
						},
					)
				})
				.collect(),
		},
		ethereum: EthereumConfig {},
		base_fee: BaseFeeConfig::new(
			(100u128 * GIGAWEI).into(),
			false,
			Permill::from_parts(1250u32),
		),
		treasury: Default::default(),
	}
}
