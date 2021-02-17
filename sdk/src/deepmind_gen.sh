#!/bin/bash

# First:
#
#    cargo install protobuf-codegen
#
# and ensure you have `protoc` installed.

protoc --rust_out ./pb -I ~/codebase/dfuse-io/proto-solana dfuse/solana/codec/v1/codec.proto
