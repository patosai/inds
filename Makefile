OUTFILE=inds

default: test

compile c:
	cargo build

test t:
	cargo test

run r:
	RUST_LOG=debug ./target/debug/inds tmp/tmp.txt

clean cl:
	cargo clean
