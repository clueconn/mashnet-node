// KILT Blockchain – https://botlabs.org
// Copyright (C) 2019-2021 BOTLabs GmbH

// The KILT Blockchain is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The KILT Blockchain is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// If you feel like getting in touch with us, you can do so at info@botlabs.org

//! Autogenerated weights for did
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2021-09-30, STEPS: {{cmd.steps}}\, REPEAT: {{cmd.repeat}}\, LOW RANGE: {{cmd.lowest_range_values}}\, HIGH RANGE: {{cmd.highest_range_values}}\
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: None, DB CACHE: 128

// Executed Command:
// ./target/release/kilt-parachain
// benchmark
// --steps=1
// --repeat=20
// --pallet=did
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./pallets/did/src/default_weights.rs
// --template=.maintain/weight-template.hbs


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for did.
pub trait WeightInfo {
	fn create_ed25519_keys(n: u32, u: u32, c: u32, ) -> Weight;
	fn create_sr25519_keys(n: u32, u: u32, c: u32, ) -> Weight;
	fn create_ecdsa_keys(n: u32, u: u32, c: u32, ) -> Weight;
	fn delete() -> Weight;
	fn submit_did_call_ed25519_key() -> Weight;
	fn submit_did_call_sr25519_key() -> Weight;
	fn submit_did_call_ecdsa_key() -> Weight;
	fn set_ed25519_authentication_key() -> Weight;
	fn set_sr25519_authentication_key() -> Weight;
	fn set_ecdsa_authentication_key() -> Weight;
	fn set_ed25519_delegation_key() -> Weight;
	fn set_sr25519_delegation_key() -> Weight;
	fn set_ecdsa_delegation_key() -> Weight;
	fn remove_ed25519_delegation_key() -> Weight;
	fn remove_sr25519_delegation_key() -> Weight;
	fn remove_ecdsa_delegation_key() -> Weight;
	fn set_ed25519_attestation_key() -> Weight;
	fn set_sr25519_attestation_key() -> Weight;
	fn set_ecdsa_attestation_key() -> Weight;
	fn remove_ed25519_attestation_key() -> Weight;
	fn remove_sr25519_attestation_key() -> Weight;
	fn remove_ecdsa_attestation_key() -> Weight;
	fn add_ed25519_key_agreement_key() -> Weight;
	fn add_sr25519_key_agreement_key() -> Weight;
	fn add_ecdsa_key_agreement_key() -> Weight;
	fn remove_ed25519_key_agreement_key() -> Weight;
	fn remove_sr25519_key_agreement_key() -> Weight;
	fn remove_ecdsa_key_agreement_key() -> Weight;
	fn reclaim_deposit() -> Weight;
}

