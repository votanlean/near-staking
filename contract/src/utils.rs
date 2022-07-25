use crate::*;

pub(crate) fn assert_at_least_one_yocto() {
  assert!(env::attached_deposit() >= 1, "assert_at_least_one_yocto");
}

pub(crate) fn refund_deposit(used_storage: u64) {
  let attached_deposit = env::attached_deposit();
  let storage_cost = env::storage_byte_cost() * Balance::from(used_storage);
  assert!(attached_deposit >= storage_cost, "not enough deposit fund, need {}", storage_cost);
  let refund = attached_deposit - storage_cost;
  if refund > 0 {
    Promise::new(env::predecessor_account_id()).transfer(refund);
  }
}