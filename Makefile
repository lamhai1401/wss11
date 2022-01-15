SHELL := /bin/bash

# 0
run_server:
	CLIENT_HOST=http://localhost:8088 RUST_BACKTRACE=full \
	cargo run --bin wss11
.PHONY: run_server