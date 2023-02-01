#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::{inherent::Vec, parameter_types, sp_runtime::RuntimeDebug, BoundedVec};
use scale_info::TypeInfo;

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

parameter_types! {
	///maximum length of tile name
	pub MaxTileNameLen: u32 = 256;
}

/// Legal space unit
#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
pub struct Tile<T: Config> {
	/// Hash for current set of laws in YJML format
	codex: T::Hash,
	/// Parent tile that imposes legal inheritance
	parent: Option<T::Hash>,
        // TODO: add treasury/lock deposit
}

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

	/// Storage for all tiles keyed by tile's ID
	#[pallet::storage]
	#[pallet::getter(fn tile)]
	pub type Tiles<T: Config> = StorageMap<_, Blake2_128Concat, T::Hash, Tile<T>, OptionQuery>;

	/// Storage for legislators chosen in given tile
	#[pallet::storage]
	#[pallet::getter(fn legislator)]
	pub type Legislators<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::Hash,
		Blake2_128Concat,
		T::AccountId,
		(),
		OptionQuery,
	>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored { something: u32, who: T::AccountId },
                /// Successful creation of new tile with [id]
                NewTileDeclared { id: T::Hash },
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
		/// Attempt to create tile that already is registered
		DuplicateTile,
		/// Attempt to state non-existing tile as parent
		OrphanNewTile,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Create a tile entity
		#[pallet::call_index(0)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn new_tile(
			origin: OriginFor<T>,
			tile_id: T::Hash,
			tile_parent: Option<T::Hash>,
			codex: T::Hash,
		) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/main-docs/build/origins/
			let who = ensure_signed(origin)?;

			ensure!(!Tiles::<T>::contains_key(tile_id), Error::<T>::DuplicateTile);

			if let Some(parent) = tile_parent {
				ensure!(Tiles::<T>::contains_key(parent), Error::<T>::OrphanNewTile);
			};

			// create tile
			Tiles::<T>::insert(tile_id, Tile::<T> { codex, parent: tile_parent });

			// add author as rulemaker
			Legislators::<T>::insert(tile_id, who, ());

			// Emit an event.
			Self::deposit_event(Event::NewTileDeclared { id: tile_id });
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}
	}
}
