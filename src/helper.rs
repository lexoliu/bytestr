use crate::ByteStr;

impl ByteStr {
    /// Returns an iterator over the lines of the string, as zero-copy `ByteStr` slices.
    ///
    /// Lines are ended with either a newline (`\n`) or a carriage return with a line
    /// feed (`\r\n`). The final line ending is optional.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytestr::ByteStr;
    ///
    /// let text = ByteStr::from("foo\nbar\nbaz");
    /// let lines: Vec<_> = text.lines().collect();
    /// assert_eq!(lines.len(), 3);
    /// assert_eq!(lines[0].as_str(), "foo");
    /// assert_eq!(lines[1].as_str(), "bar");
    /// assert_eq!(lines[2].as_str(), "baz");
    /// ```
    pub fn lines(&self) -> impl Iterator<Item = Self> {
        self.as_str().lines().map(|s| self.slice_ref(s))
    }

    /// Divides one `ByteStr` into two at an index.
    ///
    /// The argument, `mid`, should be a byte offset from the start of the string.
    /// It must be on the boundary of a UTF-8 code point.
    ///
    /// The two slices returned go from the start of the string to `mid`, and from
    /// `mid` to the end of the string.
    ///
    /// # Panics
    ///
    /// Panics if `mid` is not on a UTF-8 code point boundary, or if it is beyond
    /// the last code point of the string slice.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytestr::ByteStr;
    ///
    /// let s = ByteStr::from("Hello, world!");
    /// let (left, right) = s.split_at(7);
    /// assert_eq!(left.as_str(), "Hello, ");
    /// assert_eq!(right.as_str(), "world!");
    /// ```
    pub fn split_at(self, mid: usize) -> (Self, Self) {
        let left = self.slice_ref(&self.as_str()[..mid]);
        let right = self.slice_ref(&self.as_str()[mid..]);
        (left, right)
    }

    /// Splits a `ByteStr` by a pattern, returning an iterator of zero-copy slices.
    ///
    /// The pattern can be a `&str`. The iterator returned will yield `ByteStr` instances
    /// that reference parts of the original string without copying data.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytestr::ByteStr;
    ///
    /// let s = ByteStr::from("hello,world,rust");
    /// let parts: Vec<_> = s.split(",").collect();
    /// assert_eq!(parts.len(), 3);
    /// assert_eq!(parts[0].as_str(), "hello");
    /// assert_eq!(parts[1].as_str(), "world");
    /// assert_eq!(parts[2].as_str(), "rust");
    /// ```
    pub fn split(&self, pat: &str) -> impl Iterator<Item = Self> {
        self.as_str().split(pat).map(move |s| self.slice_ref(s))
    }

    /// Splits a `ByteStr` by a pattern, limiting the number of splits.
    ///
    /// The `n` parameter specifies the maximum number of splits to make.
    /// The last element of the iterator will contain the remainder of the string.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytestr::ByteStr;
    ///
    /// let s = ByteStr::from("a,b,c,d");
    /// let parts: Vec<_> = s.splitn(3, ",").collect();
    /// assert_eq!(parts.len(), 3);
    /// assert_eq!(parts[0].as_str(), "a");
    /// assert_eq!(parts[1].as_str(), "b");
    /// assert_eq!(parts[2].as_str(), "c,d");
    /// ```
    pub fn splitn(&self, n: usize, pat: &str) -> impl Iterator<Item = Self> {
        self.as_str().splitn(n, pat).map(move |s| self.slice_ref(s))
    }

    /// Splits a `ByteStr` on the first occurrence of a pattern.
    ///
    /// Returns `Some((before, after))` if the pattern is found, where both parts
    /// are zero-copy `ByteStr` slices. Returns `None` if the pattern is not found.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytestr::ByteStr;
    ///
    /// let s = ByteStr::from("key=value");
    /// if let Some((key, value)) = s.split_once("=") {
    ///     assert_eq!(key.as_str(), "key");
    ///     assert_eq!(value.as_str(), "value");
    /// }
    ///
    /// let s = ByteStr::from("no-equals-sign");
    /// assert!(s.split_once("=").is_none());
    /// ```
    pub fn split_once(&self, pat: &str) -> Option<(Self, Self)> {
        self.as_str()
            .split_once(pat)
            .map(|(l, r)| (self.slice_ref(l), self.slice_ref(r)))
    }

