#!/usr/bin/env bash

# /root/parity/substrate/target/release/substrate --chain /root/parity/hardchain.json --base-path /root/parity/chain_data
# --rpc-external=true --rpc-port=8545 --ws-external=true --ws-port=8546 --base-path=/root/parity/chain_data --chain=hardchain
# -l substrate=trace,sync=trace,rpc=trace,consensus=trace > /root/parity/log.txt 2>&1 &

# pkill substrate
# mv /Users/junius/data/parity/log.txt /Users/junius/data/parity/log.txt.bak

../../target/release/substrate --base-path /Users/junius/data/parity/chain_data --dev \
-l substrate=trace,sync=trace,rpc=trace,consensus=trace,db=trace,sub-libp2p=trace \
> /Users/junius/data/parity/log.txt 2>&1 &
