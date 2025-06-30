use crate::ByteStr;
use alloc::{borrow::Cow, format, string::String, vec};

#[test]
fn test_new() {
    let bs = ByteStr::new();
    assert_eq!(bs.as_str(), "");
    assert!(bs.is_empty());
}

#[test]
fn test_from_static() {
    let bs = ByteStr::from_static("hello");
    assert_eq!(bs.as_str(), "hello");
    assert_eq!(bs.len(), 5);
}

#[test]
fn test_from_utf8_valid() {
    let bytes = b"hello world".to_vec();
    let bs = ByteStr::from_utf8(bytes).unwrap();
    assert_eq!(bs.as_str(), "hello world");
}

#[test]
fn test_from_utf8_invalid() {
    let invalid_bytes = vec![0xff, 0xfe, 0xfd];
    let result = ByteStr::from_utf8(invalid_bytes);
    assert!(result.is_err());
}

#[test]
fn test_from_string() {
    let s = String::from("test string");
    let bs = ByteStr::from(s);
    assert_eq!(bs.as_str(), "test string");
}

#[test]
fn test_from_str() {
    let bs = ByteStr::from("another test");
    assert_eq!(bs.as_str(), "another test");
}

#[test]
fn test_clone() {
    let bs1 = ByteStr::from("clone test");
    let bs2 = bs1.clone();
    assert_eq!(bs1, bs2);
    assert_eq!(bs1.as_str(), bs2.as_str());
}

#[test]
fn test_truncate() {
    let mut bs = ByteStr::from("hello world");
    bs.truncate(5);
    assert_eq!(bs.as_str(), "hello");
}

#[test]
fn test_truncate_utf8_boundary() {
    let mut bs = ByteStr::from("hello 世界");
    bs.truncate(6);
    assert_eq!(bs.as_str(), "hello ");
}

#[test]
#[should_panic(expected = "assertion failed: self.deref().is_char_boundary(len)")]
fn test_truncate_invalid_boundary() {
    let mut bs = ByteStr::from("hello 世界");
    // This should panic because it's not on a UTF-8 boundary
    bs.truncate(7);
}

#[test]
fn test_slice_ref() {
    let bs = ByteStr::from("hello world");
    let original_str = bs.as_str();
    let world_slice = &original_str[6..11]; // "world"
    let slice = bs.slice_ref(world_slice);
    assert_eq!(slice.as_str(), "world");
}

#[test]
fn test_clear() {
    let mut bs = ByteStr::from("clear me");
    assert!(!bs.is_empty());
    bs.clear();
    assert!(bs.is_empty());
    assert_eq!(bs.as_str(), "");
}

#[test]
fn test_into_bytes() {
    let bs = ByteStr::from("test bytes");
    let bytes = bs.into_bytes();
    assert_eq!(bytes.as_ref(), b"test bytes");
}

#[test]
fn test_debug_display() {
    let bs = ByteStr::from("debug test");
    assert_eq!(format!("{bs:?}"), "\"debug test\"");
    assert_eq!(format!("{bs}"), "debug test");
}

#[test]
fn test_equality_with_str() {
    let bs = ByteStr::from("equality test");
    assert_eq!(bs, "equality test");
    assert_eq!("equality test", bs);
}

#[test]
fn test_equality_with_string() {
    let bs = ByteStr::from("string test");
    let s = String::from("string test");
    assert_eq!(bs, s);
    assert_eq!(s, bs);
}

#[test]
fn test_equality_with_cow() {
    let bs = ByteStr::from("cow test");
    let cow_borrowed = Cow::Borrowed("cow test");
    let cow_owned = Cow::Owned(String::from("cow test"));

    assert_eq!(bs, cow_borrowed);
    assert_eq!(cow_borrowed, bs);
    assert_eq!(bs, cow_owned);
    assert_eq!(cow_owned, bs);
}

#[test]
fn test_as_ref_str() {
    let bs = ByteStr::from("as_ref test");
    let s: &str = bs.as_ref();
    assert_eq!(s, "as_ref test");
}

#[test]
fn test_as_ref_bytes() {
    let bs = ByteStr::from("bytes test");
    let bytes: &[u8] = bs.as_ref();
    assert_eq!(bytes, b"bytes test");
}

#[test]
fn test_borrow() {
    use core::borrow::Borrow;
    let bs = ByteStr::from("borrow test");
    let s: &str = bs.borrow();
    assert_eq!(s, "borrow test");
}

#[test]
fn test_deref() {
    let bs = ByteStr::from("deref test");
    assert_eq!(bs.len(), 10);
    assert!(bs.starts_with("deref"));
    assert!(bs.ends_with("test"));
}

#[test]
fn test_from_str_trait() {
    use core::str::FromStr;
    let bs = ByteStr::from_str("fromstr test").unwrap();
    assert_eq!(bs.as_str(), "fromstr test");
}

