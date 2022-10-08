#!/bin/bash
source neardev/dev-account.env
source neardev/dev-contracts.env

OWNER_ID="$CONTRACT_NAME"

near call $CONTRACT_NAME new_with_default_meta --accountId $CONTRACT_NAME "{ \"owner_id\": \"$OWNER_ID\", \"nft_contract_id\": \"$NFT_CONTRACT\" }"
