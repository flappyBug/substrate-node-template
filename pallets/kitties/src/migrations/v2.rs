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

#[cfg(test)]
mod test {
	use crate::{migrations::versioned_types, mock::*, Kitties, Kitty, Pallet};
	use frame_support::traits::StorageVersion;

	const DNA: [u8; 16] = *b"testdnatestdna__";
	#[test]
	fn should_migrate_from_v0() {
		new_test_ext().execute_with(|| {
			let kitty_id = 1;
			StorageVersion::new(0).put::<Pallet<Test>>();
			versioned_types::v0::Kitties::<Test>::insert(kitty_id, versioned_types::v0::Kitty(DNA));
			super::migrate::<Test>();
			assert_eq!(
				Kitties::<Test>::get(kitty_id),
				Some(Kitty { name: *b"name__v1", dna: DNA })
			);
		});
	}

	#[test]
	fn should_migrate_from_v1() {
		new_test_ext().execute_with(|| {
			let kitty_id = 1;
			StorageVersion::new(1).put::<Pallet<Test>>();
			versioned_types::v1::Kitties::<Test>::insert(
				kitty_id,
				versioned_types::v1::Kitty { dna: DNA, name: *b"test" },
			);
			super::migrate::<Test>();
			assert_eq!(
				Kitties::<Test>::get(kitty_id),
				Some(Kitty { name: *b"test__v1", dna: DNA })
			);
		});
	}
}
