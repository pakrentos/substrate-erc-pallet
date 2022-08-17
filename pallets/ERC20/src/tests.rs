use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok, storage::bounded_vec::BoundedVec};



#[test]
fn init_works() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		let main_addr = Origin::signed(1);
		// type Vector = BoundedVec<u8, <mock::Test as Trait>::Config::KeyLimit>;
		let test_str =  Vec::<u8>::from("Test");
		assert_ok!(ErcModule::init(main_addr.clone(), 1000_000_000, test_str.clone().try_into().unwrap(), test_str.clone().try_into().unwrap()));
		// Read pallet storage and assert an expected result.
		assert_eq!(ErcModule::get_balance(1), 1000_000_000 as u128);
	});
}

#[test]
fn transfer_works() {
	new_init_test_ext().execute_with(|| {
		let main_addr = Origin::signed(1);
		let pred = ErcModule::get_balance(1);
		assert_ok!(ErcModule::transfer(main_addr, 3, 1000 as u128));
		assert_eq!(ErcModule::get_balance(3), 1000 as u128);
		let succ = ErcModule::get_balance(1);
		assert_eq!(succ, pred - (1000 as u128));
	});
}

#[test]
fn approve_works() {
	new_init_test_ext().execute_with(|| {
		let main_addr = Origin::signed(1);
		let second_addr = Origin::signed(3);
		assert_ok!(ErcModule::approve(main_addr, 3, 1000 as u128));
		assert_eq!(ErcModule::get_allowance(1, 3), 1000 as u128);
		assert_ok!(ErcModule::transferFrom(second_addr, 1, 4, 1000));
		assert_eq!(ErcModule::get_balance(4), 1000 as u128);
		assert_eq!(ErcModule::get_allowance(1, 3), 0 as u128);
	});
}
