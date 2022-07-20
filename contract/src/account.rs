use near_sdk::BlockHeight;
use near_sdk::EpochHeight;
use near_sdk::Timestamp;
use crate::*;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Account {
  pub stake_balance: Balance,
  pub pre_reward: Balance,
  pub last_block_balance_change: BlockHeight,
  pub unstake_balance: Balance,
  pub unstake_start_timestamp: Timestamp,
  pub unstake_available_epoch: EpochHeight
}