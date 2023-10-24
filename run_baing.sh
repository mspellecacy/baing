#!/usr/bin/env bash
cargo run &
cd frontend || exit
trunk serve