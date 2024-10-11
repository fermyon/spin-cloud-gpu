#!/bin/bash

# Build the plugin executable
cargo build --release

# Build fermyon-cloud-gpu
cd fermyon-cloud-gpu
npm i
spin build
cd ..

# Create the plugin tarball
cp target/release/spin-cloud-gpu cloud-gpu 
tar -czvf cloud-gpu.tar.gz cloud-gpu fermyon-cloud-gpu/spin.toml fermyon-cloud-gpu/target/spin-http-js.wasm

case "$OSTYPE" in
    darwin*) OSLABEL="macos" ;;
    linux*) OSLABEL="linux" ;;
    msys*) OSLABEL="windows" ;;
esac

case $(uname -m) in
    x86_64) ARCH="amd64" ;;
    *)    ARCH="aarch64" ;;
esac

# Create the plugin manifest
cat <<EOT > cloud-gpu.json
{
    "name": "cloud-gpu",
    "description": "A plugin to enable local AI development using cloud gpus.",
    "homepage": "https://developer.fermyon.com",
    "version": "0.1.0",
    "spinCompatibility": ">=1.4",
    "license": "Apache-2.0",
    "packages": [
        {
            "os": "$OSLABEL",
            "arch": "$ARCH",
            "url": "file:$(pwd)/cloud-gpu.tar.gz",
            "sha256": "$(sha256sum cloud-gpu.tar.gz | awk '{print $1;}')"
        }
    ]
}
EOT

# Cleanup
rm cloud-gpu
