use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::{U128};
use near_sdk::{
    env, near_bindgen, AccountId, PanicOnDefault,
    BorshStorageKey,
};
use near_sdk::collections::{LookupMap, TreeMap, UnorderedSet};
use mfight_sdk::pause::PauseFeature;
use mfight_sdk::owner::OwnerFeature;
use mfight_sdk::blacklist::BlacklistFeature;
use mfight_sdk::nft_fractionation::{NftFractionationFeature, FractionationId, ContractFractionationId, ContractId, TokenId};

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
  FractionationsOwners,
  Fractionation,
  Fractionations,
  FractionationTokensPerOwner,
}

#[near_bindgen]
impl Contract {
  #[init]
  pub fn new_with_default_meta(owner_id: AccountId) -> Self {
    Self::new(owner_id)
  }

  #[init]
  pub fn new(owner_id: AccountId) -> Self {
    let this = Self {
      pause: PauseFeature::new(),
      owner: OwnerFeature::new(owner_id.clone()),
      blacklist: BlacklistFeature::new(StorageKey::BlacklistAccounts),
      nft_fractionation: NftFractionationFeature::new(
        StorageKey::FractionationsOwners,
        StorageKey::Fractionation,
        StorageKey::Fractionations,
        StorageKey::FractionationTokensPerOwner,
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
      pub fractionations_owners: LookupMap<ContractFractionationId, AccountId>,
      pub fractionation_by_id: TreeMap<ContractFractionationId, UnorderedSet<TokenId>>,
      pub fractionations_by_contract:TreeMap<ContractId, UnorderedSet<FractionationId>>,
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
      fractionations_owners: old.nft_fractionation.fractionations_owners,
      fractionation_by_id: old.nft_fractionation.fractionation_by_id,
      fractionations_by_contract: old.nft_fractionation.fractionations_by_contract,
      tokens_per_owner: old.nft_fractionation.tokens_per_owner,
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

mfight_sdk::impl_fractionation_core!(Contract, nft_fractionation, assert_owner);
mfight_sdk::impl_fractionation_enumeration!(Contract, nft_fractionation);
