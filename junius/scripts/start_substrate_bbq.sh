#!/usr/bin/env bash

# /root/parity/substrate/target/release/substrate --chain /root/parity/hardchain.json --base-path /root/parity/chain_data
# --rpc-external=true --rpc-port=8545 --ws-external=true --ws-port=8546 --base-path=/root/parity/chain_data --chain=hardchain
# -l substrate=trace,sync=trace,rpc=trace,consensus=trace > /root/parity/log.txt 2>&1 &

kill -9 $(pgrep substrate)
rm -rf /data/parity/log.txt

../target/release/substrate --base-path /data/parity/chain_data --chain ./bbq-birch.json \
--rpc-external=true --rpc-port=8545 --ws-external=true --ws-port=8546 \
-l substrate=trace,sync=trace,rpc=trace,consensus=trace,db=trace,sub-libp2p=trace \
> /data/parity/log.txt 2>&1 &
