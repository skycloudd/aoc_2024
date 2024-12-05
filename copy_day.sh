#!/bin/sh

set -e

if [ -z "$1" ]; then
    echo "Usage: $0 <day_number>"
    exit 1
fi

if [ -d "day$1" ]; then
    echo "day$1 already exists"
    exit 1
fi

cargo new day$1

cp template/src/main.rs day$1/src/main.rs

echo "$(tail -n +2 day$1/src/main.rs)" > day$1/src/main.rs

echo "$(head -n 5 day$1/Cargo.toml)" > day$1/Cargo.toml

echo "
[dependencies]
common = { path = \"../common\" }

[lints]
workspace = true" >> day$1/Cargo.toml
