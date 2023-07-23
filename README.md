# <img src="./docs/vodo.png" style="width:30px;padding-right:20px;margin-bottom:-8px;">vòdo

A primitive DNS server written in Rust for fun.

## Usage

```bash

## Improvements

- Address lint warnings
- Add extensive comments and docs
- Replace hardcoded bits with constants
- Add logging

## Limitations

- There is no true concurrency in this server.
- It does not support TCP, IPv6, EDNS or DNSSEC.
- It cannot be used to host its own zones, and allow it to act as an authorative server.
- There is no caching.
- There are no automated tests or benchmarks.
