run:
	cargo run

release:
	cargo build --release
	strip target/release/fighting-game
	upx --best --lzma target/release/fighting-game
	mkdir packed
	cp -r assets packed/
	cp target/release/fighting-game packed/
	rm -rf packed
	ouch compress packed fighting-game.tar.gz

perf:
	cargo clean
	cargo build --timings --release
	RUSTFLAGS='-C force-frame-pointers=y' cargo flamegraph -c "record -g"
