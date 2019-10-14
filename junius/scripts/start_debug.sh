#!/usr/bin/env bash

RUST_LOG=debug RUST_BACKTRACE=1 target/release/substrate --dev --rpc-cors=all
