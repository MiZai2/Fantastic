#!/bin/bash

echo "--- Rust version info ---"
rustup --version
rustc --version
cargo --version

echo "--- Building plugin backend ---"
cargo build --release
mkdir -p ../bin
cp src/sh_tools.sh ../bin/
cp target/release/fantastic-rs ../bin/backend

echo " --- Cleaning up ---"
# remove root-owned target folder
cargo clean

#sudo rm /home/deck/homebrew/plugins/Fantastic/bin/backend
#sudo cp ../bin/backend /home/deck/homebrew/plugins/Fantastic/bin/backend

#sudo rm /home/deck/homebrew/plugins/Fantastic/bin/sh_tools.sh
#sudo cp ../bin/sh_tools.sh /home/deck/homebrew/plugins/Fantastic/bin/sh_tools.sh
