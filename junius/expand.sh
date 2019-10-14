#!/usr/bin/env bash

# srml
# assets                  balances                council                 executive               indices                 staking                 system
#aura                    consensus               democracy               finality-tracker        metadata                sudo                    timestamp
#babe                    contract                example                 grandpa                 session                 support                 treasury

# single crate
cargo +nightly rustc --profile=check --package <crate-name> --lib -- -Zunstable-options --pretty=expanded > <output-file>

# a lib
cargo +nightly rustc --profile=check --package node-template-runtime --lib -- -Zunstable-options --pretty=expanded > substrate-node-template-runtime.rs

# single module
cargo +nightly rustc --profile=check --package srml-sudo --lib -- -Zunstable-options --pretty=expanded > sudo-module.rs


