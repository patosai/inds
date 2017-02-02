OUTFILE=inds

default: test

compile c:
	cargo build

compile_release cr:
	cargo build --release

test t:
	cargo test

run r: compile
	RUST_LOG=debug ./target/debug/inds index tmp/tmp.txt

clean cl:
	cargo clean
