# <img src="./docs/vodo.png" style="width:30px;padding-right:20px;margin-bottom:-8px;">vòdo

A primitive DNS server written in Rust for fun.

```bash
@lucavallin ➜ /workspaces/vodo (main) $ ./target/debug/vodo -h
A primitive DNS server written in Rust for fun.

Usage: vodo [OPTIONS]

Options:
  -p, --port <PORT>  Port for the server to listen on [default: 2053]
  -h, --help         Print help
  -V, --version      Print version
```

## Usage

```bash
# Build the server
$ cargo build --release

# Run the server (or use cargo run)
$ ./target/release/vodo -p 2053

# Query the server
$ dig @127.0.0.1 -p 2053 cavall.in
```

## Improvements

- Add extensive comments, replace hardcoded bits with constants
- Pass the port as an argument to main.rs

## Limitations

- There is no true concurrency in this server.
- It does not support TCP, IPv6, EDNS or DNSSEC.
- It cannot be used to host its own zones, and allow it to act as an authorative server.
- There is no caching.
- There are no automated tests or benchmarks.
