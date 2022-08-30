#!/usr/bin/sh

mkdir -p out
rustc --emit=llvm-ir --crate-type rlib src/lib.rs -o out/compiled.ll
