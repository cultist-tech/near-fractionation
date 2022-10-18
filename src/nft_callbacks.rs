use near_sdk::{env, near_bindgen, AccountId, PromiseOrValue};
use near_sdk::serde_json::from_str;
use crate::*;
use mfight_sdk::nft::base::NonFungibleTokenReceiver;
use mfight_sdk::nft_fractionation::{TokenId, FractionationNftOnTransferArgs};

#[near_bindgen]
impl NonFungibleTokenReceiver for Contract {    

    fn nft_on_transfer(
      &mut self,
      sender_id: AccountId,
      _previous_owner_id: AccountId,
      token_id: TokenId,
      msg: String,
    ) -> PromiseOrValue<bool> {
      let nft_contract_id = env::predecessor_account_id();
      let signer_id = env::signer_account_id();

      assert_ne!(
          nft_contract_id,
          signer_id,
          "nft_on_transfer should only be called via cross-contract call"
      );
      assert_eq!(
          &sender_id,
          &signer_id,
          "owner_id should be signer_id"
      );

      let args: FractionationNftOnTransferArgs = from_str(&msg).expect("Not valid FractionationArgs");

      self.nft_fractionation.internal_on_nft_transfer(&args, &nft_contract_id, &token_id, &sender_id);

      PromiseOrValue::Value(false)
    }
}
