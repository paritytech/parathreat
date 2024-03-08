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

#![cfg(not(feature = "runtime-benchmarks"))]
#![allow(dead_code)]

use crate::*;
use casino_runtime::*;
use frame_support::traits::{OnFinalize, OnInitialize};
use network::constants::accounts::{ALICE, BOB, CHARLIE, DAVE, EVE};

pub type SystemCall = frame_system::Call<Runtime>;
pub type RaffleCall = pallet_raffle::Call<Runtime>;
pub type PokerCall = pallet_poker::Call<Runtime>;
pub type SlotsCall = pallet_slots::Call<Runtime>;

mod raffle_exploit_template;
mod raffle;

#[macro_export]
macro_rules! run {
    ($block:expr) => {{
        run!($block, true, || {})
    }};
    ($block:expr, $execute:expr) => {{
        run!($block, true, $execute)
    }};
    ($block:expr, $init_hooks:expr, $execute:expr) => {{
        let mut next = true;
        while next {
            Casino::execute_with(|| {
                if $init_hooks {
                    Raffle::on_initialize(System::block_number());
                    RandomnessCollectiveFlip::on_initialize(System::block_number());
                }

                if System::block_number() == $block {
                    // Note: Using `Box::new` to match the function signature you provided
                    // However, in macro, we directly use the passed expression `$execute`
                    // which should be a closure. It might need adjustments based on the actual use.
                    ($execute)();
                    next = false;
                }

                TransactionPayment::on_finalize(System::block_number());
            });
        }
    }};
}
