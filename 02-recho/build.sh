#!/bin/bash

# set -xe 
cargo build -q
./target/debug/recho $@ 
