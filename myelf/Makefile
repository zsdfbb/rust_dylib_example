.ONESHELL:
SHELL := /usr/bin/bash

all:
	cargo build --release -vv

clean:
	cargo clean

run:
	@ echo "CUR_DIR: $(PWD)"
	export LD_LIBRARY_PATH=$(PWD)/myclib:$(PWD)/myrclib/target/release/deps :${LD_LIBRARY_PATH}
	cargo run --release

tree:
	cargo tree
