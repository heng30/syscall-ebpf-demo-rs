#!/usr/bin/env bash

all: kernel_space user_space

kernel_space:
	cd kernel-space && cargo build --release -Z build-std=core --bin kernel-space

user_space:
	RUST_LOG=info sudo -E cargo run --bin user-space -- -i enp2s0

clean:
	sudo -E cargo clean
