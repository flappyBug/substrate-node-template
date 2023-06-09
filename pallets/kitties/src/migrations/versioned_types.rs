use crate::{Config, KittyId, Pallet};
use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::{storage_alias, Blake2_128Concat, RuntimeDebug};
use scale_info::TypeInfo;

pub mod v0 {

	use super::*;

	#[derive(
		Encode, Decode, Clone, Copy, PartialEq, Eq, Default, TypeInfo, MaxEncodedLen, RuntimeDebug,
	)]
	pub struct Kitty(pub [u8; 16]);

	#[storage_alias]
	pub type Kitties<T: Config> = StorageMap<Pallet<T>, Blake2_128Concat, KittyId, Kitty>;
}

pub mod v1 {
	use super::*;
	#[derive(
		Encode, Decode, Clone, Copy, RuntimeDebug, PartialEq, Eq, Default, TypeInfo, MaxEncodedLen,
	)]
	pub struct Kitty {
		pub name: [u8; 4],
		pub dna: [u8; 16],
	}

	#[storage_alias]
	pub type Kitties<T: Config> = StorageMap<Pallet<T>, Blake2_128Concat, KittyId, Kitty>;
}

pub mod v2 {
	use super::*;
	#[derive(
		Encode, Decode, Clone, Copy, RuntimeDebug, PartialEq, Eq, Default, TypeInfo, MaxEncodedLen,
	)]
	pub struct Kitty {
		pub name: [u8; 8],
		pub dna: [u8; 16],
	}

	#[storage_alias]
	pub type Kitties<T: Config> = StorageMap<Pallet<T>, Blake2_128Concat, KittyId, Kitty>;
}
