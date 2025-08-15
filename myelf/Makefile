all:
	echo $(PATH)
	echo $(LD_LIBRARY_PATH)
	# export LD_LIBRARY_PATH=/home/zs/Develop/RustExample/dylib_example/mylib/target/release:$(LD_LIBRARY_PATH)
	cargo build --release -vv

clean:
	cargo clean

run:
	cargo run --release

tree:
	cargo tree
