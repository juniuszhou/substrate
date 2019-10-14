#!/usr/bin/env bash

# /root/parity/substrate/target/release/substrate --chain /root/parity/hardchain.json --base-path /root/parity/chain_data
# --rpc-external=true --rpc-port=8545 --ws-external=true --ws-port=8546 --base-path=/root/parity/chain_data --chain=hardchain
# -l substrate=trace,sync=trace,rpc=trace,consensus=trace > /root/parity/log.txt 2>&1 &

pkill substrate
mv /data/substrate/log.txt /data/substrate/log.txt.bak

../target/release/substrate --base-path /data/substrate/chain_data --chain dev \
--rpc-external=true --rpc-port=8545 --ws-external=true --ws-port=8546 \
-l substrate=trace,sync=trace,rpc=trace,consensus=trace,db=trace,sub-libp2p=trace \
> /data/substrate/log.txt --rpc-cors=all 2>&1 &

## --allow-ips=public --rpc-cors=all

