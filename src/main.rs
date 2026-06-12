fn main() {
    let Some(key) = std::env::args().nth(1) else {
        eprintln!("expect first argument to be search key and string input via stdin");
        std::process::exit(0)
    };

    let lines = std::io::stdin().lines();

    for l in lines {
    }
}
}
