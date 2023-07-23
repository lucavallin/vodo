name=vodo
hostname ?= "cavall.in"
port ?= 5353

build:
	cargo build --release
build-debug:
	cargo build
build-release: build

run: build
	./target/release/$(name) -p $(port)

clean:
	rm -rf ./target

query:
	dig @127.0.0.1 -p 5353 $(hostname)

.PHONY: all
