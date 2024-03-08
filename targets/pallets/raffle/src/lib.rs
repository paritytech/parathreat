// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![cfg_attr(not(feature = "std"), no_std)]

mod benchmarking;
pub mod weights;

use codec::{Decode, Encode};
use frame_support::{
    dispatch::{DispatchResult, GetDispatchInfo},
    ensure,
    storage::bounded_vec::BoundedVec,
    traits::{Currency, ExistenceRequirement::KeepAlive, Get, Randomness, ReservableCurrency},
    PalletId,
};
use frame_system::pallet_prelude::OriginFor;
pub use pallet::*;
use sp_runtime::{
    traits::{AccountIdConversion, Dispatchable, Saturating, Zero},
    ArithmeticError, DispatchError, RuntimeDebug,
};
use sp_std::prelude::*;
pub use weights::WeightInfo;

type BalanceOf<T> =
    <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
type CallID = (u8, u8, u8);
type Ticket = u8;

#[derive(Encode, Decode, Default, Eq, PartialEq, RuntimeDebug, scale_info::TypeInfo)]
pub struct RaffleConfig<BlockNumber, Balance, Account> {
    /// Price per entry.
    price: Balance,
    /// Starting block of the raffle.
    start: BlockNumber,
    /// Length of the raffle (start + length = end).
    length: BlockNumber,
    /// Delay for choosing the winner of the raffle. (start + length + delay = payout).
    /// Randomness in the "payout" block will be used to determine the winner.
    delay: BlockNumber,
    /// The manager of the raffle.
    manager: Account,
    /// The next raffle call to be used if the raffle repeats.
    next_raffle_call: Option<Vec<u8>>,
}

/// Trait for validating calls.
pub trait ValidateCall<T: Config> {
    fn validate_call(encoded_call: Vec<u8>) -> bool;
}

/// Default implementation for validating calls which always fails.
impl<T: Config> ValidateCall<T> for () {
    fn validate_call(_: Vec<u8>) -> bool {
        false
    }
}

