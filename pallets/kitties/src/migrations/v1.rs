use crate::{migrations::versioned_types::*, *};
use frame_support::{traits::GetStorageVersion, weights::Weight};

pub fn migrate<T: Config>() -> Weight {
	let on_chain_version = Pallet::<T>::on_chain_storage_version();

	if on_chain_version >= 1 {
		log::info!("on chain version is: {on_chain_version:?}, skipping migration to v1");
		return Weight::zero()
	}

	v1::Kitties::<T>::translate::<v0::Kitty, _>(|_, kitty| {
		let new_kitty = v1::Kitty { dna: kitty.0, name: *b"name" };
		Some(new_kitty)
	});

	Weight::zero()
}
