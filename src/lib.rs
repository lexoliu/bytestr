#![no_std]
#![warn(
    missing_docs,
    missing_debug_implementations,
    clippy::all,
    clippy::style,
    clippy::correctness,
    clippy::complexity,
    clippy::suspicious,
    clippy::perf,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo
)]

//! # `ByteStr`
//!
//! A zero-copy, cheaply cloneable, and sliceable immutable UTF-8 encoded string type.
//!
//! `ByteStr` is built on top of [`bytes::Bytes`] and provides a UTF-8 guaranteed string
//! that can be cloned and sliced without additional allocations. This makes it perfect
//! for high-performance network programming, parsing, and any scenario where you need
//! efficient string manipulation.
//!
//! ## Examples
//!
//! ### Basic Usage
//!
//! ```rust
//! use bytestr::ByteStr;
//!
//! // Create from static string (zero-cost)
//! let static_str = ByteStr::from_static("Hello, world!");
//!
//! // Create from String (reuses allocation)
//! let from_string = ByteStr::from("Hello, world!".to_string());
//!
//! // Create from bytes with validation
//! let from_bytes = ByteStr::from_utf8(b"Hello, world!".to_vec()).unwrap();
//!
//! // All are equal
//! assert_eq!(static_str, from_string);
//! assert_eq!(from_string, from_bytes);
//! ```
//!
//! ### Zero-Copy Operations
//!
//! ```rust
//! use bytestr::ByteStr;
//!
//! let original = ByteStr::from_static("Hello, world!");
//!
//! // Cloning is O(1) - just increments reference count
//! let cloned = original.clone();
//!
//! // Slicing is O(1) - creates a new view without copying
//! let original_str = original.as_str();
//! let slice = original.slice_ref(&original_str[7..12]); // "world"
//!
//! assert_eq!(slice.as_str(), "world");
//! ```
//!
//! ### String Operations
//!
//! ```rust
//! use bytestr::ByteStr;
//!
//! let s = ByteStr::from("Hello, 世界! 🦀");
//!
//! // All standard string operations work
//! assert_eq!(s.len(), 19); // Byte length (not character count)
//! assert!(s.starts_with("Hello"));
//! assert!(s.contains("世界"));
//! assert!(s.contains("🦀"));
//! assert!(s.ends_with("🦀"));
//! ```
//!
//! ### Error Handling
//!
//! ```rust
//! use bytestr::ByteStr;
//!
//! // Invalid UTF-8 is rejected
//! let invalid_bytes = vec![0xFF, 0xFE, 0xFD];
//! let result = ByteStr::from_utf8(invalid_bytes);
//! assert!(result.is_err());
//! ```
//!
//! ## Optional Features
//!
//! ### Serde Support
//!
//! Enable the `serde` feature for serialization support:
//!
//! ```toml
//! [dependencies]
//! bytestr = { version = "0.2", features = ["serde"] }
//! ```
//!
//! ## Performance Notes
//!
//! - **Cloning**: O(1) - just increments a reference count
//! - **Slicing**: O(1) - creates a new view without copying data
//! - **Memory overhead**: Minimal compared to `String`
//! - **Cache friendly**: Consecutive string data in memory
//!
//! ## Safety
//!
//! This crate uses `unsafe` code internally for performance, but all unsafe operations are:
//! - Carefully reviewed and documented
//! - Thoroughly tested with comprehensive test suite
//! - Encapsulated behind safe APIs
//! - Verified for memory safety and UTF-8 validity
//!
//! [`bytes::Bytes`]: https://docs.rs/bytes/latest/bytes/struct.Bytes.html
extern crate alloc;

#[cfg(feature = "serde")]
mod serde;

use alloc::borrow::{Borrow, Cow};
use alloc::string::{FromUtf16Error, String};
use bytes::Bytes;
use core::fmt;
use core::ops::Deref;
use core::str::{FromStr, Utf8Error};

/// A cheaply cloneable and sliceable immutable UTF-8 encoded string.
#[derive(Default, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ByteStr(Bytes);

impl ByteStr {
    /// Creates an empty new `ByteStr`.
    ///
    /// This operation is very cheap as it doesn't allocate any memory.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytestr::ByteStr;
    ///
    /// let s = ByteStr::new();
    /// assert!(s.is_empty());
    /// assert_eq!(s.len(), 0);
    /// ```
    #[must_use]
    pub const fn new() -> Self {
        Self(Bytes::new())
    }

    /// Converts a vector of bytes to a `ByteStr`.
    ///
    /// This method will reuse the existing allocation, so no cloning will happen.
    /// The bytes are validated to ensure they form valid UTF-8.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytestr::ByteStr;
    ///
    /// // Valid UTF-8
    /// let valid_bytes = b"Hello, world!".to_vec();
    /// let s = ByteStr::from_utf8(valid_bytes).unwrap();
    /// assert_eq!(s.as_str(), "Hello, world!");
    ///
    /// // Invalid UTF-8
    /// let invalid_bytes = vec![0xFF, 0xFE, 0xFD];
    /// assert!(ByteStr::from_utf8(invalid_bytes).is_err());
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if the provided bytes are not valid UTF-8.
    pub fn from_utf8(bytes: impl Into<Bytes>) -> Result<Self, Utf8Error> {
        let bytes = bytes.into();

        match core::str::from_utf8(bytes.as_ref()) {
            Ok(_) => Ok(unsafe { Self::from_utf8_unchecked(bytes) }),
            Err(e) => Err(e),
        }
    }

