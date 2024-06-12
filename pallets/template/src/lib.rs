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

		/// The maximum call count.
		#[pallet::constant]
		type MaxCallCount: Get<u32>;
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

	#[pallet::storage]
	pub type RegisteredAccounts<T: Config> = StorageMap< Hasher = Blake2_128Concat, Key = T::AccountId, Value = bool>;


	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Account registered.
		AccountRegistered,

		/// Account deregistered.
		AccountDeregistered,

		/// Call count increased by the given amount.
		CallCountIncreased(u32),
	}

	#[pallet::error]
	pub enum Error<T> {
		/// The maximum call count has been reached.
		MaxCallCountReached,
		/// Account is not registered.
		AccountNotRegistered,
	}

	#[pallet::call]
	impl<T:Config> Pallet<T> {

		/// Register an account
		pub fn register(origin: OriginFor<T>) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			RegisteredAccounts::<T>::insert(&sender, true);

			Self::deposit_event(Event::AccountRegistered);

			Ok(())
		}

		/// Deregister an account
		pub fn deregister(origin: OriginFor<T>) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			RegisteredAccounts::<T>::insert(&sender, false);

			Self::deposit_event(Event::AccountDeregistered);

			Ok(())
		}

		/// Increase the call count by 1
		pub fn increase_call_count(origin: OriginFor<T>) -> DispatchResult {
			let check_account_registered = RegisteredAccounts::<T>::get(&ensure_signed(origin)?).ok_or(Error::<T>::AccountNotRegistered)?;
			ensure!(check_account_registered, Error::<T>::AccountNotRegistered);

			let call_count_in_chain = CallCount::<T>::get();

			let call_count = match call_count_in_chain {
				Some(call_count) => call_count,
				None => 0,
			};

			ensure!(call_count < T::MaxCallCount::get() , Error::<T>::MaxCallCountReached);

			CallCount::<T>::put(call_count + 1);
			Self::deposit_event(Event::CallCountIncreased(call_count + 1));

			Ok(())
		}

	}
}