/// Pallet implementation for validating calls which uses whitelist to validate calls.
impl<T: Config> ValidateCall<T> for Pallet<T> {
    fn validate_call(encoded_call: Vec<u8>) -> bool {
        let valid_calls = CallIndices::<T>::get();
        let call_id = match Self::call_id(encoded_call) {
            Ok(call_index) => call_index,
            Err(_) => return false,
        };
        valid_calls.iter().any(|id| *id == call_id)
    }
}

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    #[pallet::pallet]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    /// The pallet's config trait.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The Raffle's pallet id
        #[pallet::constant]
        type PalletId: Get<PalletId>;
        /// A dispatchable call.
        type RuntimeCall: Parameter
            + Dispatchable<RuntimeOrigin = Self::RuntimeOrigin>
            + GetDispatchInfo
            + From<frame_system::Call<Self>>;
        /// The currency trait.
        type Currency: ReservableCurrency<Self::AccountId>;
        /// Something that provides randomness in the runtime.
        type Randomness: Randomness<Self::Hash, BlockNumberFor<Self>>;
        /// The overarching event type.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        /// The manager origin.
        type ManagerOrigin: EnsureOrigin<Self::RuntimeOrigin>;
        /// The max number of calls available in a single raffle.
        #[pallet::constant]
        type MaxCalls: Get<u32>;
        /// Used to determine if a call would be valid for purchasing a ticket.
        ///
        /// Be conscious of the implementation used here. We assume at worst that
        /// a vector of `MaxCalls` indices are queried for any call validation.
        /// You may need to provide a custom benchmark if this assumption is broken.
        type ValidateCall: ValidateCall<Self>;
        /// Number of time we should try to generate a random number that has no modulo bias.
        /// The larger this number, the more potential computation is used for picking the winner,
        /// but also the more likely that the chosen winner is done fairly.
        #[pallet::constant]
        type MaxGenerateRandom: Get<u32>;
        /// Weight information for extrinsics in this pallet.
        type WeightInfo: WeightInfo;
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// A raffle has been started!
        RaffleStarted,
        /// A new set of calls have been set!
        CallsUpdated,
        /// A winner has been chosen!
        Winner {
            winner: T::AccountId,
            raffle_balance: BalanceOf<T>,
        },
        /// A ticket has been bought!
        TicketBought { who: T::AccountId },
    }

    #[pallet::error]
    pub enum Error<T> {
        /// A raffle has not been configured.
        NotConfigured,
        /// A raffle is already in progress.
        InProgress,
        /// A raffle has already ended.
        AlreadyEnded,
        /// The call is not valid for an open raffle.
        InvalidCall,
        /// You are already participating in the raffle with this call.
        AlreadyParticipating,
        /// Too many calls for a single raffle.
        TooManyCalls,
        /// Failed to encode calls
        EncodingFailed,
        /// The call could not be decoded.
        UndecodableCall,
    }

    #[pallet::storage]
    pub(crate) type RaffleIndex<T> = StorageValue<_, u32, ValueQuery>;

    /// The configuration for the current raffle.
    #[pallet::storage]
    pub(crate) type Raffle<T: Config> =
        StorageValue<_, RaffleConfig<BlockNumberFor<T>, BalanceOf<T>, T::AccountId>>;

    /// Users who have purchased a ticket. (Raffle Index, Tickets Purchased)
    #[pallet::storage]
    #[pallet::getter(fn participants)]
    pub(crate) type Participants<T: Config> = StorageMap<
        _,
        Twox64Concat,
        T::AccountId,
        (u32, BoundedVec<CallID, T::MaxCalls>),
        ValueQuery,
    >;

    /// Total number of tickets sold.
    #[pallet::storage]
    #[pallet::getter(fn tickets_count)]
    pub type TicketsCount<T> = StorageValue<_, Ticket, ValueQuery>;

    /// Each ticket's owner.
    ///
    /// May have residual storage from previous lotteries. Use `TicketsCount` to see which ones
    /// are actually valid ticket mappings.
    #[pallet::storage]
    #[pallet::getter(fn tickets)]
    pub(crate) type Tickets<T: Config> = StorageMap<_, Twox64Concat, Ticket, T::AccountId>;

    /// The calls stored in this pallet to be used in an active raffle if configured
    /// by `Config::ValidateCall`.
    #[pallet::storage]
    pub(crate) type CallIndices<T: Config> =
        StorageValue<_, BoundedVec<CallID, T::MaxCalls>, ValueQuery>;

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        /// On initialize, check if the raffle has ended and if a winner should be chosen.
        fn on_initialize(n: BlockNumberFor<T>) -> Weight {
            Raffle::<T>::mutate(|mut raffle| -> Weight {
                if let Some(config) = &mut raffle {
                    let payout_block = config
                        .start
                        .saturating_add(config.length)
                        .saturating_add(config.delay);

                    if payout_block <= n {
                        let (_, raffle_balance) = Self::pot();

                        // Randomly choose a winning ticket and return the account that purchased it.
                        // The more tickets an account bought, the higher are its chances of winning.
                        let winner = match Self::choose_ticket(TicketsCount::<T>::get()) {
                            None => None,
                            Some(ticket) => Tickets::<T>::get(ticket),
                        };

                        if let Some(winner) = winner {
                            let res = T::Currency::transfer(
                                &Self::account_id(),
                                &winner,
                                raffle_balance,
                                KeepAlive,
                            );
                            debug_assert!(res.is_ok());

                            Self::deposit_event(Event::<T>::Winner {
                                winner,
                                raffle_balance,
                            });
                        }

                        TicketsCount::<T>::kill();

                        if let Some(encoded_call) = &config.next_raffle_call {
                            if let Ok(call) =
                                <T as Config>::RuntimeCall::decode(&mut &encoded_call[..])
                            {
                                let origin =
                                    frame_system::RawOrigin::Signed(config.manager.clone()).into();

                                assert!(call.dispatch(origin).is_ok(), "Next raffle call failed");

                                return T::WeightInfo::on_initialize_repeat();
                            }
                        } else {
                            *raffle = None;

                            // Else, kill the raffle storage.
                            return T::WeightInfo::on_initialize_end();
                        }
                    }
                }

                T::DbWeight::get().reads(1)
            })
        }
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Play any Casino game and enter the raffle.
        #[pallet::call_index(0)]
        #[pallet::weight(
			T::WeightInfo::play()
		)]
        pub fn play(origin: OriginFor<T>, encoded_call: Vec<u8>) -> DispatchResult {
            let caller = ensure_signed(origin.clone())?;

            ensure!(
                T::ValidateCall::validate_call(encoded_call.clone()),
                Error::<T>::InvalidCall
            );

            let call = <T as Config>::RuntimeCall::decode(
                &mut &encoded_call[..],
            )
            .map_err(|_| Error::<T>::UndecodableCall)?;

            call.clone().dispatch(origin).map_err(|e| e.error)?;

            Self::enter_raffle(
                &caller,
                encoded_call[0],
                encoded_call[1],
                encoded_call.len(),
            )
        }

        /// Set calls in storage which can be used to purchase a raffle ticket.
        ///
        /// This function only matters if you use the `ValidateCall` implementation
        /// provided by this pallet, which uses storage to determine the valid calls.
        ///
        /// This extrinsic must be called by the Manager origin.
        #[pallet::call_index(1)]
        #[pallet::weight(T::WeightInfo::set_calls(calls.len() as u32))]
        pub fn set_calls(
            origin: OriginFor<T>,
            calls: Vec<Box<<T as Config>::RuntimeCall>>,
        ) -> DispatchResult {
            T::ManagerOrigin::ensure_origin(origin)?;
            ensure!(
                calls.len() <= T::MaxCalls::get() as usize,
                Error::<T>::TooManyCalls
            );
            if calls.is_empty() {
                CallIndices::<T>::kill();
            } else {
                // Converts a vector of calls into a vector of call indices.
                let mut indices =
                    BoundedVec::<CallID, T::MaxCalls>::with_bounded_capacity(calls.len());
                for c in calls.iter() {
                    let c = c.encode();
                    let index = Self::call_id(c.to_vec())?;
                    indices
                        .try_push(index)
                        .map_err(|_| Error::<T>::TooManyCalls)?;
                }
                CallIndices::<T>::put(indices);
            }
            Self::deposit_event(Event::<T>::CallsUpdated);
            Ok(())
        }

        /// Start a raffle using the provided configuration.
        ///
        /// This extrinsic must be called by the `ManagerOrigin`.
        ///
        /// Parameters:
        ///
        /// * `price`: The cost of a single ticket.
        /// * `length`: How long the raffle should run for starting at the current block.
        /// * `delay`: How long after the raffle end we should wait before picking a winner.
        /// * `repeat`: If the raffle should repeat when completed.
        #[pallet::call_index(2)]
        #[pallet::weight(T::WeightInfo::start_raffle())]
        pub fn start_raffle(
            origin: OriginFor<T>,
            price: BalanceOf<T>,
            length: BlockNumberFor<T>,
            delay: BlockNumberFor<T>,
            next_raffle_call: Option<Vec<u8>>,
        ) -> DispatchResult {
            T::ManagerOrigin::ensure_origin(origin.clone())?;
            let manager = ensure_signed(origin)?;
            Raffle::<T>::try_mutate(|raffle| -> DispatchResult {
                ensure!(raffle.is_none(), Error::<T>::InProgress);
                let index = RaffleIndex::<T>::get();
                let new_index = index.checked_add(1).ok_or(ArithmeticError::Overflow)?;
                let start = frame_system::Pallet::<T>::block_number();
                // Use new_index to more easily track everything with the current state.
                *raffle = Some(RaffleConfig {
                    price,
                    start,
                    length,
                    delay,
                    manager,
                    next_raffle_call,
                });
                RaffleIndex::<T>::put(new_index);
                Ok(())
            })?;
            // Make sure pot exists.
            let raffle_account = Self::account_id();
            if T::Currency::total_balance(&raffle_account).is_zero() {
                let _ =
                    T::Currency::deposit_creating(&raffle_account, T::Currency::minimum_balance());
            }
            Self::deposit_event(Event::<T>::RaffleStarted);
            Ok(())
        }
    }
}

