OUTFILE=inds

default: test

all:
	cargo build

release:
	cargo build --release

test t:
	cargo test

run r: all
	RUST_LOG=debug ./target/debug/inds index tmp/tmp.txt

run-release rr: release
	RUST_LOG=debug ./target/release/inds index tmp/tmp.txt

clean c:
	cargo clean
