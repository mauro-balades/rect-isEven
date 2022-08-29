#!/usr/bin/sh

mkdir -p out
rustc --emit=llvm-ir src/lib.rs -o out/compiled.ll
