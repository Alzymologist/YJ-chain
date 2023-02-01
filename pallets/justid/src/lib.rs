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

/// Identity's standing within a tile
#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct Rating {
    /// normal rating
    civil: u32,
    /// how well individual judges
    judical: u32,
    /// how well individual reports violations
    police: u32,
}

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
        use super::*;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
                type Tile;
	}

        /// Storage for membership information
	#[pallet::storage]
	#[pallet::getter(fn something)]
        pub type Ratings<T: Config> = StorageDoubleMap<
            _,
            Blake2_128Concat,
            T::Hash,
            Blake2_128Concat,
            T::AccountId,
            Rating,
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
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
                /// Individual tried to join tile twice
                AlreadyMember,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
	/// Create a tile entity
		#[pallet::call_index(0)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn join(
			origin: OriginFor<T>,
			tile_id: T::Hash,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

                        // TODO: ensure tile exists and accepts neophites
			//ensure!(!Tiles::<T>::contains_key(tile_id), Error::<T>::DuplicateTile);

                        ensure!(!Ratings::<T>::contains_key(tile_id, &who), Error::<T>::AlreadyMember);

			// create tile
			Ratings::<T>::insert(tile_id, who, Rating { civil: 0, judical: 0, police: 0 }); // TODO: fetch values from tile defaults

			// Emit an event.
			// Self::deposit_event(Event::SomethingStored { something, who });
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}
	
		
	}
}
