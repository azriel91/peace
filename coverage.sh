#! /bin/bash
set -euo pipefail

cargo coverage_clean
rm -rf ./target/coverage ./target/llvm-cov-target
mkdir -p ./target/coverage

# See `.config/cargo.toml`
for i in {0..0}
do cargo coverage_$i
done

cargo coverage_merge
