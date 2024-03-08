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
        dispatch::DispatchResultWithPostInfo,
        pallet_prelude::*,
        traits::{Currency, Randomness, ReservableCurrency},
    };
    use frame_system::pallet_prelude::*;
    use sp_std::vec;
    use sp_std::vec::Vec;

    type BalanceOf<T> =
        <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;
        /// Something that provides randomness in the runtime.
        type Randomness: Randomness<Self::Hash, BlockNumberFor<Self>>;
        type BetAmount: Get<BalanceOf<Self>>;
    }

    #[pallet::pallet]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn game_state)]
    pub type GameState<T> = StorageValue<_, GameStatus, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn player_hands)]
    pub type PlayerHands<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, Vec<Card>, OptionQuery>;

    #[pallet::storage]
    #[pallet::getter(fn deck)]
    pub type Deck<T> = StorageValue<_, Vec<Card>, OptionQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        GameStarted,
        HandDealt(T::AccountId),
        PlayerJoined(T::AccountId),
        PlayerLeft(T::AccountId),
        GameEnded(T::AccountId),
    }

    #[pallet::error]
    pub enum Error<T> {
        NoneValue,
        StorageOverflow,
        GameAlreadyStarted,
        NotEnoughPlayers,
        PlayerAlreadyJoined,
        DeckEmpty,
        BetAmountNotMet,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(Weight::zero())]
        pub fn start_game(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            let _ = ensure_signed(origin)?;
            ensure!(
                GameState::<T>::get() == GameStatus::NotStarted,
                Error::<T>::GameAlreadyStarted
            );
            GameState::<T>::mutate(|state| {
                *state = GameStatus::InProgress;
            });
            Self::deposit_event(Event::GameStarted);
            Ok(().into())
        }

        #[pallet::call_index(1)]
        #[pallet::weight(Weight::zero())]
        pub fn join_game(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            ensure!(
                GameState::<T>::get() == GameStatus::InProgress,
                Error::<T>::NotEnoughPlayers
            );
            ensure!(
                !PlayerHands::<T>::contains_key(&who),
                Error::<T>::PlayerAlreadyJoined
            );
            T::Currency::reserve(&who, T::BetAmount::get())
                .map_err(|_| Error::<T>::BetAmountNotMet)?;
            PlayerHands::<T>::set(&who, Some(vec![]));
            Self::deposit_event(Event::PlayerJoined(who));
            Ok(().into())
        }

        #[pallet::call_index(3)]
        #[pallet::weight(Weight::zero())]
        pub fn deal_hand(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            let _ = ensure_signed(origin)?;
            ensure!(
                GameState::<T>::get() == GameStatus::InProgress,
                Error::<T>::NotEnoughPlayers
            );
            let mut deck = Self::initialize_deck();
            Deck::<T>::put(&deck);
            for _ in 0..2 {
                for (player, _hand) in PlayerHands::<T>::iter() {
                    let card = deck.pop().ok_or(Error::<T>::DeckEmpty)?;
                    PlayerHands::<T>::mutate(&player, |hand| {
                        if let Some(hand) = hand {
                            hand.push(card);
                        } else {
                            *hand = Some(vec![card]);
                        }
                    });
                    Self::deposit_event(Event::HandDealt(player));
                }
            }
            Ok(().into())
        }

        #[pallet::call_index(4)]
        #[pallet::weight(Weight::zero())]
        pub fn end_game(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            let _who = ensure_signed(origin)?;

            ensure!(
                GameState::<T>::get() == GameStatus::InProgress,
                Error::<T>::NotEnoughPlayers
            );

            let winner = Self::evaluate_hands()?;

            Self::deposit_event(Event::GameEnded(winner));

            Ok(().into())
        }
    }

    impl<T: Config> Pallet<T> {
        fn initialize_deck() -> Vec<Card> {
            let mut deck: Vec<Card> = Vec::new();
            let suits = [Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades];
            let values = [
                CardValue::Two,
                CardValue::Three,
                CardValue::Four,
                CardValue::Five,
                CardValue::Six,
                CardValue::Seven,
                CardValue::Eight,
                CardValue::Nine,
                CardValue::Ten,
                CardValue::Jack,
                CardValue::Queen,
                CardValue::King,
                CardValue::Ace,
            ];

            for &suit in &suits {
                for &value in &values {
                    deck.push(Card { suit, value });
                }
            }

            // Shuffle the deck

            deck
        }

        fn evaluate_hands() -> Result<T::AccountId, DispatchError> {
            let mut best_hand: Option<(T::AccountId, HandRank)> = None;

            for (player, hand) in PlayerHands::<T>::iter() {
                let hand_rank = Self::evaluate_hand(&hand);

                if let Some((_, current_best)) = best_hand.clone() {
                    if hand_rank > current_best {
                        best_hand = Some((player.clone(), hand_rank));
                    }
                } else {
                    best_hand = Some((player.clone(), hand_rank));
                }
            }

            best_hand
                .map(|(winner, _)| winner)
                .ok_or(Error::<T>::NotEnoughPlayers.into())
        }

        fn evaluate_hand(_hand: &[Card]) -> HandRank {
            HandRank::HighCard // Placeholder for demonstration purposes
        }
    }

    #[derive(
        Encode, Decode, Clone, PartialEq, Eq, PartialOrd, Ord, RuntimeDebug, MaxEncodedLen, TypeInfo,
    )]
    pub enum HandRank {
        HighCard,
        Pair,
        TwoPairs,
        ThreeOfAKind,
        Straight,
        Flush,
        FullHouse,
        FourOfAKind,
        StraightFlush,
        RoyalFlush,
    }

    // Define game status, card types, etc., here
    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, MaxEncodedLen, TypeInfo)]
    pub enum GameStatus {
        NotStarted,
        InProgress,
        Finished,
    }

    impl Default for GameStatus {
        fn default() -> Self {
            Self::NotStarted
        }
    }

    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
    pub struct Card {
        pub suit: Suit,
        pub value: CardValue,
    }

    #[derive(Copy, Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
    pub enum Suit {
        Hearts,
        Diamonds,
        Clubs,
        Spades,
    }

    #[derive(Copy, Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
    pub enum CardValue {
        Two,
        Three,
        Four,
        Five,
        Six,
        Seven,
        Eight,
        Nine,
        Ten,
        Jack,
        Queen,
        King,
        Ace,
    }
}