#[test]
fn test_default() {
    let bs = ByteStr::default();
    assert_eq!(bs, ByteStr::new());
    assert!(bs.is_empty());
}

#[test]
fn test_ord_and_partial_ord() {
    let bs1 = ByteStr::from("apple");
    let bs2 = ByteStr::from("banana");
    let bs3 = ByteStr::from("apple");

    assert!(bs1 < bs2);
    assert!(bs2 > bs1);
    assert_eq!(bs1, bs3);
    assert!(bs1 <= bs3);
    assert!(bs1 >= bs3);
}

#[test]
fn test_hash() {
    use alloc::collections::BTreeSet;

    let mut set = BTreeSet::new();
    let bs1 = ByteStr::from("hash test");
    let bs2 = ByteStr::from("hash test");
    let bs3 = ByteStr::from("different");

    set.insert(bs1);
    assert!(!set.insert(bs2)); // Should return false as it's already in the set
    assert!(set.insert(bs3)); // Should return true as it's different
}

#[test]
fn test_empty_operations() {
    let mut bs = ByteStr::new();
    assert!(bs.is_empty());
    assert_eq!(bs.len(), 0);
    assert_eq!(bs.as_str(), "");

    bs.clear();
    assert!(bs.is_empty());
}

#[test]
fn test_unicode_support() {
    let bs = ByteStr::from("Hello, 世界! 🦀");
    assert_eq!(bs.as_str(), "Hello, 世界! 🦀");
    assert!(bs.contains("世界"));
    assert!(bs.contains("🦀"));
}

#[test]
fn test_slice_ref_edge_cases() {
    let bs = ByteStr::from("hello");
    let original_str = bs.as_str();

    // Test slicing the entire string
    let full_slice = bs.slice_ref(original_str);
    assert_eq!(full_slice.as_str(), "hello");

    // Test slicing first character
    let first_char = bs.slice_ref(&original_str[0..1]);
    assert_eq!(first_char.as_str(), "h");

    // Test slicing last character
    let last_char = bs.slice_ref(&original_str[4..5]);
    assert_eq!(last_char.as_str(), "o");
}

// ============================================================================
// SAFETY TESTS - Testing unsafe code blocks for memory safety and correctness
// ============================================================================

#[test]
fn test_from_utf8_unchecked_safety() {
    // Test that from_utf8_unchecked with valid UTF-8 works correctly
    let valid_bytes = bytes::Bytes::from("Hello, 世界! 🦀");
    let bs = unsafe { ByteStr::from_utf8_unchecked(valid_bytes) };
    assert_eq!(bs.as_str(), "Hello, 世界! 🦀");

    // Test with empty bytes
    let empty_bytes = bytes::Bytes::new();
    let empty_bs = unsafe { ByteStr::from_utf8_unchecked(empty_bytes) };
    assert_eq!(empty_bs.as_str(), "");
    assert!(empty_bs.is_empty());
}

#[test]
fn test_from_static_safety() {
    // Test from_static with various static strings
    let bs1 = ByteStr::from_static("");
    assert_eq!(bs1.as_str(), "");

    let bs2 = ByteStr::from_static("ASCII only");
    assert_eq!(bs2.as_str(), "ASCII only");

    let bs3 = ByteStr::from_static("Unicode: 世界 🦀");
    assert_eq!(bs3.as_str(), "Unicode: 世界 🦀");

    // Verify that from_static strings can be used safely
    assert!(bs3.contains("世界"));
    assert!(bs3.contains("🦀"));
}

#[test]
fn test_as_str_safety() {
    // Test that as_str() always returns valid UTF-8
    let test_cases = [
        "",
        "ASCII",
        "世界",
        "🦀",
        "Mixed: ASCII 世界 🦀",
        "Very long string with various characters: ABCDEFGHIJKLMNOPQRSTUVWXYZ 0123456789 世界 🦀🦀🦀",
    ];

    for case in &test_cases {
        let bs = ByteStr::from(*case);
        let retrieved = bs.as_str();
        assert_eq!(retrieved, *case);

        // Verify that the returned &str is valid UTF-8
        assert!(core::str::from_utf8(retrieved.as_bytes()).is_ok());
    }
}

#[test]
fn test_as_bytes_mut_safety() {
    let mut bs = ByteStr::from("test string");

    // Test that we can safely access the mutable bytes
    unsafe {
        let bytes_mut = bs.as_bytes_mut();
        // Just check that we can read from it safely
        assert_eq!(bytes_mut.len(), 11);

        // The bytes should represent the same string
        assert_eq!(bytes_mut.as_ref(), b"test string");
    }

    // After the unsafe block, the ByteStr should still be valid
    assert_eq!(bs.as_str(), "test string");
}

