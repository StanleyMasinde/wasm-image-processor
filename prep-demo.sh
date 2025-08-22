#!/bin/bash

mkdir -p demo

echo "Copying .js and .wasm files from pkg/ to ./demo/..."

cp pkg/*.js pkg/*.wasm demo/ 2>/dev/null || echo "Some files may not exist"

echo "Copy completed!"
echo "Files copied to ./demo/:"
ls -la demo/
