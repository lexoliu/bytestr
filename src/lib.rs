#![warn(missing_docs, missing_debug_implementations)]

//! This crate provide a utility `ByteStr`,a cheaply cloneable and sliceable immutable UTF-8 encoded string,which is using `Bytes` as storage.
//! `ByteStr` can be widely used in web programming,and reduce much unnecessary copy.

#[cfg(feature = "serde")]
mod serde;

use bytes::Bytes;
use std::borrow::{Borrow, Cow};
use std::fmt;
use std::ops::Deref;
use std::str::{FromStr, Utf8Error};

/// A cheaply cloneable and sliceable immutable UTF-8 encoded string.
#[derive(Default, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ByteStr(Bytes);

impl ByteStr {
    /// Create a empty new `ByteStr`.
    pub const fn new() -> Self {
        Self(Bytes::new())
    }

    /// Converts a vector of bytes to a `ByteStr`.This method will reuse the vector so that no clone will happen.
    pub fn from_utf8(bytes: impl Into<Bytes>) -> Result<Self, Utf8Error> {
        let bytes = bytes.into();

        std::str::from_utf8(bytes.as_ref())
            .map(|_| ())
            .map(|_| unsafe { Self::from_utf8_unchecked(bytes) })
    }
    /// Create a `ByteStr` from a static string.This method will reuse the vector so that no clone will happen.

    pub fn from_static(s: &'static str) -> Self {
        unsafe { Self::from_utf8_unchecked(Bytes::from_static(s.as_bytes())) }
    }

    /// # Safety
    ///
    /// This function is unsafe because it does not check that the bytes passed
    /// to it are valid UTF-8. If this constraint is violated, it may cause
    /// memory unsafety issues with future users of the `ByteStr`.

    pub const unsafe fn from_utf8_unchecked(bytes: Bytes) -> Self {
        Self(bytes)
    }
    /// Unwrap the `ByteStr` into the inner `Bytes` object.
    pub fn into_bytes(self) -> Bytes {
        self.0
    }

    /// Extracts a string slice containing the entire string.
    pub fn as_str(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(self.as_bytes().deref()) }
    }

    /// Shortens the string, keeping the first `len` bytes and dropping the rest.
    /// # Panics:
    /// Panic if the position of dropping the rest is not on a UTF-8 code point boundary, or if it is beyond the last code point.

    pub fn truncate(&mut self, len: usize) {
        if len < self.len() {
            assert!(self.deref().is_char_boundary(len));
            unsafe {
                self.as_bytes_mut().truncate(len);
            }
        }
    }

    /// Returns a slice of self that is equivalent to the given subset.No copy will happen in this method.
    /// # Panics:
    /// Panics if the given `subset` is not contained within the `ByteStr` in fact.
    pub fn slice_ref(&self, subset: &str) -> Self {
        unsafe { Self::from_utf8_unchecked(self.0.slice_ref(subset.as_bytes())) }
    }

    /// Removing all contents of the `ByteStr` but still remain the capacity.
    pub fn clear(&mut self) {
        self.0.clear()
    }

    /// Provide a reference of the inner `Bytes` object.
    pub const fn as_bytes(&self) -> &Bytes {
        &self.0
    }

    /// Provide a mutable reference of the inner `Bytes` object.
    /// # Safety
    ///
    /// The caller must ensure that the content of the slice is valid UTF-8
    /// before the borrow ends and the `ByteStr` is used.

    pub unsafe fn as_bytes_mut(&mut self) -> &mut Bytes {
        &mut self.0
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
        Self::from(Self(s.into().into_bytes().into()))
    }
}

impl PartialEq<str> for ByteStr {
    fn eq(&self, other: &str) -> bool {
        self.deref() == other
    }
}

impl PartialEq<String> for ByteStr {
    fn eq(&self, other: &String) -> bool {
        self.eq(other.deref())
    }
}

impl PartialEq<&str> for ByteStr {
    fn eq(&self, other: &&str) -> bool {
        self.eq(*other)
    }
}

impl<'a> PartialEq<Cow<'a, str>> for ByteStr {
    fn eq(&self, other: &Cow<str>) -> bool {
        self.eq(other.deref())
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

impl<'a> PartialEq<ByteStr> for Cow<'a, str> {
    fn eq(&self, other: &ByteStr) -> bool {
        other.eq(self)
    }
}

impl From<ByteStr> for Bytes {
    fn from(data: ByteStr) -> Self {
        data.into_bytes()
    }
}
