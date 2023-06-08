use crate::*;
use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::{
	migration::storage_key_iter, storage::generator::StorageMap, traits::GetStorageVersion,
	weights::Weight, Blake2_128Concat,
};
use frame_system::Pallet;
use scale_info::TypeInfo;

#[derive(Encode, Decode, Clone, Copy, PartialEq, Eq, Default, TypeInfo, MaxEncodedLen)]
pub struct V0Kitty(pub [u8; 16]);

pub fn migrate<T: Config>() -> Weight {
	let on_chain_version = Pallet::<T>::on_chain_storage_version();
	let currenct_version = Pallet::<T>::current_storage_version();

	if on_chain_version != 0 {
		return Weight::zero()
	}

	if currenct_version != 1 {
		return Weight::zero()
	}
	let module = Kitties::<T>::module_prefix();
	let item = Kitties::<T>::storage_prefix();

	for (index, kitty) in
		storage_key_iter::<KittyId, V0Kitty, Blake2_128Concat>(module, item).drain()
	{
		let new_kitty: Kitty = Kitty { dna: kitty.0, name: *b"name" };
		Kitties::<T>::insert(index, new_kitty);
	}

	Weight::zero()
}
