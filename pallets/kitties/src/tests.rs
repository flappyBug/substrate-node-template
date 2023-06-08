use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok, traits::Currency};
use pallet_balances::Error as BalanceError;
use sp_runtime::traits::AccountIdConversion;

fn assert_balance(account_id: u64, balance: u128) {
	assert_eq!(Balances::total_balance(&account_id), balance);
}

#[test]
fn create_kitty_works() {
	new_test_ext().execute_with(|| {
		let kitty_id = 0;
		let account_id = 1;
		let node_account = AccountIdConversion::<u64>::into_account_truncating(&PALLET_ID);
		assert_eq!(KittiesModule::next_kitty_id(), kitty_id);
		assert_noop!(
			KittiesModule::create(RuntimeOrigin::signed(account_id)),
			BalanceError::<Test>::InsufficientBalance
		);

		let _ = Balances::deposit_creating(&account_id, KITTY_PRICE + EXISTENTIAL_DEPOSIT);
		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id)));

		assert_balance(account_id, EXISTENTIAL_DEPOSIT);
		assert_balance(node_account, KITTY_PRICE);

		System::assert_last_event(RuntimeEvent::KittiesModule(Event::KittyCreated {
			who: account_id,
			kitty_id,
			kitty: KittiesModule::kitties(kitty_id).unwrap(),
		}));

		assert_eq!(KittiesModule::next_kitty_id(), kitty_id + 1);
		assert!(KittiesModule::kitties(kitty_id).is_some());
		assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(account_id));
		assert_eq!(KittiesModule::kitty_parents(kitty_id), None);

		crate::NextKittyId::<Test>::set(crate::KittyId::max_value());
		assert_noop!(
			KittiesModule::create(RuntimeOrigin::signed(account_id)),
			Error::<Test>::InvalidKittyId
		);
	});
}

#[test]
fn it_works_for_breed() {
	new_test_ext().execute_with(|| {
		let kitty_id = 0;
		let account_id = 1;
		let node_account = AccountIdConversion::<u64>::into_account_truncating(&PALLET_ID);

		assert_noop!(
			KittiesModule::breed(RuntimeOrigin::signed(account_id), kitty_id, kitty_id),
			Error::<Test>::SameKittyId
		);

		assert_noop!(
			KittiesModule::breed(RuntimeOrigin::signed(account_id), kitty_id, kitty_id + 1),
			Error::<Test>::InvalidKittyId
		);

		let _ = Balances::deposit_creating(&account_id, KITTY_PRICE * 3 + EXISTENTIAL_DEPOSIT);

		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id)));
		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id)));

		assert_eq!(KittiesModule::next_kitty_id(), kitty_id + 2);

		assert_balance(account_id, KITTY_PRICE + EXISTENTIAL_DEPOSIT);
		assert_balance(node_account, KITTY_PRICE * 2);

		assert_ok!(KittiesModule::breed(RuntimeOrigin::signed(account_id), kitty_id, kitty_id + 1));

		assert_balance(account_id, EXISTENTIAL_DEPOSIT);
		assert_balance(node_account, KITTY_PRICE * 3);

		let breed_kitty_id = 2;

		System::assert_last_event(RuntimeEvent::KittiesModule(Event::KittyBred {
			who: account_id,
			kitty_id: breed_kitty_id,
			kitty: KittiesModule::kitties(breed_kitty_id).unwrap(),
		}));

		assert_eq!(KittiesModule::next_kitty_id(), breed_kitty_id + 1);
		assert!(KittiesModule::kitties(breed_kitty_id).is_some());
		assert_eq!(KittiesModule::kitty_owner(breed_kitty_id), Some(account_id));
		assert_eq!(KittiesModule::kitty_parents(breed_kitty_id), Some((kitty_id, kitty_id + 1)));
	})
}

#[test]
fn it_works_for_transfer() {
	new_test_ext().execute_with(|| {
		let kitty_id = 0;
		let account_id = 1;
		let recipient = 2;

		let _ = Balances::deposit_creating(&account_id, KITTY_PRICE + EXISTENTIAL_DEPOSIT);
		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id)));
		assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(account_id));

		assert_noop!(
			KittiesModule::transfer(RuntimeOrigin::signed(recipient), account_id, kitty_id),
			Error::<Test>::NotOwner
		);

		assert_ok!(KittiesModule::transfer(RuntimeOrigin::signed(account_id), recipient, kitty_id));

		System::assert_last_event(RuntimeEvent::KittiesModule(Event::KittyTransfered {
			who: account_id,
			recipient,
			kitty_id,
		}));
		assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(recipient));

		assert_ok!(KittiesModule::transfer(RuntimeOrigin::signed(recipient), account_id, kitty_id));

		System::assert_last_event(RuntimeEvent::KittiesModule(Event::KittyTransfered {
			who: recipient,
			recipient: account_id,
			kitty_id,
		}));
		assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(account_id));
	})
}

#[test]
fn it_works_for_sale() {
	new_test_ext().execute_with(|| {
		let kitty_id = 0;
		let account_id = 1;

		assert_noop!(
			KittiesModule::sale(RuntimeOrigin::signed(account_id), kitty_id),
			Error::<Test>::InvalidKittyId
		);

		let _ = Balances::deposit_creating(&account_id, KITTY_PRICE + EXISTENTIAL_DEPOSIT);

		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id)));
		assert_ok!(KittiesModule::sale(RuntimeOrigin::signed(account_id), kitty_id));
		assert!(KittiesModule::kitty_on_sale(kitty_id).is_some());
		assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(account_id));

		System::assert_last_event(RuntimeEvent::KittiesModule(Event::KittyOnSale {
			who: account_id,
			kitty_id,
		}));

		assert_noop!(
			KittiesModule::sale(RuntimeOrigin::signed(account_id), kitty_id),
			Error::<Test>::AlreadyOnSale
		);
	})
}

#[test]
fn it_works_for_buy() {
	new_test_ext().execute_with(|| {
		let kitty_id = 0;
		let seller_id = 1;
		let buyer_id = 2;

		assert_noop!(
			KittiesModule::buy(RuntimeOrigin::signed(buyer_id,), kitty_id),
			Error::<Test>::NotExist
		);
		let _ = Balances::deposit_creating(&seller_id, KITTY_PRICE + EXISTENTIAL_DEPOSIT);
		let _ = Balances::deposit_creating(&buyer_id, KITTY_PRICE + EXISTENTIAL_DEPOSIT);

		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(seller_id)));
		assert_ok!(KittiesModule::sale(RuntimeOrigin::signed(seller_id), kitty_id));

		assert_ok!(KittiesModule::buy(RuntimeOrigin::signed(buyer_id,), kitty_id),);
		assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(buyer_id));
		assert_balance(seller_id, EXISTENTIAL_DEPOSIT + KITTY_PRICE);
		assert_balance(buyer_id, EXISTENTIAL_DEPOSIT);
		System::assert_last_event(RuntimeEvent::KittiesModule(Event::KittyBought {
			who: buyer_id,
			kitty_id,
		}));
		assert_noop!(
			KittiesModule::buy(RuntimeOrigin::signed(buyer_id), kitty_id),
			Error::<Test>::AlreadyOwned
		);
	})
}
