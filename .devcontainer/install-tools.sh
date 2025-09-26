set -x

sudo apt-get update -y
sudo apt install libclang-dev -y

cargo install wasm-pack
cargo install wasm-bindgen-cli

export PATH="$HOME/.cargo/bin:$PATH"

npm install turbo --global