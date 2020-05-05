#[cfg(test)] extern crate quickcheck;
#[cfg(test)] extern crate quickcheck_macros;

type PrefixTable = Vec<Option<usize>>;

fn prepare(needle: &[u8]) -> PrefixTable {
    let mut prefix_len: Option<usize> = None;
    let mut prefix_table: PrefixTable = Vec::with_capacity(needle.len());

    // Don't try to match from the beginning
    prefix_table.push(None);

    for i in 1..needle.len() {
        // Push prefix_len at index i.
        prefix_table.push(prefix_len);

        // Now we have prefix_table filled up to (including) i.
        // We're computing prefix_len for i+1.

        // Index of the next expected character of the current prefix.
        // If we don't have a prefix yet, we start at 0.
        let match_index = prefix_len.unwrap_or(0);

        prefix_len = if needle[i] == needle[match_index] {
            // Extend the current prefix (including an empty prefix).
            Some(match_index + 1)
        } else {
            match prefix_table[match_index] {
                // Try to extend next prefix
                Some(len) if needle[i] == needle[len] => Some(len + 1), // FIXME: For some reason when I comment this, QuickCheck does not fail.

                // Otherwise start from the beginning
                _ if needle[i] == needle[0] => Some(1),

                _ => None,
            }
        };
    }
    prefix_table
}

fn search(needle: &[u8], prefix_table: &PrefixTable, haystack: &[u8]) -> Option<usize> {
    if needle.len() == 0 {
        return Some(0);
    }

    let mut s: usize = 0;
    let mut i: usize = 0;
    loop {
        if i >= needle.len() {
            return Some(s);
        }
        if s + i >= haystack.len() {
            return None;
        }
        if needle[i] == haystack[s + i] {
            i += 1;
        } else {
            match prefix_table[i] {
                None => {
                    s += std::cmp::max(1, i);
                    i = 0;
                }
                Some(prefix_len) => {
                    s += i - prefix_len;
                    i = prefix_len;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::*;

    #[test]
    fn test_prepare() {
        assert_eq!(prepare(b"AAA"), vec![None, None, Some(1)]);
        assert_eq!(prepare(b"ABC"), vec![None, None, None]);
        assert_eq!(
            prepare(b"ABABABXYZ"),
            vec![None, None, None, Some(1), Some(2), Some(3), Some(4), None, None]
        );
        assert_eq!(
            prepare(b"ABCABABC"),
            vec![None, None, None, None, Some(1), Some(2), Some(1), Some(2)]
        );
    }

    #[test]
    fn test_search() {
        let needle = b"ABCABABC";
        let prefix_table = vec![None, None, None, None, Some(1), Some(2), Some(1), Some(2)];

        assert_eq!(search(needle, &prefix_table, b"ABCABCABABC"), Some(3));
        assert_eq!(search(needle, &prefix_table, b"ABXABCABABC"), Some(3));
        assert_eq!(search(needle, &prefix_table, b"XABABCABABC"), Some(3));
    }

    fn naive_search(needle: &[u8], haystack: &[u8]) -> Option<usize> {
        for i in 0..haystack.len() {
            if i + needle.len() <= haystack.len() && haystack[i..i+needle.len()] == *needle {
                return Some(i);
            }
        }
        None
    }

    quickcheck! {
        fn search_matches_naive_search(needle: Vec<u8>, haystack: Vec<u8>) -> TestResult {
            let expected = naive_search(&needle, &haystack);
            let actual = search(&needle, &prepare(&needle), &haystack);
            if expected != actual {
                TestResult::error(format!("Expected: {:?}, actual: {:?}", expected, actual))
            } else {
                TestResult::passed()
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
}
