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

use crate::*;

use kilt_support::deposit::Deposit;
use sp_runtime::traits::Zero;

// Added for consistency even if not (yet) unused.
#[allow(dead_code)]
type OldDidStorage<T> = deprecated::v2::storage::Did<T>;
type OldDidDetails<T> = deprecated::v2::DidDetails<T>;
type NewDidStorage<T> = Did<T>;
type NewDidDetails<T> = DidDetails<T>;

#[cfg(feature = "try-runtime")]
pub(crate) fn pre_migrate<T: Config>() -> Result<(), &'static str> {
	ensure!(
		StorageVersion::<T>::get() == DidStorageVersion::V2,
		"Current deployed version is not v2."
	);

	log::debug!("Version storage migrating from v2 to v3");
	Ok(())
}

pub(crate) fn migrate<T: Config>() -> Weight {
	log::info!("v2 -> v3 DID storage migrator started!");
	let mut total_weight = Weight::zero();

	NewDidStorage::<T>::translate_values(|old_did_details: OldDidDetails<T>| {
		// Add a read from the old storage and a write for the new storage
		total_weight = total_weight.saturating_add(T::DbWeight::get().reads_writes(1, 1));
		Some(old_to_new_did_details(old_did_details))
	});

	StorageVersion::<T>::set(DidStorageVersion::V3);
	// Adds a write from StorageVersion::set() weight.
	total_weight = total_weight.saturating_add(T::DbWeight::get().writes(1));
	log::debug!("Total weight consumed: {}", total_weight);
	log::info!("v2 -> v3 DID storage migrator finished!");

	total_weight
}

fn old_to_new_did_details<T: Config>(old: OldDidDetails<T>) -> NewDidDetails<T> {
	NewDidDetails {
		authentication_key: old.authentication_key,
		key_agreement_keys: old.key_agreement_keys,
		attestation_key: old.attestation_key,
		delegation_key: old.delegation_key,
		public_keys: old.public_keys,
		last_tx_counter: old.last_tx_counter,
		deposit: Deposit::<AccountIdOf<T>, BalanceOf<T>> {
			owner: Default::default(),
			amount: Zero::zero(),
		},
	}
}

#[cfg(feature = "try-runtime")]
pub(crate) fn post_migrate<T: Config>() -> Result<(), &'static str> {
	ensure!(
		StorageVersion::<T>::get() == DidStorageVersion::V3,
		"The version after deployment is not 3 as expected."
	);
	log::debug!("Version storage migrated from v2 to v3");
	Ok(())
}
