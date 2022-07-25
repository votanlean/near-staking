use crate::*;

#[derive(Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct PoolJson {
  total_stack_balance: U128,
  total_staker: U128,
  reward: U128,
  paused: bool,
  total_paid_reward_balance: U128,
}

#[near_bindgen]
impl StakingContract {
  pub fn get_account_info(&self, account_id: AccountId) -> JsonAccount {
    let upgradable_account = self.accounts.get(&account_id).unwrap();
    let account = Account::from(upgradable_account);
    let new_reward = self.internal_calculate_account_reward(&account);
    JsonAccount::from(account_id.clone(), new_reward, account)
  }

  pub fn get_account_reward(&self, account_id: AccountId) -> Balance {
    let upgradable_account = self.accounts.get(&account_id).unwrap();
    let account = Account::from(upgradable_account);
    let new_reward = self.internal_calculate_account_reward(&account);
    new_reward + account.pre_reward
  }

  pub fn get_pool_info(&self) -> PoolJson {
    PoolJson {
      total_stack_balance: U128(self.total_stack_balance),
      total_staker: U128::from(self.total_staker),
      reward: U128(self.pre_reward + self.internal_calculate_pool_reward()),
      paused: self.paused,
      total_paid_reward_balance: U128(self.total_paid_reward_balance),
    }
  }
}