impl<T: Config> Pallet<T> {
    /// The account ID of the raffle pot.
    ///
    /// This actually does computation. If you need to keep using it, then make sure you cache the
    /// value and only call this once.
    pub fn account_id() -> T::AccountId {
        T::PalletId::get().into_account_truncating()
    }

    /// Return the pot account and amount of money in the pot.
    /// The existential deposit is not part of the pot so raffle account never gets deleted.
    fn pot() -> (T::AccountId, BalanceOf<T>) {
        let account_id = Self::account_id();
        let balance =
            T::Currency::free_balance(&account_id).saturating_sub(T::Currency::minimum_balance());

        (account_id, balance)
    }

    /// Convert a call to it's call index by encoding the call and taking the first two bytes.
    fn call_id(encoded_call: Vec<u8>) -> Result<CallID, DispatchError> {
        if encoded_call.len() < 2 {
            return Err(Error::<T>::EncodingFailed.into());
        }
        Ok((encoded_call[0], encoded_call[1], encoded_call.len() as u8))
    }

    /// Logic for buying a ticket.
    fn enter_raffle(caller: &T::AccountId, pallet: u8, call: u8, len: usize) -> DispatchResult {
        // Check the call is valid raffle
        let config = Raffle::<T>::get().ok_or(Error::<T>::NotConfigured)?;
        let block_number = frame_system::Pallet::<T>::block_number();
        ensure!(
            block_number < config.start.saturating_add(config.length),
            Error::<T>::AlreadyEnded
        );
        let ticket_count = TicketsCount::<T>::get();
        let new_ticket_count = ticket_count + 1;

        // Try to update the participant status
        Participants::<T>::try_mutate(
            caller,
            |(raffle_index, participating_calls)| -> DispatchResult {
                let index = RaffleIndex::<T>::get();
                // If raffle index doesn't match, then reset participating calls and index.
                if *raffle_index != index {
                    *participating_calls = Default::default();
                    *raffle_index = index;
                } else {
                    // Check that user is not already participating under this call.
                    ensure!(
                        !participating_calls
                            .iter()
                            .any(|(p, c, l)| (*p, *c, *l as usize) == (pallet, call, len)),
                        Error::<T>::AlreadyParticipating
                    );
                }
                participating_calls
                    .try_push((pallet, call, len as u8))
                    .map_err(|_| Error::<T>::TooManyCalls)?;
                // Check user has enough funds and send it to the Raffle account.
                T::Currency::transfer(caller, &Self::account_id(), config.price, KeepAlive)?;
                // Create a new ticket.
                TicketsCount::<T>::put(new_ticket_count);
                Tickets::<T>::insert(ticket_count, caller.clone());
                Ok(())
            },
        )?;

        Self::deposit_event(Event::<T>::TicketBought {
            who: caller.clone(),
        });

        Ok(())
    }

    /// Randomly choose a winning ticket from among the total number of tickets.
    /// Returns `None` if there are no tickets.
    pub fn choose_ticket(total: Ticket) -> Option<Ticket> {
        if total == 0 {
            return None;
        }

        let random_number = Self::generate_random_number(0) as Ticket;

        Some(random_number % total)
    }

    /// Generate a random number from a given seed.
    pub fn generate_random_number(seed: u32) -> u32 {
        let (random_seed, _) = T::Randomness::random(&(T::PalletId::get(), seed).encode());
        let random_number = <u32>::decode(&mut random_seed.as_ref())
            .expect("secure hashes should always be bigger than u32; qed");
        random_number
    }
}
