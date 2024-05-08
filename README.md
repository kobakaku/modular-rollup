# SSR (Simple Sovereign Rollup)
Reference: https://github.com/Sovereign-Labs/sovereign-sdk

## How to run the rollup using Mock Rollup:

#### 1. Change the working directory:
```shell,test-ci
$ cd demo-rollup
```

#### 2. Start the rollup node:

This will compile and start the rollup node:

```shell
$ cargo run --bin node
```

#### 3. Submit a token creation transaction to the `bank` module:

```sh
$ cargo run --bin wallet rpc submit-batch
```

#### 4. Test if token creation succeeded:

```sh,test-ci
$ curl -X POST -H "Content-Type: application/json" --data '{"jsonrpc":"2.0","method":"bankModule_supply_of","params":{"token_address":"sov1mulxkzakdn42mjj0sn9uxu0avmsy6g87287yzndg6xuy6vw30r0qu82x7a"},"id":1}' http://127.0.0.1:8000 | jq
```

## How to run the rollup using Celestia (under development) :

#### 1. Change the working directory:
```shell,test-ci
$ cd demo-rollup
```

#### 2. If you want to run a fresh rollup, clean the database:

```
$ make clean
```

#### 3. Start the Celestia local docker service:

```
$ make start
```
#### 4. Start the rollup node with the feature flag building with the celestia adapter:

This will compile and start the rollup node:

```shell
$ cargo run --bin node --no-default-features --features celestia_da
```