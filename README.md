# ByteStr

[![crates.io](https://img.shields.io/crates/v/bytestr.svg)](https://crates.io/crates/bytestr)
[![doc.rs](https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square)](https://docs.rs/bytestr)
[![Rust Version](https://img.shields.io/badge/rust-1.85.0%2B-orange.svg)](https://www.rust-lang.org)
[![CI](https://github.com/lexoliu/bytestr/workflows/CI/badge.svg)](https://github.com/lexoliu/bytestr/actions)

A **zero-copy**, **cheaply cloneable**, and **sliceable** immutable UTF-8 encoded string type built on top of [`bytes::Bytes`](https://docs.rs/bytes). Perfect for high-performance network programming and efficient string manipulation.

## ✨ Features

- **🚀 Zero-copy operations**: Clone and slice without additional allocations
- **⚡ High performance**: Built on the battle-tested `bytes` crate
- **🔄 Serde support**: Optional serialization/deserialization (feature-gated)
- **📦 `no_std` compatible**: Works in embedded and resource-constrained environments

## 🚀 Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
bytestr = "0.3"
```

### Basic Usage

```rust
use bytestr::ByteStr;

// Create from static string (zero-cost)
let s1 = ByteStr::from_static("Hello, world!");

// Create from String (reuses allocation)
let s2 = ByteStr::from("Hello, world!".to_string());

// Create from bytes with validation
let s3 = ByteStr::from_utf8(b"Hello, world!".to_vec())?;

// Clone is cheap (just a reference count increment)
let cloned = s1.clone();

// Slice without copying
let original_str = s1.as_str();
let slice = s1.slice_ref(&original_str[7..12]); // "world"

// Or use convenient indexing syntax (returns &str)
let indexed_slice = &s1[7..12]; // "world"

// All standard string operations work
assert_eq!(s1.len(), 13);
assert!(s1.starts_with("Hello"));
assert!(s1.contains("world"));
assert_eq!(slice.as_str(), indexed_slice);
```

### Advanced Usage

```rust
use bytestr::ByteStr;
use std::collections::HashMap;

// Perfect for network protocols and caching
let mut cache: HashMap<ByteStr, Vec<u8>> = HashMap::new();

// Zero-copy slicing for parsing
fn parse_header(data: &ByteStr) -> (ByteStr, ByteStr) {
    let data_str = data.as_str();
    let colon_pos = data_str.find(':').unwrap();
    let key_slice = &data_str[..colon_pos];
    let value_slice = &data_str[colon_pos + 2..]; // Skip ": "
    
    (
        data.slice_ref(key_slice),
        data.slice_ref(value_slice),
    )
}
```

## 🔧 Optional Features

### Serde Support

Enable serialization/deserialization support:

```toml
[dependencies]
bytestr = { version = "0.2", features = ["serde"] }
```

```rust
use bytestr::ByteStr;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Message {
    content: ByteStr,
}

let msg = Message {
    content: ByteStr::from("Hello, serde!"),
};

let json = serde_json::to_string(&msg)?;
let deserialized: Message = serde_json::from_str(&json)?;
```

## 📄 License

This project is licensed under the [MIT License](./LICENSE).