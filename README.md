# <img src="./docs/vodo.png" style="width:30px;padding-right:20px;margin-bottom:-8px;">vòdo

A primitive DNS server written in Rust for fun.

```bash
@lucavallin ➜ /workspaces/vodo (main) $ ./target/debug/vodo -h
A primitive DNS server written in Rust for fun.

Usage: vodo [OPTIONS]

Options:
  -p, --port <PORT>  Port for the server to listen on [default: 5353]
  -h, --help         Print help
  -V, --version      Print version
```

## Usage

```bash
# Build the server
$ cargo build --release

# Run the server (or use cargo run)
$ ./target/release/vodo -p 5353

# Query the server
$ dig @127.0.0.1 -p 5353 cavall.in

; <<>> DiG 9.18.16-1~deb12u1-Debian <<>> @127.0.0.1 -p 5353 cavall.in
; (1 server found)
;; global options: +cmd
;; Got answer:
;; ->>HEADER<<- opcode: QUERY, status: NOERROR, id: 8919
;; flags: qr rd ra; QUERY: 1, ANSWER: 4, AUTHORITY: 2, ADDITIONAL: 0

;; QUESTION SECTION:
;cavall.in.                     IN      A

;; ANSWER SECTION:
cavall.in.              1799    IN      A       185.199.111.153
cavall.in.              1799    IN      A       185.199.108.153
cavall.in.              1799    IN      A       185.199.109.153
cavall.in.              1799    IN      A       185.199.110.153

;; AUTHORITY SECTION:
cavall.in.              1800    IN      NS      dns1.registrar-servers.com.
cavall.in.              1800    IN      NS      dns2.registrar-servers.com.

;; Query time: 120 msec
;; SERVER: 127.0.0.1#2053(127.0.0.1) (UDP)
;; WHEN: Sun Jul 23 16:04:24 UTC 2023
;; MSG SIZE  rcvd: 225

```

## Makefile

I have included a Makefile to make it easier to build and run the server.

```bash
# Build the server
$ make build[-release|-debug]
# Run the server
$ make run [port=5353]
# Clean the build
$ make clean
# Query the server
$ make query [hostname=example.com]
```

## Limitations

- There is no true concurrency in this server.
- It does not support TCP, IPv6, EDNS or DNSSEC.
- It cannot be used to host its own zones, and allow it to act as an authorative server.
- There is no caching.
- There are no automated tests or benchmarks.

## Improvements

- Use [tokio-rs/bytes](https://github.com/tokio-rs/bytes) for handling buffers.
- Replace `BufferError::GenericError(String)` with `#[error("I/O error: {0}")] IoError(#[from] std::io::Error)`
- Async/await with tokio.rs (`header.rs` and `packet.rs` could use [tokio_util::codec](https://docs.rs/tokio-util/latest/tokio_util/codec/index.html))
- Consider using crate `bitvec` for bit manipulation.
