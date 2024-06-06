//! A shell pallet built with [`frame`].

#![cfg_attr(not(feature = "std"), no_std)]

use frame::prelude::*;

// Re-export all pallet parts, this is needed to properly import the pallet into the runtime.
pub use pallet::*;

#[frame::pallet(dev_mode)]
pub mod pallet {
	use super::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	pub type Balance = u128;

	#[pallet::storage]
	pub type TotalIssuance<T: Config> = StorageValue<_, Balance>;

	#[pallet::storage]
	pub type Balances<T: Config> = StorageMap< Key = T::AccountId, Value = Balance>;

	#[pallet::call]
	impl<T:Config> Pallet<T> {

		pub fn mint_unsafe(
			origin: T::RuntimeOrigin,
			dest: T::AccountId,
			amount: Balance
		) -> DispatchResult {
			let _ = ensure_signed(origin)?;
			
			Balances::<T>::mutate(dest, |bal| *bal = Some(bal.unwrap_or(0).checked_add(amount).expect("BalanceOverflow".into())));
			TotalIssuance::<T>::mutate(|ti| *ti = Some(ti.unwrap_or(0).checked_add(amount).expect("TotalIssuanceOverflow".into())));
			
			Ok(())
		}

		pub fn transfer(
			origin: T::RuntimeOrigin,
			dest: T::AccountId,
			amount: Balance
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			let sender_balance = Balances::<T>::get(&sender).expect("NonExistentAccount".into());
			let remainder = sender_balance.checked_sub(amount).expect("InsufficientBalance".into());

			Balances::<T>::mutate(dest, |bal| *bal = Some(bal.unwrap_or(0).checked_add(amount).expect("BalanceOverflow".into())));
			Balances::<T>::insert(&sender, remainder);

			Ok(())
		}

	}
}