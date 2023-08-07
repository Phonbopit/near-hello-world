#!/bin/sh

./build.sh

if [ $? -ne 0 ]; then
  echo ">> Error building contract"
  exit 1
fi

echo ">> Deploying contract"

near deploy --wasmFile ./target/wasm32-unknown-unknown/release/hello_world.wasm --accountId helloworld.0xchai.near