/// Weights for did using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	fn create_ed25519_keys(n: u32, u: u32, c: u32, ) -> Weight {
		(94_602_000_u64)
			// Standard Error: 62_000
			.saturating_add((2_271_000_u64).saturating_mul(n as Weight))
			// Standard Error: 2_000
			.saturating_add((24_000_u64).saturating_mul(u as Weight))
			// Standard Error: 282_000
			.saturating_add((393_000_u64).saturating_mul(c as Weight))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn create_sr25519_keys(n: u32, u: u32, c: u32, ) -> Weight {
		(94_170_000_u64)
			// Standard Error: 12_000
			.saturating_add((2_234_000_u64).saturating_mul(n as Weight))
			// Standard Error: 0
			.saturating_add((24_000_u64).saturating_mul(u as Weight))
			// Standard Error: 57_000
			.saturating_add((1_775_000_u64).saturating_mul(c as Weight))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn create_ecdsa_keys(n: u32, u: u32, c: u32, ) -> Weight {
		(208_941_000_u64)
			// Standard Error: 31_000
			.saturating_add((2_189_000_u64).saturating_mul(n as Weight))
			// Standard Error: 1_000
			.saturating_add((19_000_u64).saturating_mul(u as Weight))
			// Standard Error: 143_000
			.saturating_add((1_777_000_u64).saturating_mul(c as Weight))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn delete() -> Weight {
		(26_928_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	fn submit_did_call_ed25519_key() -> Weight {
		(87_031_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn submit_did_call_sr25519_key() -> Weight {
		(90_123_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn submit_did_call_ecdsa_key() -> Weight {
		(205_245_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn set_ed25519_authentication_key() -> Weight {
		(50_165_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn set_sr25519_authentication_key() -> Weight {
		(50_438_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn set_ecdsa_authentication_key() -> Weight {
		(50_013_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn set_ed25519_delegation_key() -> Weight {
		(49_663_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn set_sr25519_delegation_key() -> Weight {
		(50_468_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn set_ecdsa_delegation_key() -> Weight {
		(49_886_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn remove_ed25519_delegation_key() -> Weight {
		(47_499_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn remove_sr25519_delegation_key() -> Weight {
		(46_268_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn remove_ecdsa_delegation_key() -> Weight {
		(46_179_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn set_ed25519_attestation_key() -> Weight {
		(50_087_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn set_sr25519_attestation_key() -> Weight {
		(50_022_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn set_ecdsa_attestation_key() -> Weight {
		(49_883_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn remove_ed25519_attestation_key() -> Weight {
		(45_906_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn remove_sr25519_attestation_key() -> Weight {
		(46_333_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn remove_ecdsa_attestation_key() -> Weight {
		(48_782_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn add_ed25519_key_agreement_key() -> Weight {
		(48_817_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn add_sr25519_key_agreement_key() -> Weight {
		(48_715_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn add_ecdsa_key_agreement_key() -> Weight {
		(49_040_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn remove_ed25519_key_agreement_key() -> Weight {
		(45_987_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn remove_sr25519_key_agreement_key() -> Weight {
		(45_997_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn remove_ecdsa_key_agreement_key() -> Weight {
		(46_479_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn reclaim_deposit() -> Weight {
		(26_928_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	fn create_ed25519_keys(n: u32, u: u32, c: u32, ) -> Weight {
		(94_602_000_u64)
			// Standard Error: 62_000
			.saturating_add((2_271_000_u64).saturating_mul(n as Weight))
			// Standard Error: 2_000
			.saturating_add((24_000_u64).saturating_mul(u as Weight))
			// Standard Error: 282_000
			.saturating_add((393_000_u64).saturating_mul(c as Weight))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	fn create_sr25519_keys(n: u32, u: u32, c: u32, ) -> Weight {
		(94_170_000_u64)
			// Standard Error: 12_000
			.saturating_add((2_234_000_u64).saturating_mul(n as Weight))
			// Standard Error: 0
			.saturating_add((24_000_u64).saturating_mul(u as Weight))
			// Standard Error: 57_000
			.saturating_add((1_775_000_u64).saturating_mul(c as Weight))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	fn create_ecdsa_keys(n: u32, u: u32, c: u32, ) -> Weight {
		(208_941_000_u64)
			// Standard Error: 31_000
			.saturating_add((2_189_000_u64).saturating_mul(n as Weight))
			// Standard Error: 1_000
			.saturating_add((19_000_u64).saturating_mul(u as Weight))
			// Standard Error: 143_000
			.saturating_add((1_777_000_u64).saturating_mul(c as Weight))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	fn delete() -> Weight {
		(26_928_000_u64)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	fn submit_did_call_ed25519_key() -> Weight {
		(87_031_000_u64)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	fn submit_did_call_sr25519_key() -> Weight {
		(90_123_000_u64)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	fn submit_did_call_ecdsa_key() -> Weight {
		(205_245_000_u64)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	fn set_ed25519_authentication_key() -> Weight {
		(50_165_000_u64)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	fn set_sr25519_authentication_key() -> Weight {
		(50_438_000_u64)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	fn set_ecdsa_authentication_key() -> Weight {
		(50_013_000_u64)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	fn set_ed25519_delegation_key() -> Weight {
		(49_663_000_u64)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	fn set_sr25519_delegation_key() -> Weight {
		(50_468_000_u64)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	fn set_ecdsa_delegation_key() -> Weight {
		(49_886_000_u64)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	fn remove_ed25519_delegation_key() -> Weight {
		(47_499_000_u64)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	fn remove_sr25519_delegation_key() -> Weight {
		(46_268_000_u64)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	fn remove_ecdsa_delegation_key() -> Weight {
		(46_179_000_u64)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	fn set_ed25519_attestation_key() -> Weight {
		(50_087_000_u64)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	fn set_sr25519_attestation_key() -> Weight {
		(50_022_000_u64)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	fn set_ecdsa_attestation_key() -> Weight {
		(49_883_000_u64)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	fn remove_ed25519_attestation_key() -> Weight {
		(45_906_000_u64)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	fn remove_sr25519_attestation_key() -> Weight {
		(46_333_000_u64)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	fn remove_ecdsa_attestation_key() -> Weight {
		(48_782_000_u64)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	fn add_ed25519_key_agreement_key() -> Weight {
		(48_817_000_u64)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	fn add_sr25519_key_agreement_key() -> Weight {
		(48_715_000_u64)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	fn add_ecdsa_key_agreement_key() -> Weight {
		(49_040_000_u64)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	fn remove_ed25519_key_agreement_key() -> Weight {
		(45_987_000_u64)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	fn remove_sr25519_key_agreement_key() -> Weight {
		(45_997_000_u64)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	fn remove_ecdsa_key_agreement_key() -> Weight {
		(46_479_000_u64)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	fn reclaim_deposit() -> Weight {
		(26_928_000_u64)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
}
