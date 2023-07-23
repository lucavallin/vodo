name=vodo
hostname ?= "cavall.in"

build:
	cargo build --release
build-debug:
	cargo build
build-release: build

run: build
	./target/release/$(name)

clean:
	rm -rf ./target

query:
	dig @127.0.0.1 -p 5353 $(hostname)

.PHONY: all
