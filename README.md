# <img src="./docs/vodo.png" style="width:30px;padding-right:20px;margin-bottom:-8px;">vòdo

A primitive DNS server written in Rust for fun.

## TODO

- Add extensive comments and docs
- Add tests
- Add benchmarks
- Add more DNS record types
- Address lint warnings
- Replace hardcoded bits with constants
- CLI?
- Add more logging

### Limitations

- There is no true concurrency in this server.
- It does not support TCP, IPv6, EDNS or DNSSEC.
- It cannot be used to host its own zones, and allow it to act as an authorative server.
- There is no caching.
