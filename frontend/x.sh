#!/bin/sh

wasm-pack build --release --no-typescript --target web --out-name phr --out-dir ./static

# https://github.com/rustwasm/wasm-pack/issues/811
rm -f static/package.json static/.gitignore
