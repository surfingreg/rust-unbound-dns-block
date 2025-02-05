# https://github.com/rustsec/rustsec/issues/1296#issuecomment-2523080051
cargo lock translate > Cargo.lock.v3
mv Cargo.lock.v3 Cargo.lock
cargo audit
