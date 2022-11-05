#!/bin/bash
source neardev/.env

TOKEN_ID="8003"
ACCOUNT_ID="muzikant.testnet"

near call $NFT_CONTRACT nft_transfer_call --accountId $ACCOUNT_ID "{ \"token_id\": \"$TOKEN_ID\", \"receiver_id\": \"$CONTRACT_NAME\",\"msg\": \"{\\\"fractionation_tokens\\\":[\\\"_9\\\", \\\"_5\\\"]}\" }" --depositYocto 1 --gas 40000000000000
