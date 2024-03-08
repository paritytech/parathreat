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

use crate::casino::*;

// Raffle E2E Test
//
// Use this test as inspiration for your own exploits. You can use the `run!` macro to simulate
// the passage of time and execute arbitrary code at a specific block number.

#[test]
fn raffle_end_to_end() {
    let calls = vec![
        Box::new(RuntimeCall::Poker(PokerCall::start_game {})),
        Box::new(RuntimeCall::Poker(PokerCall::join_game {})),
    ];
    let mut winner_balance = 0;
    let mut draw_block = 0;

    Casino::execute_with(|| {
        assert_ok!(Raffle::set_calls(RuntimeOrigin::root(), calls));

        let block = System::block_number();
        let price = 10;
        let length = 20;
        let delay = 5;

        winner_balance = Balances::free_balance(&Casino::account_id_of(EVE));
        draw_block = length + delay + block;

        assert_ok!(Raffle::start_raffle(
            RuntimeOrigin::signed(Casino::account_id_of(ALICE)),
            price,
            length,
            delay,
            None,
        ));

        assert_ok!(Raffle::play(
            RuntimeOrigin::signed(Casino::account_id_of(BOB)),
            RuntimeCall::Poker(PokerCall::start_game {}).encode()
        ));
        assert_eq!(Raffle::participants(&Casino::account_id_of(BOB)).1.len(), 1);
        assert_eq!(Raffle::tickets_count(), 1);
        // 1 owns the 0 ticket
        assert_eq!(Raffle::tickets(0), Some(Casino::account_id_of(BOB)));

        // More ticket purchases
        assert_eq!(Balances::free_balance(Casino::account_id_of(EVE)), 4096);
        assert_eq!(Balances::free_balance(Casino::account_id_of(CHARLIE)), 4096);
        assert_eq!(Balances::free_balance(Casino::account_id_of(DAVE)), 4096);

        assert_ok!(Raffle::play(
            RuntimeOrigin::signed(Casino::account_id_of(ALICE)),
            RuntimeCall::Poker(PokerCall::join_game {}).encode()
        ));
        assert_ok!(Raffle::play(
            RuntimeOrigin::signed(Casino::account_id_of(CHARLIE)),
            RuntimeCall::Poker(PokerCall::join_game {}).encode()
        ));
        assert_ok!(Raffle::play(
            RuntimeOrigin::signed(Casino::account_id_of(DAVE)),
            RuntimeCall::Poker(PokerCall::join_game {}).encode()
        ));

        assert_eq!(Raffle::tickets_count(), 4);
    });

    // Go to payout
    run!(draw_block, || {
        // User 1 wins
        assert_expected_events!(
            Casino,
            vec![
                RuntimeEvent::Raffle(pallet_raffle::Event::Winner { .. }) => {},
            ]
        );
    });
}
