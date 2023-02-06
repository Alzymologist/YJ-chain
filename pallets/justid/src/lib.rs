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

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
pub enum Opinion {
    Neutral,
    Endorse,
    Grudge,
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
        /// keys:
        /// [Tile ID, User ID]
	#[pallet::storage]
	#[pallet::getter(fn something)]
        pub type Memberships<T: Config> = StorageDoubleMap<
            _,
            Identity,
            T::Hash,
            Identity,
            T::AccountId,
            (),
            OptionQuery,
        >;

        #[pallet::type_value]
        pub fn DefaultOpinion() -> Opinion { Opinion::Neutral }

        /// Storage for inte-user relations
        /// keys:
        /// [Tile ID, Who stated opinion, About whom]
        /// value:
        /// Opinion struct with 3 states, for simplicity
        #[pallet::storage]
        pub type PeerOpinion<T: Config> = StorageNMap<
            _,
            (
                NMapKey<Identity, T::Hash>,
                NMapKey<Identity, T::AccountId>,
                NMapKey<Identity, T::AccountId>,
            ),
            Opinion,
            ValueQuery,
            DefaultOpinion,
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
        	/// Join a tile
		#[pallet::call_index(0)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn join(
			origin: OriginFor<T>,
			tile_id: T::Hash,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

                        // TODO: ensure tile exists and accepts neophites
			//ensure!(!Tiles::<T>::contains_key(tile_id), Error::<T>::DuplicateTile);


                        ensure!(!Memberships::<T>::contains_key(tile_id, &who), Error::<T>::AlreadyMember);

			// Join tile
		        Memberships::<T>::insert(tile_id, who, ());

			// Emit an event.
			// Self::deposit_event(Event::SomethingStored { something, who });
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

                /*
		#[pallet::call_index(1)]
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
*/
	}
}