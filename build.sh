#!/usr/bin/env bash
cargo build --release
tar -C ./target/release -czf tidy_$1-x86_macos_darwin.tar.gz tidy
