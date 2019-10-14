#!/usr/bin/env bash

# ../../target/release/substrate build-spec > ./mychainspec.json
../../target/release/substrate build-spec --chain dev > ./mychainspec.json
