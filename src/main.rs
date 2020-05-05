type PrefixTable = Vec<Option<usize>>;

fn prepare(needle: &[u8]) -> PrefixTable {
    needle.iter().map(|_| None).collect()
}

fn search(needle: &[u8], prefix_table: &PrefixTable, haystack: &[u8]) -> Option<usize> {
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

#[cfg(test)] mod tests {
    use super::*;

    #[test]
    fn test_search() {
        let needle = b"ABCABABC";
        let prefix_table = vec![None, None, None, None, Some(1), Some(2), Some(1), Some(2)];

        assert_eq!(search(needle, &prefix_table, b"ABCABCABABC"), Some(3));
        assert_eq!(search(needle, &prefix_table, b"ABXABCABABC"), Some(3));
        assert_eq!(search(needle, &prefix_table, b"XABABCABABC"), Some(3));
    }
}

fn main() {
    println!("Hello, world!");
}
