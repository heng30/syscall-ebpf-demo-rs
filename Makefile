#!/usr/bin/env bash

all: ebpf debug

ebpf:
	cd syscall_ebpf-ebpf && cargo build --release -Z build-std=core --bin syscall_ebpf-ebpf

debug:
	RUST_LOG=info sudo -E cargo run --bin syscall_ebpf -- -i enp2s0

clean:
	sudo -E cargo clean
