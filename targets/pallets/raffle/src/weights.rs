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

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for pallet_raffle.
pub trait WeightInfo {
	fn play() -> Weight;
	fn set_calls(n: u32, ) -> Weight;
	fn start_raffle() -> Weight;
	fn on_initialize_end() -> Weight;
	fn on_initialize_repeat() -> Weight;
}

/// Weights for pallet_raffle using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: Raffle Raffle (r:1 w:0)
	/// Proof: Raffle Raffle (max_values: Some(1), max_size: Some(29), added: 524, mode: MaxEncodedLen)
	/// Storage: Raffle CallIndices (r:1 w:0)
	/// Proof: Raffle CallIndices (max_values: Some(1), max_size: Some(21), added: 516, mode: MaxEncodedLen)
	/// Storage: Raffle TicketsCount (r:1 w:1)
	/// Proof: Raffle TicketsCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Raffle Participants (r:1 w:1)
	/// Proof: Raffle Participants (max_values: None, max_size: Some(65), added: 2540, mode: MaxEncodedLen)
	/// Storage: Raffle RaffleIndex (r:1 w:0)
	/// Proof: Raffle RaffleIndex (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Raffle Tickets (r:0 w:1)
	/// Proof: Raffle Tickets (max_values: None, max_size: Some(44), added: 2519, mode: MaxEncodedLen)
	fn play() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `452`
		//  Estimated: `3593`
		// Minimum execution time: 60_298_000 picoseconds.
		Weight::from_parts(62_058_000, 3593)
			.saturating_add(T::DbWeight::get().reads(6_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: Raffle CallIndices (r:0 w:1)
	/// Proof: Raffle CallIndices (max_values: Some(1), max_size: Some(21), added: 516, mode: MaxEncodedLen)
	/// The range of component `n` is `[0, 10]`.
	fn set_calls(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 7_291_000 picoseconds.
		Weight::from_parts(8_178_186, 0)
			// Standard Error: 3_048
			.saturating_add(Weight::from_parts(330_871, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: Raffle Raffle (r:1 w:1)
	/// Proof: Raffle Raffle (max_values: Some(1), max_size: Some(29), added: 524, mode: MaxEncodedLen)
	/// Storage: Raffle RaffleIndex (r:1 w:1)
	/// Proof: Raffle RaffleIndex (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn start_raffle() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `161`
		//  Estimated: `3593`
		// Minimum execution time: 36_741_000 picoseconds.
		Weight::from_parts(38_288_000, 3593)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: RandomnessCollectiveFlip RandomMaterial (r:1 w:0)
	/// Proof: RandomnessCollectiveFlip RandomMaterial (max_values: Some(1), max_size: Some(2594), added: 3089, mode: MaxEncodedLen)
	/// Storage: Raffle Raffle (r:1 w:1)
	/// Proof: Raffle Raffle (max_values: Some(1), max_size: Some(29), added: 524, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Raffle TicketsCount (r:1 w:1)
	/// Proof: Raffle TicketsCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Raffle Tickets (r:1 w:0)
	/// Proof: Raffle Tickets (max_values: None, max_size: Some(44), added: 2519, mode: MaxEncodedLen)
	fn on_initialize_end() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `558`
		//  Estimated: `6196`
		// Minimum execution time: 76_611_000 picoseconds.
		Weight::from_parts(78_107_000, 6196)
			.saturating_add(T::DbWeight::get().reads(6_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: RandomnessCollectiveFlip RandomMaterial (r:1 w:0)
	/// Proof: RandomnessCollectiveFlip RandomMaterial (max_values: Some(1), max_size: Some(2594), added: 3089, mode: MaxEncodedLen)
	/// Storage: Raffle Raffle (r:1 w:1)
	/// Proof: Raffle Raffle (max_values: Some(1), max_size: Some(29), added: 524, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Raffle TicketsCount (r:1 w:1)
	/// Proof: Raffle TicketsCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Raffle Tickets (r:1 w:0)
	/// Proof: Raffle Tickets (max_values: None, max_size: Some(44), added: 2519, mode: MaxEncodedLen)
	/// Storage: Raffle RaffleIndex (r:1 w:1)
	/// Proof: Raffle RaffleIndex (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	fn on_initialize_repeat() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `558`
		//  Estimated: `6196`
		// Minimum execution time: 78_731_000 picoseconds.
		Weight::from_parts(80_248_000, 6196)
			.saturating_add(T::DbWeight::get().reads(7_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: Raffle Raffle (r:1 w:0)
	/// Proof: Raffle Raffle (max_values: Some(1), max_size: Some(29), added: 524, mode: MaxEncodedLen)
	/// Storage: Raffle CallIndices (r:1 w:0)
	/// Proof: Raffle CallIndices (max_values: Some(1), max_size: Some(21), added: 516, mode: MaxEncodedLen)
	/// Storage: Raffle TicketsCount (r:1 w:1)
	/// Proof: Raffle TicketsCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Raffle Participants (r:1 w:1)
	/// Proof: Raffle Participants (max_values: None, max_size: Some(65), added: 2540, mode: MaxEncodedLen)
	/// Storage: Raffle RaffleIndex (r:1 w:0)
	/// Proof: Raffle RaffleIndex (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Raffle Tickets (r:0 w:1)
	/// Proof: Raffle Tickets (max_values: None, max_size: Some(44), added: 2519, mode: MaxEncodedLen)
	fn play() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `452`
		//  Estimated: `3593`
		// Minimum execution time: 60_298_000 picoseconds.
		Weight::from_parts(62_058_000, 3593)
			.saturating_add(RocksDbWeight::get().reads(6_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	/// Storage: Raffle CallIndices (r:0 w:1)
	/// Proof: Raffle CallIndices (max_values: Some(1), max_size: Some(21), added: 516, mode: MaxEncodedLen)
	/// The range of component `n` is `[0, 10]`.
	fn set_calls(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 7_291_000 picoseconds.
		Weight::from_parts(8_178_186, 0)
			// Standard Error: 3_048
			.saturating_add(Weight::from_parts(330_871, 0).saturating_mul(n.into()))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: Raffle Raffle (r:1 w:1)
	/// Proof: Raffle Raffle (max_values: Some(1), max_size: Some(29), added: 524, mode: MaxEncodedLen)
	/// Storage: Raffle RaffleIndex (r:1 w:1)
	/// Proof: Raffle RaffleIndex (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn start_raffle() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `161`
		//  Estimated: `3593`
		// Minimum execution time: 36_741_000 picoseconds.
		Weight::from_parts(38_288_000, 3593)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: RandomnessCollectiveFlip RandomMaterial (r:1 w:0)
	/// Proof: RandomnessCollectiveFlip RandomMaterial (max_values: Some(1), max_size: Some(2594), added: 3089, mode: MaxEncodedLen)
	/// Storage: Raffle Raffle (r:1 w:1)
	/// Proof: Raffle Raffle (max_values: Some(1), max_size: Some(29), added: 524, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Raffle TicketsCount (r:1 w:1)
	/// Proof: Raffle TicketsCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Raffle Tickets (r:1 w:0)
	/// Proof: Raffle Tickets (max_values: None, max_size: Some(44), added: 2519, mode: MaxEncodedLen)
	fn on_initialize_end() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `558`
		//  Estimated: `6196`
		// Minimum execution time: 76_611_000 picoseconds.
		Weight::from_parts(78_107_000, 6196)
			.saturating_add(RocksDbWeight::get().reads(6_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	/// Storage: RandomnessCollectiveFlip RandomMaterial (r:1 w:0)
	/// Proof: RandomnessCollectiveFlip RandomMaterial (max_values: Some(1), max_size: Some(2594), added: 3089, mode: MaxEncodedLen)
	/// Storage: Raffle Raffle (r:1 w:1)
	/// Proof: Raffle Raffle (max_values: Some(1), max_size: Some(29), added: 524, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Raffle TicketsCount (r:1 w:1)
	/// Proof: Raffle TicketsCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Raffle Tickets (r:1 w:0)
	/// Proof: Raffle Tickets (max_values: None, max_size: Some(44), added: 2519, mode: MaxEncodedLen)
	/// Storage: Raffle RaffleIndex (r:1 w:1)
	/// Proof: Raffle RaffleIndex (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	fn on_initialize_repeat() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `558`
		//  Estimated: `6196`
		// Minimum execution time: 78_731_000 picoseconds.
		Weight::from_parts(80_248_000, 6196)
			.saturating_add(RocksDbWeight::get().reads(7_u64))
			.saturating_add(RocksDbWeight::get().writes(5_u64))
	}
}
