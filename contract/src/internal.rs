use crate::*;

#[near_bindgen]
impl StakingContract {
  pub(crate) fn internal_register_account(&mut self, account_id: AccountId) {
    let account = Account {
      stake_balance: 0,
      pre_reward: 0,
      last_block_balance_change: env::block_height(),
      unstake_balance: 0,
      unstake_start_timestamp: 0,
      unstake_available_epoch: 0,
      new_account_data: U128(1),
      new_account_data1: U128(2),
    };
    self
      .accounts
      .insert(&account_id, &UpgradableAccount::from(account));
  }

  pub(crate) fn internal_calculate_account_reward(&self, account: &Account) -> Balance {
    let latest_block = if self.paused == true {
      self.pause_in_block
    } else {
      env::block_height()
    };
    let block_diff = latest_block - account.last_block_balance_change;
    let emission_rate =
      self.config.reward_numerator as u128 / self.config.reward_denumerator as u128;
    let reward: Balance = account.stake_balance * emission_rate * block_diff as u128;
    reward
  }

  pub(crate) fn internal_calculate_pool_reward(&self) -> Balance {
    let latest_block = if self.paused == true {
      self.pause_in_block
    } else {
      env::block_height()
    };
    let block_diff = latest_block - self.last_block_balance_change;
    let emission_rate =
      self.config.reward_numerator as u128 / self.config.reward_denumerator as u128;
    let reward: Balance = self.total_stack_balance * emission_rate * block_diff as u128;
    reward
  }
}
