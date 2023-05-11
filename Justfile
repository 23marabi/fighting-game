run:
	cargo run

release:
	cargo build --release
	upx --best --lzma target/release/fighting-game
	mkdir packed
	cp -r assets packed/
	cp target/release/fighting-game packed/
	rm -rf packed
	ouch compress packed fighting-game.tar.gz
