.ONESHELL:
SHELL := /usr/bin/bash

all:
	cargo build --release -vv

clean:
	cargo clean

run:
	export LD_LIBRARY_PATH=$(PWD)/myclib:${LD_LIBRARY_PATH}
	cargo run --release

tree:
	cargo tree
