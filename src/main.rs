#[macro_use]
extern crate serde_derive;
mod reader;
mod flex;
use std::path::PathBuf;
use std::env::current_exe;
fn main() {
    let mut path = PathBuf::new();
    match std::env::args().skip(1).next() {
        Some(p) => {
            path.push(p);
        }
        None => {
            let current_exe = current_exe().unwrap();
            path.push(current_exe.join("logs"));
        }
    }

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
        match flex::parse(&data) {
            Err(e) => {
                println!("Error: {}", e);
                continue;
            },
            Ok(_) => {}
        };
    }
}







