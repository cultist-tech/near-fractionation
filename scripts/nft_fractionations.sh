#!/bin/bash
source neardev/.env

CONTRACT_ID="$NFT_CONTRACT"
FROM_INDEX="0"
LIMIT=1000

near view $CONTRACT_NAME nft_fractionations "{ \"contract_id\": \"$CONTRACT_ID\", \"limit\": $LIMIT, \"offset\": \"$FROM_INDEX\" }"

