//! # Migration pallet for runtime
//!
//! This pallet provides functionality for migrating a previous chain-state (possibly from a
//! stand-alone chain) to a new chain state (possbily a parachain now). This pallet is necessary due
//! to the exising boundaries that are put onto runtime upgrades from the relay-chain side.  
#![cfg_attr(not(feature = "std"), no_std)]
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	use sp_runtime::offchain::storage_lock::BlockNumberProvider;
	use frame_support::sp_runtime::traits::Zero;

	#[pallet::pallet]
	#[pallet::generate_store(pub (super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config
	{
		/// The number of blocks this migration should run
		#[pallet::constant]
		type NumBlock: Get<Self::BlockNumber>;

		/// Associated type for Event enum
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

	}

	/// Start block of the last contribution
	#[pallet::storage]
	#[pallet::getter(fn starting_block)]
	pub(super) type StartingBlock<T: Config> = StorageValue<_, T::BlockNumber>;


	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_runtime_upgrade() -> Weight {

			<StartingBlock<T>>::put(frame_system::Pallet::<T>::current_block_number());
			// Block the rest of the block.
			T::BlockWeights::get().max_block
		}

		fn on_initialize(n: T::BlockNumber) -> Weight {
			let starting_block = Pallet::<T>::starting_block().unwrap_or(Zero::zero());

			if starting_block <= n && n <= starting_block + T::NumBlock::get() {
				// Typically, one would access the data from somewhere here and have it chunked automatically
				// depending on the needs. Then inject it into storage.

				Self::deposit_event(Event::MigrationInProgress(n));

				T::BlockWeights::get().max_block
			} else {
				Zero::zero()
			}
		}
	}

	/// Pallet genesis configuration type declaration.
	///
	/// It allows to build genesis storage.
	#[pallet::genesis_config]
	pub struct GenesisConfig {}

	#[cfg(feature = "std")]
	impl Default for GenesisConfig {
		fn default() -> Self {
			Self {}
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig {
		fn build(&self) {}
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Indicates that a migration is happening. Shows remaining blocks
        MigrationInProgress(T::BlockNumber)
	}
}
