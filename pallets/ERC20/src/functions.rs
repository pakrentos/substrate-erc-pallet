use super::*;
// use frame_support::{traits::Get, BoundedVec};

// #[must_use]
// pub(super) enum DeadConsequence {
// 	Remove,
// 	Keep,
// }
use frame_support::pallet_prelude::*;
use sp_runtime::traits::{CheckedAdd, CheckedSub};

// use DeadConsequence::*;

// The main implementation block for the module.
impl<T: Config> Pallet<T> {
	pub fn doApprove(
		sender: &T::AccountId,
		spender: &T::AccountId,
		value: &T::Balance,
	) -> DispatchResultWithPostInfo {
		<Allowances<T>>::insert(sender, spender, value);
		Self::deposit_event(
			Event::Approval(sender.clone(), spender.clone(), value.clone())
		);
		Ok(().into())
	}

	pub fn doSpendAllowance(
		owner: &T::AccountId,
		spender: &T::AccountId,
		amount: &T::Balance
	) -> DispatchResultWithPostInfo {
		let current_allowance = Self::get_allowance(owner, spender);
		let updated_allowance = current_allowance.checked_sub(amount).ok_or(Error::<T>::InsufficientAllowance)?;
		Self::doApprove(owner, spender, &updated_allowance)
	}

	pub fn doTransfer(
		from: &T::AccountId,
		to: &T::AccountId,
		value: &T::Balance,
	) -> DispatchResultWithPostInfo {
		let from_balance = Self::get_balance(from);
		let receiver_balance = Self::get_balance(to);

		// Calculate new balances
		let updated_from_balance = from_balance
			.checked_sub(value)
			.ok_or(Error::<T>::InsufficientFunds)?;
		let updated_to_balance = receiver_balance
			.checked_add(value)
			.expect("Entire supply fits in T::Balance; qed");

		// Write new balances to storage
		<Balances<T>>::insert(from, updated_from_balance);
		<Balances<T>>::insert(to, updated_to_balance);

		Self::deposit_event(Event::Transfer(from.clone(), to.clone(), value.clone()));
		Ok(().into())
	}


}
