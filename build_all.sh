#!/bin/bash

set -e
set -o pipefail

# Targets to build
targets=(
  "x86_64-unknown-linux-gnu"
  "x86_64-pc-windows-gnu"
  # "x86_64-apple-darwin"
  # "aarch64-unknown-linux-gnu"
  # "aarch64-pc-windows-gnu"
  # "aarch64-apple-darwin"
)

# Ensure cross is installed
if ! command -v cross &> /dev/null
then
    echo "cross could not be found, installing..."
    cargo install cross
fi

# Clean the previous builds
rm -rf target/release

# Build project for each target
for target in "${targets[@]}"
do
  rustup target add $target
  echo "Building for target $target..."
  cross build --release --target $target
done

echo "All builds completed successfully."
