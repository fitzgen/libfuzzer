#!/bin/bash -ex

set -eux

cd $(dirname $0)/..
project_dir="$(pwd)"
tmp_dir="$(mktemp -d)"

git clone https://github.com/llvm-mirror/compiler-rt.git "$tmp_dir"
cd "$tmp_dir"
git checkout "$1"
rm -rf "$project_dir/libfuzzer-sys/upstream"
mv "$tmp_dir/lib/fuzzer/" "$project_dir/libfuzzer-sys/upstream"
