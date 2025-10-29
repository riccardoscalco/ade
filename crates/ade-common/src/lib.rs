/// Error message used when validating key sequences in graph structures.
///
/// This constant provides a standard error message for cases where a collection of keys
/// does not form a valid sequential sequence. Valid key sequences must start from 0
/// and continue without gaps up to n-1, where n is the total number of keys.
///
/// # Expected Format
///
/// A valid key sequence should be:
/// - Sequential integers
/// - Starting from 0
/// - Ending at n-1 (where n is the count of keys)
/// - Without gaps or duplicates
///
/// # Examples
///
/// Valid sequences:
/// - `[0, 1, 2, 3]` (4 keys)
/// - `[0, 1]` (2 keys)
/// - `[0]` (1 key)
///
/// Invalid sequences:
/// - `[1, 2, 3]` (doesn't start from 0)
/// - `[0, 1, 3]` (missing 2)
/// - `[0, 1, 1, 2]` (duplicate key)
pub const INVALID_KEY_SEQUENCE: &str =
    "Invalid key sequence: keys must be sequential integers from 0 to n-1";

#[cfg(feature = "test-helpers")]
#[macro_export]
macro_rules! assert_panics_with {
    ($expr:expr, $expected:expr $(,)?) => {{
        let res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| $expr));
        match res {
            Ok(_) => panic!(
                "expected panic with {:?}, but code did not panic",
                $expected
            ),
            Err(err) => {
                let msg = err
                    .downcast_ref::<String>()
                    .map(String::as_str)
                    .or_else(|| err.downcast_ref::<&'static str>().copied());
                assert_eq!(msg, Some($expected), "panic message mismatch");
            }
        }
    }};
}
