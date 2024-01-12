use frame_support::{
	migration::storage_key_iter, pallet_prelude::*, storage::StoragePrefixedMap,
	traits::GetStorageVersion, weights::Weight, Blake2_128Concat,
};

use crate::{Config, Kitties, Kitty, KittyId, Pallet};

#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct OldKitty(pub [u8; 16]);

pub fn migrate<T: Config>() -> Weight {
	let on_chain_version = Pallet::<T>::on_chain_storage_version();
	let current_version = Pallet::<T>::current_storage_version();

	if on_chain_version != 0 {
		return Weight::zero();
	}

	if current_version != 1 {
		return Weight::zero();
	}

	let module = Kitties::<T>::module_prefix();
	let item = Kitties::<T>::storage_prefix();

	for (index, kitty) in
		storage_key_iter::<KittyId, OldKitty, Blake2_128Concat>(module, item).drain()
	{
		// let kitty = kitty.unwrap();
		// let kitty_id = index as u32;
		let new_kitty = Kitty { dna: kitty.0, name: *b"abcd" };
		Kitties::<T>::insert(index, &new_kitty);
	}
	return Weight::zero();
}
