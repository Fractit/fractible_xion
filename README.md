# CosmWasm Starter Pack

This is a template to build smart contracts in Rust to run inside a
[Cosmos SDK](https://github.com/cosmos/cosmos-sdk) module on all chains that enable it.
To understand the framework better, please read the overview in the
[cosmwasm repo](https://github.com/CosmWasm/cosmwasm/blob/master/README.md),
and dig into the [cosmwasm docs](https://www.cosmwasm.com).
This assumes you understand the theory and just want to get coding.

## Creating a new repo from template

Assuming you have a recent version of Rust and Cargo installed
(via [rustup](https://rustup.rs/)),
then the following should get you a new repo to start a contract:

Install [cargo-generate](https://github.com/ashleygwilliams/cargo-generate) and cargo-run-script.
Unless you did that before, run this line now:

```sh
cargo install cargo-generate --features vendored-openssl
cargo install cargo-run-script
```

Now, use it to create your new contract.
Go to the folder in which you want to place it and run:

**Latest**

```sh
cargo generate --git https://github.com/CosmWasm/cw-template.git --name PROJECT_NAME
```

For cloning minimal code repo:

```sh
cargo generate --git https://github.com/CosmWasm/cw-template.git --name PROJECT_NAME -d minimal=true
```

You will now have a new folder called `PROJECT_NAME` (I hope you changed that to something else)
containing a simple working contract and build system that you can customize.

## Create a Repo

After generating, you have a initialized local git repo, but no commits, and no remote.
Go to a server (eg. github) and create a new upstream repo (called `YOUR-GIT-URL` below).
Then run the following:

```sh
# this is needed to create a valid Cargo.lock file (see below)
cargo check
git branch -M main
git add .
git commit -m 'Initial Commit'
git remote add origin YOUR-GIT-URL
git push -u origin main
```

## CI Support

We have template configurations for both [GitHub Actions](.github/workflows/Basic.yml)
and [Circle CI](.circleci/config.yml) in the generated project, so you can
get up and running with CI right away.

One note is that the CI runs all `cargo` commands
with `--locked` to ensure it uses the exact same versions as you have locally. This also means
you must have an up-to-date `Cargo.lock` file, which is not auto-generated.
The first time you set up the project (or after adding any dep), you should ensure the
`Cargo.lock` file is updated, so the CI will test properly. This can be done simply by
running `cargo check` or `cargo unit-test`.

## Using your project

Once you have your custom repo, you should check out [Developing](./Developing.md) to explain
more on how to run tests and develop code. Or go through the
[online tutorial](https://docs.cosmwasm.com/) to get a better feel
of how to develop.

[Publishing](./Publishing.md) contains useful information on how to publish your contract
to the world, once you are ready to deploy it on a running blockchain. And
[Importing](./Importing.md) contains information about pulling in other contracts or crates
that have been published.

Please replace this README file with information about your specific project. You can keep
the `Developing.md` and `Publishing.md` files as useful references, but please set some
proper description in the README.


xiond tx wasm store artifacts/newc.wasm --from xionowner --gas-prices 0.1uxion --gas auto --gas-adjustment 1.3 -y --output json -b block

xiond tx wasm store artifacts/newc.wasm \
  --chain-id xion-local-testnet-1 \
  --gas-adjustment 1.3 \
  --gas-prices 0.001uxion \
  --gas auto \
  --chain-id xion-testnet-1 \
  --node https://rpc.xion-testnet-1.burnt.com:443 \
  --from xion1844yvxc82ar5msd3qap44fjrfxnrxlw8586r7y


  
  protocol
  1216
  xion1azlcqsf8d7x57kfj3ffzal3qw7l2tsag937g5q6nxhgfce06c2jq4zl83t

  xiond tx wasm instantiate 1216 '{"owner": "xion1844yvxc82ar5msd3qap44fjrfxnrxlw8586r7y","denom" :"ibc/57097251ED81A232CE3C9D899E7C8096D6D87EF84BA203E12E424AA4C9B57A64" ,"price" : "1" ,"decimals" : "6" , "max_mint" :"365000000000" }' --from xion1844yvxc82ar5msd3qap44fjrfxnrxlw8586r7y --chain-id xion-testnet-1 --gas auto --gas-adjustment 1.3 --gas-prices 0.001uxion --node https://rpc.xion-testnet-1.burnt.com:443 --label "FracitFirst" --no-admin


  xiond tx wasm execute "xion1azlcqsf8d7x57kfj3ffzal3qw7l2tsag937g5q6nxhgfce06c2jq4zl83t" '{"set_config" : {"protocol_token": "xion1ykp5zsqdn0ffcqygfpnr6l8zkeldy622q56pp7aey6fnkr6vfwts20qknv"} }' --from xion1844yvxc82ar5msd3qap44fjrfxnrxlw8586r7y --chain-id https://rpc.xion-testnet-1.burnt.com:443 --gas auto --gas-prices 0.001uxion --node https://rpc.xion-testnet-1.burnt.com:443

  cw20
  1217
  xion1ykp5zsqdn0ffcqygfpnr6l8zkeldy622q56pp7aey6fnkr6vfwts20qknv


  xiond tx wasm instantiate 1217 '{"name": "Fractible STA","symbol" :"FSTA" ,"decimals" : 6 ,"initial_balances":[],"mint":{"minter":"xion1azlcqsf8d7x57kfj3ffzal3qw7l2tsag937g5q6nxhgfce06c2jq4zl83t"}}' --from xion1844yvxc82ar5msd3qap44fjrfxnrxlw8586r7y --chain-id xion-testnet-1 --gas auto --gas-adjustment 1.3 --gas-prices 0.001uxion --node https://rpc.xion-testnet-1.burnt.com:443 --label "FracitFirstToken" --no-admin