#/bin/bash


cp src/*.rs ../edgetracer-wasm/src
pushd ../edgetracer-wasm
git checkout src/main.rs
popd