    /// Splits a `ByteStr` by ASCII whitespace, returning an iterator of zero-copy slices.
    ///
    /// 'Whitespace' is defined according to the terms of the Unicode Derived Core Property
    /// `White_Space`. This includes spaces, tabs, newlines, and other whitespace characters.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytestr::ByteStr;
    ///
    /// let s = ByteStr::from("  hello   world  ");
    /// let words: Vec<_> = s.split_whitespace().collect();
    /// assert_eq!(words.len(), 2);
    /// assert_eq!(words[0].as_str(), "hello");
    /// assert_eq!(words[1].as_str(), "world");
    /// ```
    pub fn split_whitespace(&self) -> impl Iterator<Item = Self> {
        self.as_str().split_whitespace().map(|s| self.slice_ref(s))
    }

    /// Removes a prefix from the string, returning the remainder as a new `ByteStr`.
    ///
    /// If the string starts with the pattern `prefix`, returns `Some` with the remainder
    /// of the string after the prefix. Otherwise, returns `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytestr::ByteStr;
    ///
    /// let s = ByteStr::from("foo:bar");
    /// assert_eq!(s.strip_prefix("foo:"), Some(ByteStr::from("bar")));
    /// assert_eq!(s.strip_prefix("bar"), None);
    /// ```
    pub fn strip_prefix(&self, prefix: &str) -> Option<Self> {
        self.as_str()
            .strip_prefix(prefix)
            .map(|s| self.slice_ref(s))
    }

    /// Removes a suffix from the string, returning the remainder as a new `ByteStr`.
    ///
    /// If the string ends with the pattern `suffix`, returns `Some` with the remainder
    /// of the string before the suffix. Otherwise, returns `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytestr::ByteStr;
    ///
    /// let s = ByteStr::from("bar:foo");
    /// assert_eq!(s.strip_suffix(":foo"), Some(ByteStr::from("bar")));
    /// assert_eq!(s.strip_suffix("baz"), None);
    /// ```
    pub fn strip_suffix(&self, suffix: &str) -> Option<Self> {
        self.as_str()
            .strip_suffix(suffix)
            .map(|s| self.slice_ref(s))
    }

    /// Returns a `ByteStr` with leading whitespace removed.
    ///
    /// 'Whitespace' is defined according to the terms of the Unicode Derived Core Property
    /// `White_Space`.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytestr::ByteStr;
    ///
    /// let s = ByteStr::from("  hello world  ");
    /// assert_eq!(s.trim_start().as_str(), "hello world  ");
    /// ```
    #[must_use]
    pub fn trim_start(&self) -> Self {
        self.slice_ref(self.as_str().trim_start())
    }

    /// Returns a `ByteStr` with trailing whitespace removed.
    ///
    /// 'Whitespace' is defined according to the terms of the Unicode Derived Core Property
    /// `White_Space`.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytestr::ByteStr;
    ///
    /// let s = ByteStr::from("  hello world  ");
    /// assert_eq!(s.trim_end().as_str(), "  hello world");
    /// ```
    #[must_use]
    pub fn trim_end(&self) -> Self {
        self.slice_ref(self.as_str().trim_end())
    }

    /// Returns a `ByteStr` with leading and trailing whitespace removed.
    ///
    /// 'Whitespace' is defined according to the terms of the Unicode Derived Core Property
    /// `White_Space`.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytestr::ByteStr;
    ///
    /// let s = ByteStr::from("  hello world  ");
    /// assert_eq!(s.trim().as_str(), "hello world");
    /// ```
    #[must_use]
    pub fn trim(&self) -> Self {
        self.slice_ref(self.as_str().trim())
    }

