#!/usr/bin/env bash

# create the binary
cargo build --release

# install the binary
rm "$HOME/.local/bin/c"
ln -s "$(pwd)/target/release/c" "$HOME/.local/bin"

