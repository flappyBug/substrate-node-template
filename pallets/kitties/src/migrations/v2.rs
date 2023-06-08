use crate::{migrations::versioned_types::*, *};
use frame_support::{
	migration::storage_key_iter, storage::generator::StorageMap, traits::GetStorageVersion,
	weights::Weight, Blake2_128Concat,
};
use frame_system::Pallet;

pub fn migrate<T: Config>() -> Weight {
	let on_chain_version = Pallet::<T>::on_chain_storage_version();
	let current_version = Pallet::<T>::current_storage_version();

	if on_chain_version != 0 {
		return Weight::zero()
	}

	if current_version != 1 {
		return Weight::zero()
	}

	let module = Kitties::<T>::module_prefix();
	let item = Kitties::<T>::storage_prefix();

	for (index, kitty) in
		storage_key_iter::<KittyId, V1Kitty, Blake2_128Concat>(module, item).drain()
	{
		let mut new_name = [0; 8];
		new_name[..4].copy_from_slice(&kitty.name);
		new_name[4..].copy_from_slice(b"__v1");
		let new_kitty: Kitty = Kitty { dna: kitty.dna, name: new_name };
		Kitties::<T>::insert(index, new_kitty);
	}

	Weight::zero()
}
