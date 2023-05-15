run:
	cargo run

release:
	cargo build --release
	upx --best --lzma target/release/fighting-game
	mkdir packed
	cp -r assets packed/
	cp target/release/fighting-game packed/
	ouch compress packed fighting-game.tar.gz
	rm -rf packed

dev:
	cargo build
	strip target/debug/fighting-game
	upx -1 --color target/debug/fighting-game
	mkdir packed
	cp -r assets packed/
	cp target/debug/fighting-game packed/
	ouch compress packed fighting-game.tar.gz
	rm -rf packed
