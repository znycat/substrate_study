use super::*;
use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok, BoundedVec};

#[test]
fn create_claim_works() {
	new_test_ext().execute_with(|| {
		let origin = RuntimeOrigin::signed(1);
		let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
		assert_ok!(PoeModule::create_claim(origin, claim.clone()));

		assert_eq!(
			Proofs::<Test>::get(&claim),
			Some((1, frame_system::Pallet::<Test>::block_number())),
		)
	});
}

#[test]
fn create_claim_failed_when_claim_already_exist() {
	new_test_ext().execute_with(|| {
		// 验证存证已经存在
		let origin = RuntimeOrigin::signed(1);
		let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
		assert_ok!(PoeModule::create_claim(origin.clone(), claim.clone()));
		assert_noop!(
			PoeModule::create_claim(origin.clone(), claim.clone()),
			Error::<Test>::ProofAlreadyExist
		);
	});
}

#[test]
fn revoke_claim_works() {
	new_test_ext().execute_with(|| {
		// 验证撤销存证
		let origin = RuntimeOrigin::signed(1);
		let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
		assert_ok!(PoeModule::create_claim(origin.clone(), claim.clone()));
		assert_ok!(PoeModule::revoke_claim(origin.clone(), claim.clone()));
	});
}

#[test]
fn revoke_claim_failed_when_claim_is_not_exist() {
	new_test_ext().execute_with(|| {
		// 验证撤销不存在的存证, 返回的应该是错ClaimNotExist
		let origin = RuntimeOrigin::signed(1);
		let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
		assert_noop!(PoeModule::revoke_claim(origin, claim.clone()), Error::<Test>::ClaimNotExist);
	});
}

#[test]
fn revoke_claim_failed_with_wrong_owner() {
	new_test_ext().execute_with(|| {
		// 验证撤销存证并不属于调用人, 返回的应该是NotClaimOwner
		let origin = RuntimeOrigin::signed(1);
		let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
		let _ = PoeModule::create_claim(origin, claim.clone());

		let origin2 = RuntimeOrigin::signed(2);
		assert_noop!(PoeModule::revoke_claim(origin2, claim.clone()), Error::<Test>::NotClaimOwner);
	})
}

#[test]
fn transfer_claim_works() {
	new_test_ext().execute_with(|| {
		// 验证转移存证
		let origin1 = RuntimeOrigin::signed(1);
		let origin2 = RuntimeOrigin::signed(2);
		let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
		// 存证给1
		assert_ok!(PoeModule::create_claim(origin1.clone(), claim.clone()));
		// 转移存证给2
		assert_ok!(PoeModule::transfer_claim(origin1.clone(), 2, claim.clone()));
		// 验证1还有没有存证, 这时候应该是1没有存证,报错NotClaimOwner
		assert_noop!(
			PoeModule::revoke_claim(origin1.clone(), claim.clone()),
			Error::<Test>::NotClaimOwner
		);
		// 验证2拥有存证
		assert_ok!(PoeModule::revoke_claim(origin2.clone(), claim.clone()));
	});
}

#[test]
fn transfer_claim_failed_when_claim_is_not_exist() {
	new_test_ext().execute_with(|| {
		// 验证转移不存在的存证, 返回的应该是错ClaimNotExist

		// 验证转移存证
		let origin1 = RuntimeOrigin::signed(1);
		// let origin2 = RuntimeOrigin::signed(2);
		let claim = BoundedVec::try_from(vec![0, 1]).unwrap();

		// 转移存证给2, 但是存证不存在
		assert_noop!(
			PoeModule::transfer_claim(origin1.clone(), 2, claim.clone()),
			Error::<Test>::ClaimNotExist
		);
	});
}

#[test]
fn transfer_claim_failed_with_wrong_owner() {
	new_test_ext().execute_with(|| {
		// 验证转移存证并不属于调用人, 返回的应该是NotClaimOwner
		let origin1 = RuntimeOrigin::signed(1);
		let origin2 = RuntimeOrigin::signed(2);
		let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
		// 存证给1
		assert_ok!(PoeModule::create_claim(origin1.clone(), claim.clone()));
		// 转移存证从2 给1 ,但是2不是存证的拥有者
		assert_noop!(
			PoeModule::transfer_claim(origin2.clone(), 1, claim.clone()),
			Error::<Test>::NotClaimOwner
		);
	})
}
