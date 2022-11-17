default: compile test

compile: src/lib.rs Cargo.toml
	cargo build

test: src/lib.rs Cargo.toml test/Cargo.toml test/src/lib.rs
	cd test && cargo +nightly test -- --format json -Z unstable-options --report-time > ../test-report.json
	markdown-test-report test-report.json
	rm -rf test-report.json