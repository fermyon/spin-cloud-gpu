#!/bin/bash

cargo build --release
cd cloud-gpu-app
npm i
spin build
cd ..
cp target/release/spin-cloud-gpu cloud-gpu 
tar -czvf cloud-gpu.tar.gz cloud-gpu cloud-gpu-app/spin.toml cloud-gpu-app/target/spin-http-js.wasm

cat <<EOT > cloud-gpu.json
{
    "name": "cloud-gpu",
    "description": "A plugin to enable local AI development using cloud gpus.",
    "homepage": "www.example.com",
    "version": "0.1.0",
    "spinCompatibility": ">=1.4",
    "license": "Apache-2.0",
    "packages": [
        {
            "os": "macos",
            "arch": "aarch64",
            "url": "file:$(pwd)/cloud-gpu.tar.gz",
            "sha256": "$(sha256sum cloud-gpu.tar.gz | awk '{print $1;}')"
        }
    ]
}
EOT