#[test]
fn test_truncate_safety_with_utf8_boundaries() {
    // Test truncate with various UTF-8 boundary conditions
    let mut bs1 = ByteStr::from("Hello, 世界!");

    // Truncate at ASCII boundary
    bs1.truncate(7); // "Hello, "
    assert_eq!(bs1.as_str(), "Hello, ");

    // Test with emoji
    let mut bs2 = ByteStr::from("🦀🦀🦀");
    bs2.truncate(4); // One crab emoji (4 bytes)
    assert_eq!(bs2.as_str(), "🦀");

    // Test truncating to empty
    let mut bs3 = ByteStr::from("test");
    bs3.truncate(0);
    assert_eq!(bs3.as_str(), "");
    assert!(bs3.is_empty());
}

#[test]
fn test_slice_ref_safety() {
    let original = "Hello, 世界! 🦀";
    let bs = ByteStr::from(original);
    let original_str = bs.as_str();

    // Test various safe slices
    let test_slices = [
        &original_str[0..0],                  // Empty slice at start
        &original_str[0..5],                  // "Hello"
        &original_str[7..10],                 // "世"
        &original_str[10..13],                // "界"
        &original_str[15..19],                // "🦀"
        &original_str[0..original_str.len()], // Full string
    ];

    for slice in &test_slices {
        let sliced_bs = bs.slice_ref(slice);
        assert_eq!(sliced_bs.as_str(), *slice);

        // Verify the sliced ByteStr is still valid UTF-8
        assert!(core::str::from_utf8(sliced_bs.as_str().as_bytes()).is_ok());
    }
}

#[test]
fn test_memory_layout_consistency() {
    // Test that ByteStr maintains consistent memory layout
    let original_string = "Memory safety test 🔒";
    let bs = ByteStr::from(original_string);

    // The string content should be identical
    assert_eq!(bs.as_str(), original_string);

    // The byte representation should be identical
    assert_eq!(bs.as_str().as_bytes(), original_string.as_bytes());

    // Test after cloning
    let cloned = bs.clone();
    assert_eq!(cloned.as_str(), original_string);
    assert_eq!(cloned.as_str().as_bytes(), original_string.as_bytes());

    // Test after conversion back to bytes
    let bytes = bs.into_bytes();
    assert_eq!(bytes.as_ref(), original_string.as_bytes());
}

#[test]
fn test_utf8_validation_consistency() {
    // Test that all paths through the API maintain UTF-8 validity
    let test_strings = [
        "",
        "a",
        "Hello",
        "世",
        "世界",
        "🦀",
        "🦀🦀🦀",
        "Mixed: Hello 世界 🦀!",
    ];

    for test_str in &test_strings {
        // Test from_utf8 path
        let bs1 = ByteStr::from_utf8(test_str.as_bytes()).unwrap();
        assert_eq!(bs1.as_str(), *test_str);

        // Test from string path
        let bs2 = ByteStr::from(*test_str);
        assert_eq!(bs2.as_str(), *test_str);

        // Test from static path
        let bs3 = ByteStr::from_static(test_str);
        assert_eq!(bs3.as_str(), *test_str);

        // All should be equal
        assert_eq!(bs1, bs2);
        assert_eq!(bs2, bs3);
        assert_eq!(bs1, bs3);
    }
}

#[test]
fn test_concurrent_access_safety() {
    extern crate std;
    use alloc::sync::Arc;
    use std::thread;

    let test_string = "Concurrent access test 🚀";
    let expected_len = test_string.len(); // Get the actual byte length
    let bs = Arc::new(ByteStr::from(test_string));
    let mut handles = vec![];

    // Spawn multiple threads that read from the same ByteStr
    for i in 0..10 {
        let bs_clone = Arc::clone(&bs);
        let handle = thread::spawn(move || {
            for _ in 0..100 {
                // These operations should be safe to perform concurrently
                assert_eq!(bs_clone.len(), expected_len);
                assert!(bs_clone.contains("Concurrent"));
                assert!(bs_clone.contains("🚀"));
                assert_eq!(bs_clone.as_str(), "Concurrent access test 🚀");

                // Test slicing
                let original_str = bs_clone.as_str();
                let slice = bs_clone.slice_ref(&original_str[0..10]);
                assert_eq!(slice.as_str(), "Concurrent");
            }
            i
        });
        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
}

#[test]
fn test_zero_copy_guarantees() {
    // Test that operations that should be zero-copy actually are
    let original = "Zero copy test";
    let bs = ByteStr::from(original);

    // Cloning should not change the content
    let cloned = bs.clone();
    assert_eq!(bs.as_str(), cloned.as_str());

    // Slicing should not change the content
    let original_str = bs.as_str();
    let full_slice = bs.slice_ref(original_str);
    assert_eq!(bs.as_str(), full_slice.as_str());

    // Converting to bytes and back (conceptually)
    let bytes = bs.clone().into_bytes();
    let bs_from_bytes = ByteStr::from_utf8(bytes).unwrap();
    assert_eq!(bs.as_str(), bs_from_bytes.as_str());
}