    /// Takes the first `n` bytes from the string.
    ///
    /// This operation creates a new `ByteStr` that references the first `n` bytes
    /// of the original string without copying. The position must be on a UTF-8
    /// character boundary.
    ///
    /// # Panics
    ///
    /// Panics if `n` is not on a UTF-8 code point boundary, or if it is beyond
    /// the last code point of the string slice.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytestr::ByteStr;
    ///
    /// let s = ByteStr::from("Hello, world!");
    /// let prefix = s.take(5);
    /// assert_eq!(prefix.as_str(), "Hello");
    /// ```
    #[must_use]
    pub fn take(&self, n: usize) -> Self {
        self.slice_ref(&self.as_str()[..n])
    }

    /// Skips the first `n` bytes and returns the remainder.
    ///
    /// This operation creates a new `ByteStr` that references the string starting
    /// from the `n`th byte without copying. The position must be on a UTF-8
    /// character boundary.
    ///
    /// # Panics
    ///
    /// Panics if `n` is not on a UTF-8 code point boundary, or if it is beyond
    /// the last code point of the string slice.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytestr::ByteStr;
    ///
    /// let s = ByteStr::from("Hello, world!");
    /// let suffix = s.skip(7);
    /// assert_eq!(suffix.as_str(), "world!");
    /// ```
    #[must_use]
    pub fn skip(&self, n: usize) -> Self {
        self.slice_ref(&self.as_str()[n..])
    }

    /// Takes characters from the start until a pattern is found.
    ///
    /// Returns a `ByteStr` containing everything before the first occurrence of
    /// the pattern. If the pattern is not found, returns a clone of the entire string.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytestr::ByteStr;
    ///
    /// let s = ByteStr::from("Hello, world!");
    /// let prefix = s.take_until(",");
    /// assert_eq!(prefix.as_str(), "Hello");
    ///
    /// let no_comma = ByteStr::from("Hello world");
    /// let all = no_comma.take_until(",");
    /// assert_eq!(all.as_str(), "Hello world");
    /// ```
    #[must_use]
    pub fn take_until(&self, pat: &str) -> Self {
        self.as_str()
            .find(pat)
            .map_or_else(|| self.clone(), |pos| self.slice_ref(&self.as_str()[..pos]))
    }

    /// Skips characters from the start while they match a predicate.
    ///
    /// Returns a `ByteStr` starting from the first character that doesn't match
    /// the predicate, or an empty `ByteStr` if all characters match.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytestr::ByteStr;
    ///
    /// let s = ByteStr::from("   hello world");
    /// let trimmed = s.skip_while(|c| c.is_whitespace());
    /// assert_eq!(trimmed.as_str(), "hello world");
    ///
    /// let digits = ByteStr::from("123abc");
    /// let letters = digits.skip_while(|c| c.is_ascii_digit());
    /// assert_eq!(letters.as_str(), "abc");
    /// ```
    #[must_use]
    pub fn skip_while<F>(&self, mut f: F) -> Self
    where
        F: FnMut(char) -> bool,
    {
        let mut start = 0;
        for ch in self.as_str().chars() {
            if f(ch) {
                start += ch.len_utf8();
            } else {
                break;
            }
        }
        self.slice_ref(&self.as_str()[start..])
    }

    /// Takes characters from the start while they match a predicate.
    ///
    /// Returns a tuple where the first element contains the characters that matched
    /// the predicate, and the second element contains the remaining characters.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytestr::ByteStr;
    ///
    /// let s = ByteStr::from("123abc456");
    /// let (digits, rest) = s.take_while(|c| c.is_ascii_digit());
    /// assert_eq!(digits.as_str(), "123");
    /// assert_eq!(rest.as_str(), "abc456");
    ///
    /// let letters = ByteStr::from("hello123");
    /// let (word, numbers) = letters.take_while(|c| c.is_alphabetic());
    /// assert_eq!(word.as_str(), "hello");
    /// assert_eq!(numbers.as_str(), "123");
    /// ```
    pub fn take_while<F>(&self, mut predicate: F) -> (Self, Self)
    where
        F: FnMut(char) -> bool,
    {
        let mut end = 0;
        for ch in self.as_str().chars() {
            if predicate(ch) {
                end += ch.len_utf8();
            } else {
                break;
            }
        }
        let taken = self.slice_ref(&self.as_str()[..end]);
        let remaining = self.slice_ref(&self.as_str()[end..]);
        (taken, remaining)
    }
}
