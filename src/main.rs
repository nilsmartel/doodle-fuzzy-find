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

}
