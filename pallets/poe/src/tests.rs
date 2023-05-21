use crate::{mock::*, Error, Event, Proofs};
use frame_support::{assert_noop, assert_ok};
use sp_runtime::BoundedVec;

#[test]
fn create_claim_works() {
	new_test_ext().execute_with(|| {
		let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
		assert_ok!(PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone()));

		assert_eq!(
			Proofs::<Test>::get(&claim),
			Some((1, frame_system::Pallet::<Test>::block_number()))
		);
	});
}

#[test]
fn create_claim_failed_when_claim_already_exist() {
	new_test_ext().execute_with(|| {
		let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
		let _ = PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone());
		assert_noop!(
			PoeModule::create_claim(RuntimeOrigin::signed(1), claim),
			Error::<Test>::ProofAlreadyExist
		);
	});
}

#[test]
fn revoke_claim_works() {
	new_test_ext().execute_with(|| {
		let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
		let origin = RuntimeOrigin::signed(1);
		let _ = PoeModule::create_claim(origin.clone(), claim.clone());
		assert_ok!(PoeModule::revoke_claim(origin, claim.clone()));
		assert_eq!(Proofs::<Test>::get(&claim), None)
	});
}

#[test]
fn revoke_claim_failed_when_claim_is_not_exist() {
	new_test_ext().execute_with(|| {
		let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
		let origin = RuntimeOrigin::signed(1);
		assert_noop!(PoeModule::revoke_claim(origin, claim), Error::<Test>::ClaimNotExist);
	});
}

#[test]
fn revoke_claim_failed_when_claim_with_wrong_owner() {
	new_test_ext().execute_with(|| {
		let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
		let origin = RuntimeOrigin::signed(1);
		let _ = PoeModule::create_claim(origin, claim.clone());
		assert_noop!(
			PoeModule::revoke_claim(RuntimeOrigin::signed(2), claim),
			Error::<Test>::NotClaimOwner
		);
	});
}
