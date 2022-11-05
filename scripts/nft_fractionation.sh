#!/bin/bash
source neardev/.env

TOKEN_ID="4019"
CONTRACT_ID="$NFT_CONTRACT"

near view $CONTRACT_NAME nft_fractionation "{ \"contract_id\": \"$CONTRACT_ID\", \"token_id\": \"$TOKEN_ID\" }"