    /// Converts a vector of bytes to a `ByteStr`, replacing invalid UTF-8 sequences with the replacement character (U+FFFD).
    ///
    /// This method will reuse the existing allocation if the bytes are valid UTF-8, or allocate a new string if invalid sequences are found.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytestr::ByteStr;
    ///
    /// let valid_bytes = b"Hello, world!".to_vec();
    /// let s = ByteStr::from_utf8_lossy(valid_bytes);
    /// assert_eq!(s.as_str(), "Hello, world!");
    ///
    /// let invalid_bytes = vec![0xFF, 0xFE, 0xFD];
    /// let s = ByteStr::from_utf8_lossy(invalid_bytes);
    /// assert_eq!(s.as_str(), "\u{FFFD}\u{FFFD}\u{FFFD}");
    /// ```
    pub fn from_utf8_lossy(bytes: impl Into<Bytes>) -> Self {
        let bytes = bytes.into();

        match String::from_utf8_lossy(bytes.as_ref()) {
            Cow::Borrowed(_) => unsafe { Self::from_utf8_unchecked(bytes) },
            Cow::Owned(string) => Self::from(string),
        }
    }

    /// Converts a slice of UTF-16 encoded data to a `ByteStr`.
    ///
    /// This method will allocate a new string and convert the UTF-16 data to UTF-8.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytestr::ByteStr;
    ///
    /// let utf16: Vec<u16> = "Hello, world!".encode_utf16().collect();
    /// let s = ByteStr::from_utf16(&utf16).unwrap();
    /// assert_eq!(s.as_str(), "Hello, world!");
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if the provided UTF-16 data is not valid.
    pub fn from_utf16(bytes: impl AsRef<[u16]>) -> Result<Self, FromUtf16Error> {
        String::from_utf16(bytes.as_ref()).map(Self::from)
    }

    /// Converts a slice of UTF-16 encoded data to a `ByteStr`, replacing invalid sequences with the replacement character (U+FFFD).
    ///
    /// This method will allocate a new string and convert the UTF-16 data to UTF-8, replacing any invalid sequences.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytestr::ByteStr;
    ///
    /// let utf16: Vec<u16> = vec![0xD800, 0xDC00, 0x0041]; // valid surrogate pair + 'A'
    /// let s = ByteStr::from_utf16_lossy(&utf16);
    /// assert!(s.as_str().contains('\u{FFFD}') || s.as_str().contains('A'));
    /// ```
    pub fn from_utf16_lossy(bytes: impl AsRef<[u16]>) -> Self {
        String::from_utf16_lossy(bytes.as_ref()).into()
    }

    /// Creates a `ByteStr` from a static string slice.
    ///
    /// This is a zero-cost operation as it directly references the static data
    /// without any allocation or copying.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytestr::ByteStr;
    ///
    /// let s = ByteStr::from_static("Hello, static world!");
    /// assert_eq!(s.as_str(), "Hello, static world!");
    /// ```
    #[must_use]
    pub const fn from_static(s: &'static str) -> Self {
        unsafe { Self::from_utf8_unchecked(Bytes::from_static(s.as_bytes())) }
    }

    /// Creates a `ByteStr` from bytes without UTF-8 validation.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytestr::ByteStr;
    /// use bytes::Bytes;
    ///
    /// let bytes = Bytes::from("Hello, world!");
    /// let s = unsafe { ByteStr::from_utf8_unchecked(bytes) };
    /// assert_eq!(s.as_str(), "Hello, world!");
    /// ```
    ///
    /// # Safety
    ///
    /// This function is unsafe because it does not check that the bytes passed
    /// to it are valid UTF-8. If this constraint is violated, it may cause
    /// memory unsafety issues with future users of the `ByteStr`.
    #[must_use]
    pub const unsafe fn from_utf8_unchecked(bytes: Bytes) -> Self {
        Self(bytes)
    }
    /// Unwraps the `ByteStr` into the inner `Bytes` object.
    ///
    /// This operation consumes the `ByteStr` and returns the underlying
    /// `Bytes` without any copying.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytestr::ByteStr;
    ///
    /// let s = ByteStr::from("Hello, world!");
    /// let bytes = s.into_bytes();
    /// assert_eq!(bytes.as_ref(), b"Hello, world!");
    /// ```
    pub fn into_bytes(self) -> Bytes {
        self.0
    }

    /// Extracts a string slice containing the entire string.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytestr::ByteStr;
    ///
    /// let s = ByteStr::from("Hello, world!");
    /// assert_eq!(s.as_str(), "Hello, world!");
    /// ```
    pub fn as_str(&self) -> &str {
        unsafe { core::str::from_utf8_unchecked(self.as_bytes()) }
    }

