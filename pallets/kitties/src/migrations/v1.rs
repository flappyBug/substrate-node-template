use crate::{migrations::versioned_types::*, *};
use frame_support::{
	migration::storage_key_iter, runtime_print, storage::generator::StorageMap,
	traits::GetStorageVersion, weights::Weight, Blake2_128Concat,
};

pub fn migrate<T: Config>() -> Weight {
	let on_chain_version = Pallet::<T>::on_chain_storage_version();
	let current_version = Pallet::<T>::current_storage_version();
	log::info!("on_chain_version: {on_chain_version:?}, current_version: {current_version:?}");

	if on_chain_version != 0 {
		return Weight::zero()
	}

	if current_version != 1 {
		return Weight::zero()
	}
	let module = Kitties::<T>::module_prefix();
	let item = Kitties::<T>::storage_prefix();

	for (index, kitty) in
		storage_key_iter::<KittyId, V0Kitty, Blake2_128Concat>(module, item).drain()
	{
		let new_kitty: Kitty = Kitty { dna: kitty.0, name: *b"name__v0" };
		Kitties::<T>::insert(index, new_kitty);
	}

	Weight::zero()
}
