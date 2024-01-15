use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok};

#[test]
fn it_works_for_create() {
	new_test_ext().execute_with(|| {
		let kitty_id = 0;
		let account_id: u64 = 0;

		assert_ok!(Balances::force_set_balance(RuntimeOrigin::root(), account_id, 100000000000));

		assert_eq!(KittiesModule::next_kitty_id(), kitty_id);

		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id), *b"1111"));

		System::assert_last_event(
			Event::KittyCreated {
				who: account_id,
				kitty_id,
				kitty: KittiesModule::kitties(kitty_id).unwrap(),
			}
			.into(),
		);

		assert_eq!(KittiesModule::next_kitty_id(), kitty_id + 1);
		assert_eq!(KittiesModule::kitties(kitty_id).is_some(), true);
		assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(account_id));
		assert_eq!(KittiesModule::kitty_parents(kitty_id), None);

		crate::NextKittyId::<Test>::set(crate::KittyId::max_value());
		assert_noop!(
			KittiesModule::create(RuntimeOrigin::signed(account_id), *b"1111"),
			Error::<Test>::InvalidKittyId
		);
	})
}

#[test]
fn it_works_for_bred() {
	new_test_ext().execute_with(|| {
		let kitty_id = 0;
		let account_id = 1;

		assert_ok!(Balances::force_set_balance(RuntimeOrigin::root(), account_id, 100000000000));

		assert_noop!(
			KittiesModule::bred(RuntimeOrigin::signed(account_id), kitty_id, kitty_id, *b"1111"),
			Error::<Test>::SameKittyId
		);

		assert_noop!(
			KittiesModule::bred(RuntimeOrigin::signed(account_id), kitty_id, kitty_id, *b"1111"),
			Error::<Test>::SameKittyId
		);

		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id), *b"1111"));
		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id), *b"2222"));

		assert_eq!(KittiesModule::next_kitty_id(), kitty_id + 2);

		assert_ok!(KittiesModule::bred(
			RuntimeOrigin::signed(account_id),
			kitty_id,
			kitty_id + 1,
			*b"1111"
		));

		System::assert_last_event(
			Event::KittyBred {
				who: account_id,
				kitty_id: kitty_id + 2,
				kitty: KittiesModule::kitties(kitty_id + 2).unwrap(),
			}
			.into(),
		);

		let bred_kitty_id = 2;
		assert_eq!(KittiesModule::next_kitty_id(), bred_kitty_id + 1);
		assert_eq!(KittiesModule::kitties(bred_kitty_id).is_some(), true);
		assert_eq!(KittiesModule::kitty_owner(bred_kitty_id), Some(account_id));
		assert_eq!(KittiesModule::kitty_parents(bred_kitty_id), Some((kitty_id, kitty_id + 1)));
	});
}

#[test]
fn it_works_for_transfer() {
	new_test_ext().execute_with(|| {
		let kitty_id = 0;
		let account_id = 1;
		let recipient = 2;

		assert_ok!(Balances::force_set_balance(RuntimeOrigin::root(), account_id, 100000000000));

		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id), *b"1111"));
		assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(account_id));

		assert_noop!(
			KittiesModule::transfer(RuntimeOrigin::signed(recipient), account_id, kitty_id),
			Error::<Test>::NotKittyOwner
		);

		assert_ok!(KittiesModule::transfer(RuntimeOrigin::signed(account_id), recipient, kitty_id));

		System::assert_last_event(
			Event::KittyTransferred { who: account_id, recipient, kitty_id }.into(),
		);

		assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(recipient));

		assert_ok!(KittiesModule::transfer(RuntimeOrigin::signed(recipient), account_id, kitty_id));

		System::assert_last_event(
			Event::KittyTransferred { who: recipient, recipient: account_id, kitty_id }.into(),
		);

		assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(account_id));
	});
}
