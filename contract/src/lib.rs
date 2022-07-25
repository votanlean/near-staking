use near_sdk::collections::LookupMap;
use near_sdk::json_types::U128;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, AccountId, Timestamp, Balance, BlockHeight, EpochHeight, near_bindgen, PanicOnDefault, BorshStorageKey, Promise, PromiseOrValue};


use crate::config::*;
use crate::account::*;
pub use crate::account::JsonAccount;
use crate::utils::*;
use crate::internal::*;
use enumeration::*;
mod config;
mod account;
mod utils;
mod internal;
mod enumeration;

#[derive(BorshDeserialize, BorshSerialize, BorshStorageKey)]
pub enum StorageKey {
    AccountKey
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct StakingContract {
    pub owner_id: AccountId,
    pub ft_contract_id: AccountId,
    pub config: Config,
    pub total_stack_balance: Balance,
    pub total_paid_reward_balance: Balance,
    pub total_staker: Balance,
    pub pre_reward: Balance,
    pub last_block_balance_change: BlockHeight,
    pub accounts: LookupMap<AccountId, Account>,
    pub paused: bool,
    pub pause_in_block: BlockHeight
}

#[near_bindgen]
impl StakingContract {

    #[init]
    pub fn new_default_config(owner_id: AccountId, ft_contract_id: AccountId) -> Self {
        Self::new(owner_id, ft_contract_id, Config::default())
    }

    #[init]
    pub fn new(owner_id: AccountId, ft_contract_id: AccountId, config: Config) -> Self {
        StakingContract { owner_id,
            ft_contract_id,
            config,
            total_stack_balance: 0,
            total_paid_reward_balance: 0,
            total_staker: 0,
            pre_reward: 0,
            last_block_balance_change: env::block_height(),
            accounts: LookupMap::new(StorageKey::AccountKey),
            paused: false,
            pause_in_block: 0, 
        }
    }

    #[payable]
    pub fn create_account(&mut self, account_id: Option<AccountId>) {//is caller may or may not pass account_id, hence use Option. AccountId is truct constructed from account string
        assert_at_least_one_yocto();
        let account = account_id.unwrap_or_else(|| env::predecessor_account_id());
        let staked_account = self.accounts.get(&account);
        if staked_account.is_some() {
            refund_deposit(0);
        } else {
            let before_storage_usage = env::storage_usage();
            self.internal_register_account(account.clone());
            let after_storage_usage = env::storage_usage();
            refund_deposit(after_storage_usage - before_storage_usage);
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::{testing_env};
    use near_sdk::test_utils::{VMContextBuilder, accounts};

    fn get_context (is_view: bool) -> VMContextBuilder{
        let mut builder = VMContextBuilder::new();
        builder.current_account_id(accounts(0))
        .signer_account_id(accounts(0))
        .predecessor_account_id(accounts(0))
        .is_view(is_view);

        builder
    }

    #[test]
    fn test_init_contract() {
        let context = get_context(false);
        testing_env!(context.build());

        let config: Config = Config { reward_numerator: 500, reward_denumerator: 100000 };

        let contract = StakingContract::new(accounts(1), AccountId::new_unchecked("ft_contract".to_string()), config);

        assert_eq!(contract.owner_id.to_string(), accounts(1).to_string());
        assert_eq!(contract.ft_contract_id.to_string(), "ft_contract".to_string());
        assert_eq!(contract.config.reward_numerator, config.reward_numerator);
        assert_eq!(contract.paused, false);

    }
}

