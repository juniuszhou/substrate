#!/usr/bin/env bash

# /root/parity/substrate/target/release/substrate --chain /root/parity/hardchain.json --base-path /root/parity/chain_data
# --rpc-external=true --rpc-port=8545 --ws-external=true --ws-port=8546 --base-path=/root/parity/chain_data --chain=hardchain
# -l substrate=trace,sync=trace,rpc=trace,consensus=trace > /root/parity/log.txt 2>&1 &

pkill substrate
mv /data/parity/log.txt /data/parity/log.txt.bak

../target/release/substrate --base-path /data/parity/chain_data --chain ./hardchain.json \
--rpc-external=true --rpc-port=8545 --ws-external=true --ws-port=8546 --name=Junius \
-l substrate=trace,sync=trace,rpc=trace,consensus=trace,db=trace,sub-libp2p=trace \
--key=5Gg5SQ6uZFBvakCdL1BjLLtZe7KTy5qJGACwgX6oFavLhK1N, --validator=Junius \
> /data/parity/log.txt 2>&1 &
