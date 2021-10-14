#!/bin/bash

# First:
#
#    cargo install protobuf-codegen
#
# and ensure you have `protoc` installed.

protoc --rust_out ./pb -I /Users/cbillett/devel/sf/proto-solana sf/solana/codec/v1/codec.proto
