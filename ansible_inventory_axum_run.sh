#!/usr/bin/env bash

# ./ansible_inventory_axum_run.sh
# 
# PORT=8081 ./ansible_inventory_axum_run.sh

set -ex

script_path=$(cd $(dirname $0) ; pwd -P)
script_path_root="${script_path}/"

# 
port="${PORT:-8080}"

RUST_BACKTRACE=1 RUST_LOG=trace cargo run --manifest-path=$script_path_root/Cargo.toml \
        -p ansible-inventory-cloud-demo-axum -- $port
