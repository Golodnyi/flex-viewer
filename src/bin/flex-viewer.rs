extern crate reader;
extern crate flex;
extern crate html;

use std::path::PathBuf;
use std::env::current_dir;
use flex::Flex;

fn main() {
    let mut flex_data: Vec<Flex> = vec![];
    let mut path = PathBuf::new();

    match std::env::args().skip(1).next() {
        Some(p) => {
            path.push(p);
        }
        None => {
            let current_dir = current_dir().unwrap();
            path.push(current_dir.join("logs"));
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
        match flex::parse(data) {
            Err(e) => {
                println!("Error: {}", e);
                continue;
            },
            Ok(flex) => {
                flex_data = flex;
            }
        };
    }

    match html::report(&mut flex_data) {
        Err(e) => {
            println!("Error: {}", e);
        },
        Ok(_) => {}
    }
}