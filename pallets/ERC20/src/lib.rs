#![cfg_attr(not(feature = "std"), no_std)]
// #![allow(clippy::unused_unit)]

//! Simple Token Transfer
//! 1. set total supply
//! 2. establish ownership upon configuration of circulating tokens
//! 3. coordinate token transfers with the runtime functions
mod functions;

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;


#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use sp_runtime::traits::{AtLeast32BitUnsigned, CheckedAdd, CheckedSub};


	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type Balance: Member
		+ Parameter
		+ AtLeast32BitUnsigned
		+ Default
		+ Copy
		+ MaybeSerializeDeserialize
		+ MaxEncodedLen
		+ CheckedAdd
		+ CheckedSub
		+ TypeInfo;

		#[pallet::constant]
		type KeyLimit: Get<u32>;

	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		Initialized(T::AccountId),
		Transfer(T::AccountId, T::AccountId, T::Balance), // (from, to, value)
		Approval(T::AccountId, T::AccountId, T::Balance), // (owner, spender, amount)
		Balance(T::Balance),
	}

	#[pallet::type_value]
	pub(super) fn EmptyBalance<T: Config>() -> T::Balance { (0 as u32).into() }

	#[pallet::storage]
	#[pallet::getter(fn get_balance)]
	pub(super) type Balances<T: Config> =
	StorageMap<_, Blake2_128Concat, T::AccountId, T::Balance, ValueQuery, EmptyBalance<T>>;

	#[pallet::storage]
	#[pallet::getter(fn get_allowance)]
	pub(super) type Allowances<T: Config> =
	StorageDoubleMap<_, Blake2_128Concat, T::AccountId, Blake2_128Concat, T::AccountId, T::Balance, ValueQuery, EmptyBalance<T>>;

	#[pallet::storage]
	#[pallet::getter(fn _name)]
	pub(super) type Name<T: Config> =
	StorageValue<_, BoundedVec<u8, T::KeyLimit>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn _symbol)]
	pub(super) type Symbol<T: Config> =
	StorageValue<_, BoundedVec<u8, T::KeyLimit>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn total_supply)]
	pub(super) type TotalSupply<T: Config> =
	StorageValue<_, T::Balance, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn is_init)]
	pub(super) type Init<T: Config> = StorageValue<_, bool, ValueQuery>;

	#[pallet::pallet]
	#[pallet::generate_store(pub (super) trait Store)]
	pub struct Pallet<T>(PhantomData<T>);

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::error]
	pub enum Error<T> {
		AlreadyInitialized,
		NotInitialized,
		InsufficientFunds,
		InsufficientAllowance,
		Overflow
	}

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub balances: Vec<(T::AccountId, T::Balance)>,
		pub allowances: Vec<(T::AccountId, T::AccountId, T::Balance)>,
		pub name: Vec<u8>,
		pub symbol: Vec<u8>,
		pub total_supply: T::Balance,
		pub init: bool,
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self {
				balances: Default::default(),
				allowances: Default::default(),
				name: Default::default(),
				symbol: Default::default(),
				total_supply: (0 as u32).into(),
				init: false,
			}
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			let name: &BoundedVec<u8, T::KeyLimit> = &self.name.clone().try_into().expect("Token name is too long");
			let symbol: &BoundedVec<u8, T::KeyLimit> = &self.symbol.clone().try_into().expect("Token symbol is too long");
			Name::<T>::put(name.clone());
			Symbol::<T>::put(symbol.clone());
			Init::<T>::put(&self.init);
			TotalSupply::<T>::put(&self.total_supply.clone());

			let mut total_balance: T::Balance = (0 as u32).into();
			for (acc, balance) in &self.balances {
				// this should panic in case of overflow
				total_balance = total_balance.checked_add(balance).unwrap();
				assert!(&total_balance <= &self.total_supply, "Overall balance must not exceed the set TotalBalance");
				assert!(!Balances::<T>::contains_key(acc), "Account id already in use");
				Balances::<T>::insert(acc, balance.clone());
			}

			for (owner, spender, balance) in &self.allowances {
				assert!(!Allowances::<T>::contains_key(owner, spender), "Account ids already in use");
				Allowances::<T>::insert(owner, spender, balance.clone());
			}
		}
	}

	#[cfg(feature = "std")]
	impl<T: Config> GenesisConfig<T> {
		pub fn build_storage(&self) -> Result<sp_runtime::Storage, String> {
			<Self as GenesisBuild<T>>::build_storage(self)
		}

		pub fn assimilate_storage(&self, storage: &mut sp_runtime::Storage) -> Result<(), String> {
			<Self as GenesisBuild<T>>::assimilate_storage(self, storage)
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Initialize the token
		/// transfers the total_supply amount to the caller
		#[pallet::weight(10_000)]
		pub fn init(
			_origin: OriginFor<T>,
			totalSupply: T::Balance,
			name: BoundedVec<u8, T::KeyLimit>,
			symbol: BoundedVec<u8, T::KeyLimit>
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(_origin)?;
			ensure!(!Self::is_init(), Error::<T>::AlreadyInitialized);
			<TotalSupply<T>>::put(totalSupply);
			<Balances<T>>::insert(sender, Self::total_supply());
			<Name<T>>::put(name);
			<Symbol<T>>::put(symbol);

			Init::<T>::put(true);
			Ok(().into())
			}


		#[pallet::weight(10_000)]
		pub fn approve(
			_origin: OriginFor<T>,
			spender: T::AccountId,
			amount: T::Balance,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(_origin)?;
			ensure!(Self::is_init(), Error::<T>::NotInitialized);
			Self::doApprove(&sender, &spender, &amount)
			}

		/// Transfer tokens from one account to another
		#[pallet::weight(10_000)]
		pub fn transfer(
			_origin: OriginFor<T>,
			to: T::AccountId,
			amount: T::Balance,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(_origin)?;
			ensure!(Self::is_init(), Error::<T>::NotInitialized);
			Self::doTransfer(&sender, &to, &amount)
			}


		#[pallet::weight(10_000)]
		pub fn transferFrom(
			_origin: OriginFor<T>,
			from: T::AccountId,
			to: T::AccountId,
			amount: T::Balance
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(_origin)?;
			ensure!(Self::is_init(), Error::<T>::NotInitialized);
			Self::doSpendAllowance(&from, &sender, &amount)?;
			Self::doTransfer(&from, &to, &amount)
			}

		#[pallet::weight(10_000)]
		pub fn increaseAllowance(
			_origin: OriginFor<T>,
			spender: T::AccountId,
			amount: T::Balance
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(_origin)?;
			ensure!(Self::is_init(), Error::<T>::NotInitialized);
			let current_allowance = Self::get_allowance(&sender, &spender);
			Self::doApprove(&sender, &spender, &amount)
			}

		#[pallet::weight(10_000)]
		pub fn decreaseAllowance(
			_origin: OriginFor<T>,
			spender: T::AccountId,
			amount: T::Balance
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(_origin)?;
			ensure!(Self::is_init(), Error::<T>::NotInitialized);
			let current_allowance = Self::get_allowance(&sender, &spender);
			Self::doApprove(&sender, &spender, &amount)
			}

		}
}