    /// Shortens the string, keeping the first `len` bytes and dropping the rest.
    ///
    /// # Panics
    ///
    /// Panics if the position of dropping the rest is not on a UTF-8 code point boundary, or if it is beyond the last code point.
    pub fn truncate(&mut self, len: usize) {
        if len < self.len() {
            assert!(self.deref().is_char_boundary(len));
            unsafe {
                self.as_bytes_mut().truncate(len);
            }
        }
    }

    /// Returns a slice of self that is equivalent to the given subset.
    ///
    /// This operation creates a new `ByteStr` that references a subset of the
    /// original data without copying. The subset must be a slice of the original
    /// string that lies on UTF-8 character boundaries.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytestr::ByteStr;
    ///
    /// let s = ByteStr::from("Hello, world!");
    /// let original_str = s.as_str();
    /// let world_slice = &original_str[7..12]; // "world"
    /// let sliced = s.slice_ref(world_slice);
    /// assert_eq!(sliced.as_str(), "world");
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the given `subset` is not contained within the `ByteStr`.
    #[must_use]
    pub fn slice_ref(&self, subset: &str) -> Self {
        unsafe { Self::from_utf8_unchecked(self.0.slice_ref(subset.as_bytes())) }
    }

    /// Removes all contents of the `ByteStr` while retaining the capacity.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytestr::ByteStr;
    ///
    /// let mut s = ByteStr::from("Hello, world!");
    /// assert!(!s.is_empty());
    /// s.clear();
    /// assert!(s.is_empty());
    /// ```
    pub fn clear(&mut self) {
        self.0.clear();
    }

    /// Provides a reference to the inner `Bytes` object.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytestr::ByteStr;
    ///
    /// let s = ByteStr::from("Hello, world!");
    /// let bytes = s.as_bytes();
    /// assert_eq!(bytes.len(), 13);
    /// ```
    pub const fn as_bytes(&self) -> &Bytes {
        &self.0
    }

    /// Provides a mutable reference to the inner `Bytes` object.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytestr::ByteStr;
    ///
    /// let mut s = ByteStr::from("Hello, world!");
    /// unsafe {
    ///     let bytes_mut = s.as_bytes_mut();
    ///     // Careful: ensure any modifications maintain UTF-8 validity
    /// }
    /// ```
    ///
    /// # Safety
    ///
    /// The caller must ensure that the content of the slice is valid UTF-8
    /// before the borrow ends and the `ByteStr` is used.
    pub const unsafe fn as_bytes_mut(&mut self) -> &mut Bytes {
        &mut self.0
    }

    /// Returns `true` if the `ByteStr` has a length of zero bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytestr::ByteStr;
    ///
    /// let empty = ByteStr::new();
    /// assert!(empty.is_empty());
    ///
    /// let non_empty = ByteStr::from("hello");
    /// assert!(!non_empty.is_empty());
    /// ```
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns the length of this `ByteStr` in bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytestr::ByteStr;
    ///
    /// let s = ByteStr::from("Hello, 世界!");
    /// assert_eq!(s.len(), 13);
    /// ```
    #[must_use]
    pub const fn len(&self) -> usize {
        self.0.len()
    }
}

impl fmt::Debug for ByteStr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self.as_str(), f)
    }
}

impl fmt::Display for ByteStr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.as_str(), f)
    }
}

impl Deref for ByteStr {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl AsRef<str> for ByteStr {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Borrow<str> for ByteStr {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl AsRef<[u8]> for ByteStr {
    fn as_ref(&self) -> &[u8] {
        self.as_str().as_bytes()
    }
}

impl FromStr for ByteStr {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s))
    }
}

impl<T: Into<String>> From<T> for ByteStr {
    fn from(s: T) -> Self {
        Self(s.into().into_bytes().into())
    }
}

impl PartialEq<str> for ByteStr {
    fn eq(&self, other: &str) -> bool {
        &**self == other
    }
}

impl PartialEq<String> for ByteStr {
    fn eq(&self, other: &String) -> bool {
        self.eq(&**other)
    }
}

impl PartialEq<&str> for ByteStr {
    fn eq(&self, other: &&str) -> bool {
        self.eq(*other)
    }
}

impl PartialEq<Cow<'_, str>> for ByteStr {
    fn eq(&self, other: &Cow<str>) -> bool {
        self.eq(&**other)
    }
}

impl PartialEq<ByteStr> for String {
    fn eq(&self, other: &ByteStr) -> bool {
        other.eq(self)
    }
}

impl PartialEq<ByteStr> for str {
    fn eq(&self, other: &ByteStr) -> bool {
        other.eq(self)
    }
}

impl PartialEq<ByteStr> for &str {
    fn eq(&self, other: &ByteStr) -> bool {
        other.eq(self)
    }
}

impl PartialEq<ByteStr> for Cow<'_, str> {
    fn eq(&self, other: &ByteStr) -> bool {
        other.eq(self)
    }
}

impl From<ByteStr> for Bytes {
    fn from(data: ByteStr) -> Self {
        data.into_bytes()
    }
}

#[cfg(test)]
mod tests;
