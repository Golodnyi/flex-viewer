extern crate reader;
extern crate flex;
extern crate html;

use std::path::PathBuf;
use std::env::current_dir;
use flex::Flex;
use std::fs;
use html::Table;

fn main() {
    let mut path = PathBuf::new();
    let mut tables: Vec<Table> = vec![];

    match std::env::args().skip(1).next() {
        Some(p) => {
            path.push(p);
        }
        None => {
            let current_dir = current_dir().unwrap();
            path.push(current_dir.join("logs"));
        }
    }

    match fs::remove_file("report.html") {
        Err(e) => {
            println!("Error: {}", e);
        },
        _ => {}
    }

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
                let mut flex: Vec<Flex> = Vec::from(flex);
                match html::append(&mut flex) {
                    Err(e) => {
                        println!("Error: {}", e);
                        continue;
                    },
                    Ok(table) => {
                        tables.push(table);
                    }
                }
            }
        };
    }

    html::generate(&tables).expect("Error: generate report");
}