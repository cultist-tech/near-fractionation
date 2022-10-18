NFT Fractionation
===================

This repository includes an implementation of a nft-fractionation contract.

Introduction
=============

You collect certain NFTs to exchange them for another certain NFT from any collection. For example: a unique sword is created in the game, but it can only be obtained by collecting all its parts (blade, hilt). This would motivate users to search for the necessary NFT parts, offer to redeem them to the holders of the parts themselves. And the user who collected all the parts and exchanged them for the same unique item can get special opportunities.

Prerequisites
=============

  * Make sure Rust is installed per the prerequisites in [`near-sdk-rs`](https://github.com/near/near-sdk-rs).
  * Make sure [near-cli](https://github.com/near/near-cli) is installed.

Explore this contract
=====================

The source for this contract is in `/src/lib.rs`.

Building this contract
======================
Run the following, and we'll build our rust project up via cargo. This will generate our WASM binaries into our `res/` directory. This is the smart contract we'll be deploying onto the NEAR blockchain later.
```bash
sh ./scripts/build.sh
```

Using this contract
===================

### Quickest deploy

You can build and deploy this smart contract to a development account. [Dev Accounts](https://docs.near.org/docs/concepts/account#dev-accounts) are auto-generated accounts to assist in developing and testing smart contracts. Please see the [Standard deploy](#standard-deploy) section for creating a more personalized account to deploy to.

```bash
sh ./scripts/dev-deploy.sh
```

Behind the scenes, this is creating an account and deploying a contract to it. On the console, notice a message like:

>Done deploying to dev-1234567890123

In this instance, the account is `dev-1234567890123`. A file has been created containing a key pair to
the account, located at `neardev/dev-account`. To make the next few steps easier, we're going to set an
environment variable containing this development account id and use that when copy/pasting commands.
Run this command to set the environment variable:

```bash
source ./scripts/neardev/dev-account.env
```

You can tell if the environment variable is set correctly if your command line prints the account name after this command:
```bash
echo $CONTRACT_NAME
```

The next command will initialize the contract using the `new` method:

```bash
near call $CONTRACT_NAME new_default_meta '{"owner_id": "'$CONTRACT_NAME'"}' --accountId $CONTRACT_NAME
```
