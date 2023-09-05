# bytestr

[![crates.io](https://img.shields.io/crates/v/bytestr.svg)](https://crates.io/crates/bytestr) [![doc.rs](https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square)](https://docs.rs/bytestr)


A utility benefits zero-copy network programming,providing a cheaply cloneable and sliceable immutable UTF-8 encoded string.

# Serde support
Serde support is optional, and it is disabled by default.Enable `serde` feature to use this function:

```
[dependencies]
bytes = { version = "0.1.0", features = ["serde"] }
```

# Licnese
This project is licensed under [MIT license](https://opensource.org/licenses/MIT)