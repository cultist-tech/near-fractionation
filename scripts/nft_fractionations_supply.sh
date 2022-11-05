#!/bin/bash
source neardev/.env

CONTRACT_ID="$NFT_CONTRACT"

near view $CONTRACT_NAME nft_fractionations_supply "{ \"contract_id\": \"$CONTRACT_ID\" }"
