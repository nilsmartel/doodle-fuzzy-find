fn main() {
    let Some(key) = std::env::args().nth(1) else {
        eprintln!("expect first argument to be search key and string input via stdin");
        std::process::exit(0)
    };

    let lines = std::io::stdin().lines();

    for l in lines {
        let text = l.expect("read stdin");

        if is_match(&key.as_bytes(), text.as_bytes()) {
            println!("{text}");
        }
    }
}


fn is_match(key: &[u8], text: &[u8]) -> bool {
    if key.is_empty() {
        return true;
    }

    let token = key[0];
    let key = &key[1..];

    if let Some(index) = text.iter().position(|elem| *elem == token) {
        is_match(key, &text[index..])
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_key_returns_true() {
        assert!(is_match(&[], &[1, 2, 3]));
        assert!(is_match(&[], &[]));
    }

    #[test]
    fn test_single_char_match() {
        assert!(is_match(&[b'a'], &[b'a', b'b', b'c']));
        assert!(is_match(&[b'x'], &[b'x']));
    }

    #[test]
    fn test_single_char_no_match() {
        assert!(!is_match(&[b'a'], &[b'b', b'c', b'd']));
        assert!(!is_match(&[b'x'], &[]));
    }

    #[test]
    fn test_multiple_chars_match_sequential() {
        assert!(is_match(&[b'a', b'b', b'c'], &[b'a', b'b', b'c', b'd']));
    }

    #[test]
    fn test_multiple_chars_match_with_gaps() {
        assert!(is_match(&[b'a', b'c', b'e'], &[b'a', b'b', b'c', b'd', b'e']));
        assert!(is_match(&[b'1', b'3', b'5'], &[b'1', b'2', b'3', b'4', b'5']));
    }

    #[test]
    fn test_multiple_chars_no_match() {
        assert!(!is_match(&[b'a', b'b', b'c'], &[b'a', b'b', b'd']));
        assert!(!is_match(&[b'x', b'y'], &[b'a', b'x', b'b']));
    }

    #[test]
    fn test_match_at_end_of_text() {
        assert!(is_match(&[b'c', b'd'], &[b'a', b'b', b'c', b'd']));
        assert!(is_match(&[b'z'], &[b'a', b'b', b'c', b'z']));
    }

    #[test]
    fn test_duplicate_characters_in_key() {
        assert!(is_match(&[b'a', b'a', b'a'], &[b'a', b'a', b'a']));
        assert!(is_match(&[b'a', b'a', b'a'], &[b'a', b'b', b'a', b'c', b'a']));
    }

    #[test]
    fn test_duplicate_characters_in_text() {
        assert!(is_match(&[b'a', b'b'], &[b'a', b'a', b'b']));
        assert!(is_match(&[b'b', b'a'], &[b'b', b'b', b'a']));
    }

    #[test]
    fn test_overlapping_matches() {
        // Should match first 'a' with key[0], then remaining key "aa" with text "aa"
        assert!(is_match(&[b'a', b'a'], &[b'a', b'a', b'a']));

        // Should match first 'a' at position 0, then second 'a' at position 2
        assert!(is_match(&[b'a', b'a'], &[b'a', b'b', b'a']));
    }

    #[test]
    fn test_no_match_when_text_shorter() {
        assert!(!is_match(&[b'a', b'b', b'c'], &[b'a', b'b']));
        assert!(!is_match(&[b'x', b'y'], &[b'x']));
    }

    #[test]
    fn test_match_with_empty_text_and_nonempty_key() {
        assert!(!is_match(&[b'a'], &[]));
        assert!(!is_match(&[b'1', b'2'], &[]));
    }

    #[test]
    fn test_match_with_various_byte_values() {
        // Null byte
        assert!(is_match(&[0], &[0, 1, 2]));

        // Special characters
        assert!(is_match(&[b'\n', b'\t'], &[b'\n', b' ', b'\t']));

        // High ASCII values
        assert!(is_match(&[255], &[254, 255]));

        // Mix of different bytes
        assert!(is_match(&[b'A', 0, b'Z'], &[b'A', b'B', 0, b'Z']));
    }

    #[test]
    fn test_long_sequences() {
        let key: Vec<u8> = (0..100).map(|i| i as u8).collect();
        let text: Vec<u8> = (0..200).map(|i| i as u8).collect();
        assert!(is_match(&key, &text));

        // Should fail if text doesn't contain all key elements in order
        let mut incomplete_text = text.clone();
        incomplete_text[50] = 200; // Change one byte that should be in key
        // This might still match if the changed byte wasn't needed for the sequence
        // Let's make a more reliable failure case: remove the first element
        let incomplete_text = &text[1..];
        assert!(!is_match(&key, incomplete_text));
    }

    #[test]
    fn test_match_finds_earliest_positions() {
        // Key: 'a', 'b'
        // Text: 'a', 'x', 'a', 'b'
        // Should match 'a' at pos 0, then 'b' at pos 3
        assert!(is_match(&[b'a', b'b'], &[b'a', b'x', b'a', b'b']));

        // Should match even though there's an earlier 'b' before the second 'a'
        assert!(is_match(&[b'a', b'b'], &[b'a', b'b', b'a', b'b']));
    }

    #[test]
    fn test_no_match_when_characters_out_of_order() {
        assert!(!is_match(&[b'a', b'b'], &[b'b', b'a']));
        assert!(!is_match(&[b'1', b'2', b'3'], &[b'3', b'2', b'1']));
    }

    #[test]
    fn test_key_is_subsequence_not_substring() {
        // Key is a subsequence, not a contiguous substring
        assert!(is_match(&[b'a', b'c', b'e'], &[b'a', b'b', b'c', b'd', b'e']));
    }
}
