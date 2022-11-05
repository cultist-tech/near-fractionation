#!/bin/bash
source neardev/.env

CONTRACT_ID="$NFT_CONTRACT"
TOKEN_ID="1"
ACCOUNT_ID='muzikant.testnet'

near call $CONTRACT_NAME nft_fractionation_complete --accountId $ACCOUNT_ID "{ \"contract_id\": \"$CONTRACT_ID\", \"token_id\": \"$TOKEN_ID\" }" --gas 300000000000000
