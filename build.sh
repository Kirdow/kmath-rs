#/bin/bash

set -e

cargo build
cp ./target/debug/kmath ./kmath

