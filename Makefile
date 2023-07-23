NAME=vodo
.PHONY: build run clean query

build:
	cargo build --release

run: build
	./target/release/$(NAME)

clean:
	rm -rf ./target

query:
	dig @127.0.0.1 -p 5353 cavall.in
