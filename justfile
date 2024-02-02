build:
	cargo build --release

install:
	cp -f target/release/rwm /usr/local/bin/

uninstall:
	rm -f /usr/local/bin/rwm

clean:
	rm -rf target/ Cargo.lock
