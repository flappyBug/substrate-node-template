#![cfg_attr(not(feature = "std"), no_std)]
/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

use sp_core::crypto::KeyTypeId;

pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"btc!");

pub const STORAGE_KEY: &[u8] = b"node-template::rock-paper-scissors";

pub mod crypto {
	use super::KEY_TYPE;
	use sp_core::sr25519::Signature as Sr25519Signature;
	use sp_runtime::{
		app_crypto::{app_crypto, sr25519},
		traits::Verify,
		MultiSignature, MultiSigner,
	};
	app_crypto!(sr25519, KEY_TYPE);

	pub struct TestAuthId;

	impl frame_system::offchain::AppCrypto<MultiSigner, MultiSignature> for TestAuthId {
		type RuntimeAppPublic = Public;
		type GenericSignature = sp_core::sr25519::Signature;
		type GenericPublic = sp_core::sr25519::Public;
	}

	// implemented for mock runtime in test
	impl frame_system::offchain::AppCrypto<<Sr25519Signature as Verify>::Signer, Sr25519Signature>
		for TestAuthId
	{
		type RuntimeAppPublic = Public;
		type GenericSignature = sp_core::sr25519::Signature;
		type GenericPublic = sp_core::sr25519::Public;
	}
}

#[frame_support::pallet]
pub mod pallet {
	use super::STORAGE_KEY;

	use frame_support::{
		pallet_prelude::{DispatchResult, *},
		traits::Hooks,
	};
	use frame_system::{
		ensure_signed,
		offchain::{
			AppCrypto, CreateSignedTransaction, SendUnsignedTransaction, SignedPayload, Signer,
			SigningTypes,
		},
		pallet_prelude::*,
	};
	use sp_io::offchain_index;
	use sp_runtime::offchain::storage;
	#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, scale_info::TypeInfo)]
	pub struct Payload<Public, AccountId> {
		who: AccountId,
		user_hand: Hand,
		system_hand: Hand,
		result: GameResult,
		public: Public,
	}

	impl<T: SigningTypes> SignedPayload<T> for Payload<T::Public, T::AccountId> {
		fn public(&self) -> T::Public {
			self.public.clone()
		}
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config + CreateSignedTransaction<Call<Self>> {
		/// The identifier type for an offchain worker
		type AuthorityId: AppCrypto<Self::Public, Self::Signature>;
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

	#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, scale_info::TypeInfo, Copy)]
	pub enum Hand {
		Rock,
		Paper,
		Scissors,
	}

	#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, scale_info::TypeInfo, Copy)]
	pub enum GameResult {
		Win,
		Lose,
		Tie,
	}

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		UserPlayed { hand: Hand, who: T::AccountId },
		GameResolved { user_hand: Hand, system_hand: Hand, result: GameResult, who: T::AccountId },
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(0)]
		pub fn bet(origin: OriginFor<T>, hand: Hand) -> DispatchResult {
			let who = ensure_signed(origin)?;
			offchain_index::set(STORAGE_KEY, &(who.clone(), hand).encode());
			Self::deposit_event(Event::UserPlayed { hand, who });
			Ok(())
		}

		#[pallet::call_index(2)]
		#[pallet::weight(0)]
		pub fn unsigned_extrinsic_with_signed_payload(
			origin: OriginFor<T>,
			payload: Payload<T::Public, T::AccountId>,
			_signature: T::Signature,
		) -> DispatchResult {
			ensure_none(origin)?;

			log::info!("OCW ==> in call unsigned_extrinsic_with_signed_payload: {:?}", payload);
			Self::deposit_event(Event::GameResolved {
				user_hand: payload.user_hand,
				system_hand: payload.system_hand,
				result: payload.result,
				who: payload.who,
			});
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}
	}

	#[pallet::validate_unsigned]
	impl<T: Config> ValidateUnsigned for Pallet<T> {
		type Call = Call<T>;

		/// Validate unsigned call to this module.
		///
		/// By default unsigned transactions are disallowed, but implementing the validator
		/// here we make sure that some particular calls (the ones produced by offchain worker)
		/// are being whitelisted and marked as valid.
		fn validate_unsigned(_source: TransactionSource, call: &Self::Call) -> TransactionValidity {
			const UNSIGNED_TXS_PRIORITY: u64 = 100;
			let valid_tx = |provide| {
				ValidTransaction::with_tag_prefix("my-pallet")
					.priority(UNSIGNED_TXS_PRIORITY) // please define `UNSIGNED_TXS_PRIORITY` before this line
					.and_provides([&provide])
					.longevity(3)
					.propagate(true)
					.build()
			};

			match call {
				Call::unsigned_extrinsic_with_signed_payload { ref signature, ref payload } => {
					if !SignedPayload::<T>::verify::<T::AuthorityId>(payload, signature.clone()) {
						return InvalidTransaction::BadProof.into()
					}
					valid_tx(b"unsigned_extrinsic_with_signed_payload".to_vec())
				},
				_ => InvalidTransaction::Call.into(),
			}
		}
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		/// Offchain worker entry point.
		fn offchain_worker(_block_number: T::BlockNumber) {
			let mut val_ref: storage::StorageValueRef<'_> =
				storage::StorageValueRef::persistent(STORAGE_KEY);
			if let Ok(Some((who, user_hand))) = val_ref.get::<(T::AccountId, Hand)>() {
				log::info!("OCW ==> user played: {:?}", user_hand);

				let random_slice = sp_io::offchain::random_seed();
				let system_hand = match random_slice[0] % 3 {
					0 => Hand::Rock,
					1 => Hand::Paper,
					2 => Hand::Scissors,
					_ => unreachable!(),
				};
				val_ref.clear();
				log::info!("OCW ==> system plays: {:?}", user_hand);
				let result = match (user_hand, system_hand) {
					(Hand::Rock, Hand::Rock) |
					(Hand::Paper, Hand::Paper) |
					(Hand::Scissors, Hand::Scissors) => GameResult::Tie,
					(Hand::Rock, Hand::Scissors) |
					(Hand::Paper, Hand::Rock) |
					(Hand::Scissors, Hand::Paper) => GameResult::Win,
					_ => GameResult::Lose,
				};
				let signer = Signer::<T, T::AuthorityId>::any_account();

				if let Some((_, res)) = signer.send_unsigned_transaction(
					// this line is to prepare and return payload
					|acct| Payload {
						who: who.clone(),
						user_hand,
						system_hand,
						result,
						public: acct.public.clone(),
					},
					|payload, signature| Call::unsigned_extrinsic_with_signed_payload {
						payload,
						signature,
					},
				) {
					match res {
						Ok(()) => {
							log::info!(
								"OCW ==> unsigned tx with signed payload successfully sent."
							);
						},
						Err(()) => {
							log::error!("OCW ==> sending unsigned tx with signed payload failed.");
						},
					};
				} else {
					// The case of `None`: no account is available for sending
					log::error!("OCW ==> No local account available");
				}
			}
		}
	}
}
