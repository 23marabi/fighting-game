run:
	cargo run

release:
	cargo build --release
	strip target/release/fighting-game
	upx --best --lzma target/release/fighting-game
	mkdir fighting-game
	cp -r assets fighting-game/
	cp config.ron fighting-game/
	cp target/release/fighting-game fighting-game/
	ouch compress fighting-game fighting-game.tar.bz
	rm -rf fighting-game

perf:
	cargo clean
	cargo build --timings --release
	RUSTFLAGS='-C force-frame-pointers=y' cargo flamegraph -c "record -g"
