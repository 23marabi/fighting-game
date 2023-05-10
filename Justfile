run:
	cargo run

release:
	cargo clippy -- -W clippy::pedantic -W clippy::suspicious -W clippy::complexity -W clippy::perf -W clippy::cargo -W clipps::nursery -W clippy::unwrap_used -D warnings
	cargo +nightly udeps
	cargo build --release
	upx --best --lzma target/release/fighting-game
