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

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        dispatch::DispatchResult,
        pallet_prelude::*,
        traits::{Currency, ExistenceRequirement, Randomness, WithdrawReasons},
    };
    use frame_system::pallet_prelude::*;

    type AccountOf<T> = <T as frame_system::Config>::AccountId;
    type BalanceOf<T> = <<T as Config>::Currency as Currency<AccountOf<T>>>::Balance;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type Currency: Currency<Self::AccountId>;
        type Randomness: Randomness<Self::Hash, BlockNumberFor<Self>>;
        #[pallet::constant]
        type BetFee: Get<BalanceOf<Self>>;
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    // Storing bets
    #[pallet::storage]
    #[pallet::getter(fn bets)]
    pub type Bets<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, BalanceOf<T>, ValueQuery>;

    // Events
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        BetPlaced(T::AccountId, BalanceOf<T>),
        BetResult(T::AccountId, BalanceOf<T>, bool), // User, Bet Amount, Win/Lose
    }

    // Errors
    #[pallet::error]
    pub enum Error<T> {
        InsufficientBalance,
    }

    // The pallet's callable functions.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(1)]
        #[pallet::weight(Weight::zero())]
        pub fn bet(
            origin: OriginFor<T>,
            bet_amount: BalanceOf<T>,
            lucky_number: u8,
            tries: u64,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            let current_balance = T::Currency::free_balance(&who);

            // Ensure the user has enough balance to place the bet
            ensure!(
                current_balance >= bet_amount + T::BetFee::get(),
                Error::<T>::InsufficientBalance
            );

            for _ in 0..tries {
                // Deduct the bet fee from the user's balance
                let _ = T::Currency::withdraw(
                    &who,
                    bet_amount + T::BetFee::get(),
                    WithdrawReasons::TRANSFER,
                    ExistenceRequirement::KeepAlive,
                )?;

                let outcome = Self::get_outcome(lucky_number as usize);

                // If win, pay double the bet amount; otherwise, the bet is lost
                if outcome {
                    let _ = T::Currency::deposit_into_existing(&who, bet_amount * 2u32.into())?;
                }

                // Emit event for bet result
                Self::deposit_event(Event::BetResult(who.clone(), bet_amount, outcome));
            }

            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        pub fn get_outcome(lucky_number: usize) -> bool {
            let random_value = T::Randomness::random(&b"slot_machine"[..]).0;
            random_value.encode()[lucky_number] % 2 == 0
        }
    }
}
