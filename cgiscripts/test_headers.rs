fn main() {
    print!("Content-type:text/plain charset=utf8\n\n");
    for (k, v) in std::env::vars() {
        println!("{}: {}", k, v);
    }
}
