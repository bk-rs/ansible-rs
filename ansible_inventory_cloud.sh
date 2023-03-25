#!/usr/bin/env bash

# ./ansible_inventory_cloud.sh --list
# PORT=8081 ./ansible_inventory_cloud.sh --list
# ./ansible_inventory_cloud.sh --host host_foo
# PORT=8081 ./ansible_inventory_cloud.sh --host host_foo
# 
# ansible-inventory -i ansible_inventory_cloud.sh --list
# PORT=8081 ansible-inventory -i ansible_inventory_cloud.sh --list
# 

script_path=$(cd $(dirname $0) ; pwd -P)
script_path_root="${script_path}/"

# 
port="${PORT:-8080}"

RUST_BACKTRACE=1 RUST_LOG=trace cargo run --manifest-path=${script_path_root}Cargo.toml \
        -q \
        -p ansible-inventory-cloud-cli --features with_http \
        --bin ansible-inventory-http-cli -- \
        --list-url "http://127.0.0.1:$port/ansible_inventory/list" \
        --host-url "http://127.0.0.1:$port/ansible_inventory/host" \
        --access-token "TOKEN" \
        "$@"

<<'COMMENT'
cargo install ansible-inventory-cloud-cli

ansible-inventory-http-cli \
    --list-url 'http://127.0.0.1:8080/ansible_inventory/list' \
    --host-url 'http://127.0.0.1:8080/ansible_inventory/host' \
    --access-token "TOKEN" \
    "$@"
COMMENT
