use crate::{migrations::versioned_types::*, *};
use frame_support::{traits::GetStorageVersion, weights::Weight};

pub fn migrate<T: Config>() -> Weight {
	let on_chain_version = Pallet::<T>::on_chain_storage_version();

	if on_chain_version >= 2 {
		log::info!("on chain version is: {on_chain_version:?}, skipping migration to v2");
		return Weight::zero()
	}

	super::v1::migrate::<T>();

	v2::Kitties::<T>::translate::<v1::Kitty, _>(|_, kitty| {
		let mut new_name = [0; 8];
		new_name[..4].copy_from_slice(&kitty.name);
		new_name[4..].copy_from_slice(b"__v1");
		let new_kitty = v2::Kitty { name: new_name, dna: kitty.dna };
		Some(new_kitty)
	});

	Weight::zero()
}
