//! A shell pallet built with [`frame`].

#![cfg_attr(not(feature = "std"), no_std)]

use frame::prelude::*;

// Re-export all pallet parts, this is needed to properly import the pallet into the runtime.
pub use pallet::*;

#[frame::pallet(dev_mode)]
pub mod pallet {
	use super::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> +  IsType<<Self as frame_system::Config>::RuntimeEvent>;

		// The maximum call count.
		// #[pallet::constant]
		// type MaxCallCount<T>: Get<u32>;
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	pub type Balance = u128;

	#[pallet::storage]
	pub type TotalIssuance<T: Config> = StorageValue<_, Balance>;

	#[pallet::storage]
	pub type Balances<T: Config> = StorageMap< Key = T::AccountId, Value = Balance>;

	#[pallet::storage]
	pub type CallCount<T: Config> = StorageValue<_, u32>;


	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Call count increased by the given amount.
		CallCountIncreased(u32),
	}

	#[pallet::error]
	pub enum Error<T> {
		/// The maximum call count has been reached.
		MaxCallCountReached,
	}

	#[pallet::call]
	impl<T:Config> Pallet<T> {

		/// Increase the call count by 1
		pub fn increase_call_count(origin: OriginFor<T>) -> DispatchResult {
			let _ = ensure_signed(origin)?;

			let call_count = CallCount::<T>::get().unwrap().checked_add(1).ok_or(Error::<T>::MaxCallCountReached)?;
			ensure!(call_count < 10, Error::<T>::MaxCallCountReached);

			CallCount::<T>::put(call_count);
			Self::deposit_event(Event::CallCountIncreased(1));

			Ok(())
		}

	}
}