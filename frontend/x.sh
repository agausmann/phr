#!/bin/sh

cd "$(dirname "$0")"

wasm-pack build --release --no-typescript --target web --out-name phr --out-dir static \
    || exit 1

# https://github.com/rustwasm/wasm-pack/issues/811
rm -f static/package.json static/.gitignore
