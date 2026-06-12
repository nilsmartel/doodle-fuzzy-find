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
