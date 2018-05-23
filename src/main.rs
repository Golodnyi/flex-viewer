fn main() {
    let path = std::env::args().skip(1).next()
        .expect("usage: flex-viewer PATH");

    println!("path: {}", path);
}
