#!/bin/sh

echo compiling... 
cargo build --release 

eval dir="~/.local/bin"

echo "moving binary to $dir"
mkdir -p $dir  # create dir if it doesn't exist
cp -f ./target/release/covid $dir