pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
pub use pallet_rewards;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{
		pallet_prelude::*,
		traits::{Currency, OnInitialize},
	};
	use frame_system::pallet_prelude::*;
	use sp_runtime::traits::Saturating;

	const BLOCKS_PER_HALVING: u32 = 1_051_200; // ~2 years (at 6s blocks)
	const INITIAL_REWARD: u128 = 250 * 1_000_000_000_000; // 250 ₭
    Rewards: pallet_rewards::{Pallet, Storage, Config<T>, Hooks},

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Currency: Currency<Self::AccountId>;
		type RewardDestination: Get<Self::AccountId>;
        parameter_types! {
            pub const RewardReceiver: AccountId = AccountId::from([0u8; 32]); // Change this later
        }
        
        impl pallet_rewards::Config for Runtime {
            type Currency = Balances;
            type RewardDestination = RewardReceiver;
        }
        
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);
    parameter_types! {
	
}


	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_initialize(n: T::BlockNumber) -> Weight {
			let n_u32: u32 = n.saturated_into();
			let halvings = n_u32 / BLOCKS_PER_HALVING;
			let reward = INITIAL_REWARD >> halvings;

			if reward > 0 {
				let _ = T::Currency::deposit_creating(&T::RewardDestination::get(), reward.into());
			}

			Weight::zero()
		}
	}
}

