OUTFILE=inds

default: test

compile c:
	cargo build

compile_release cr:
	cargo build --release

test t:
	cargo test

run r:
	RUST_LOG=debug ./target/debug/inds tmp/tmp.txt

clean cl:
	cargo clean
