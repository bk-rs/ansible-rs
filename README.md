## Dev

```
cargo clippy --all-features --tests --examples -- -D clippy::all
cargo +nightly clippy --all-features --tests --examples -- -D clippy::all

cargo fmt -- --check

cargo test-all-features -- --nocapture
```

```
./ansible_inventory_axum_run.sh

ansible-inventory -i ./ansible_inventory_cloud.sh --list
```
