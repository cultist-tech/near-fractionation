use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::{U128};
use near_sdk::{
    env, near_bindgen, AccountId, PanicOnDefault,
    Promise, PromiseOrValue, BorshStorageKey,
};
use near_sdk::collections::{LookupMap, TreeMap, UnorderedSet};
use std::collections::HashMap;
use mfight_sdk::pause::PauseFeature;
use mfight_sdk::owner::OwnerFeature;
use mfight_sdk::blacklist::BlacklistFeature;
use mfight_sdk::nft_fractionation::{NftFractionationFeature, FractionationId, ContractId, TokenId};

mod nft_callbacks;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
  pause: PauseFeature,
  owner: OwnerFeature,
  blacklist: BlacklistFeature,
  nft_fractionation: NftFractionationFeature,
}

/// Helper structure to for keys of the persistent collections.
#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKey {
  BlacklistAccounts,
  //
  Fractionations,
  FractionationTokens,
  FractionationsIds,
  FractionationsAvailableIds,
  FractionationsCompleted,
  FractionationContractById,
  FractionationTokensPerOwner,
  //
}

#[near_bindgen]
impl Contract {
  #[init]
  pub fn new_with_default_meta(owner_id: AccountId, nft_contract_id: AccountId) -> Self {
    Self::new(
      owner_id,
      nft_contract_id,
    )
  }

  #[init]
  pub fn new(owner_id: AccountId, nft_contract_id: AccountId) -> Self {
    let this = Self {
      pause: PauseFeature::new(),
      owner: OwnerFeature::new(owner_id.clone()),
      blacklist: BlacklistFeature::new(StorageKey::BlacklistAccounts),
      nft_fractionation: NftFractionationFeature::new(
        nft_contract_id,
        StorageKey::Fractionations,
        StorageKey::FractionationTokens,
        StorageKey::FractionationsIds,
        StorageKey::FractionationsCompleted,
        StorageKey::FractionationsAvailableIds,
        StorageKey::FractionationTokensPerOwner,
        StorageKey::FractionationContractById,
      )
    };

    this
  }

  pub fn assert_owner(&self) {
    self.owner.assert_owner();
  }

  #[init(ignore_state)]
  #[private]
  pub fn migrate() -> Self {
    #[derive(BorshDeserialize, BorshSerialize)]
    pub struct OldNftFractionation {
      pub nft_contract_id: AccountId,

      pub fractionation_token_by_id: LookupMap<FractionationId, TokenId>,
      pub fractionation_by_id: TreeMap<FractionationId, UnorderedSet<TokenId>>,
      pub fractionation_ids: UnorderedSet<FractionationId>,
      pub fractionation_available_ids: UnorderedSet<FractionationId>,
      pub fractionation_completed_by_id: LookupMap<FractionationId, u64>,
      pub fractionation_contract_by_id: LookupMap<FractionationId, AccountId>,

      pub tokens_per_owner: LookupMap<AccountId, TreeMap<ContractId, UnorderedSet<TokenId>>>,
    }

    #[derive(BorshDeserialize)]
    struct Old {
      pause: PauseFeature,
      owner: OwnerFeature,
      blacklist: BlacklistFeature,
      nft_fractionation: OldNftFractionation,
    }

    let old: Old = env::state_read().expect("Error");
    let nft_fractionation = NftFractionationFeature {
      nft_contract_id: old.nft_fractionation.nft_contract_id,
      fractionation_token_by_id: old.nft_fractionation.fractionation_token_by_id,
      fractionation_by_id: old.nft_fractionation.fractionation_by_id,
      fractionation_ids: old.nft_fractionation.fractionation_ids,
      fractionation_completed_by_id: old.nft_fractionation.fractionation_completed_by_id,
      tokens_per_owner: old.nft_fractionation.tokens_per_owner,
      fractionation_available_ids: old.nft_fractionation.fractionation_available_ids,
      fractionation_contract_by_id: old.nft_fractionation.fractionation_contract_by_id,
    };

    Self {
      owner: old.owner,
      pause: old.pause,
      blacklist: old.blacklist,
      nft_fractionation,
    }
  }
}

mfight_sdk::impl_pause_feature!(Contract, pause, assert_owner);
mfight_sdk::impl_owner_feature!(Contract, owner);
mfight_sdk::impl_blacklist_feature!(Contract, blacklist, assert_owner);

mfight_sdk::impl_non_fungible_token_fractionation!(Contract, nft_fractionation, assert_owner);

