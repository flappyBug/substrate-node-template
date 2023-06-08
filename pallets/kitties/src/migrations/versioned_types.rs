use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::RuntimeDebug;
use scale_info::TypeInfo;

#[derive(
	Encode, Decode, Clone, Copy, PartialEq, Eq, Default, TypeInfo, MaxEncodedLen, RuntimeDebug,
)]
pub struct V0Kitty(pub [u8; 16]);

#[derive(
	Encode, Decode, Clone, Copy, RuntimeDebug, PartialEq, Eq, Default, TypeInfo, MaxEncodedLen,
)]
pub struct V1Kitty {
	pub name: [u8; 4],
	pub dna: [u8; 16],
}

#[derive(
	Encode, Decode, Clone, Copy, RuntimeDebug, PartialEq, Eq, Default, TypeInfo, MaxEncodedLen,
)]
pub struct V2Kitty {
	pub name: [u8; 8],
	pub dna: [u8; 16],
}
