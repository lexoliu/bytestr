use alloc::borrow::{Borrow, Cow};
use alloc::string::String;
use bytes::Bytes;
use core::fmt;
use core::ops::{Deref, Index, Range, RangeFrom, RangeFull, RangeTo, RangeToInclusive};
use core::str::FromStr;

use crate::ByteStr;

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

// Index trait implementations for convenient slicing syntax

impl Index<Range<usize>> for ByteStr {
    type Output = str;

    fn index(&self, index: Range<usize>) -> &Self::Output {
        &self.as_str()[index]
    }
}

impl Index<RangeFrom<usize>> for ByteStr {
    type Output = str;

    fn index(&self, index: RangeFrom<usize>) -> &Self::Output {
        &self.as_str()[index]
    }
}

impl Index<RangeTo<usize>> for ByteStr {
    type Output = str;

    fn index(&self, index: RangeTo<usize>) -> &Self::Output {
        &self.as_str()[index]
    }
}

impl Index<RangeToInclusive<usize>> for ByteStr {
    type Output = str;

    fn index(&self, index: RangeToInclusive<usize>) -> &Self::Output {
        &self.as_str()[index]
    }
}

impl Index<RangeFull> for ByteStr {
    type Output = str;

    fn index(&self, _index: RangeFull) -> &Self::Output {
        self.as_str()
    }
}
