#!/usr/bin/env -S just --justfile

# the name of the crate
crate_name := "digital_link_rs"

# wasm build destination
wasm_dist := "pkg"

# rust backtrace
export RUST_BACKTRACE := "0"

# aliases
alias b := build
alias bw := build-wasm

# list all available recipes
[no-exit-message]
default:
	@just \
		--chooser 'fzf \
			--height 40% \
			--reverse' \
		--choose

build target profile="dev":
	cargo build \
		--target {{ target }} \
		--profile {{ profile }}

build-wasm: (build "wasm32-unknown-unknown" "release")
	wasm-bindgen ./target/wasm32-unknown-unknown/release/{{ crate_name }}.wasm \
		--out-dir {{ wasm_dist }} \
		--target nodejs
