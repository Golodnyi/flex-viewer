mod reader;
mod flex;
use std::path::PathBuf;

fn main() {
    let mut path = PathBuf::new();
    path.push(std::env::args().skip(1).next()
        .expect("usage: flex-viewer PATH")
    );

    println!("path: {:?}", path);
    let files = reader::read_dir(path).expect("Cannot read dir");
    for file in files.iter() {
        let data = match reader::read_binary_file(file) {
            Ok(data) => data,
            Err(e) => {
                println!("Error: {}", e);
                continue;
            }
        };
        match flex::bitfield_parse(&data) {
            Err(e) => {
                println!("Error: {}", e);
                continue;
            },
            Ok(_) => {}
        };
    }
}







