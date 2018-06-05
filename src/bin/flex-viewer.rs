extern crate reader;
extern crate flex;
extern crate html;

use std::path::PathBuf;
use std::env::current_dir;
use flex::Flex;
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

    let files = reader::read_dir(path).expect("Cannot read dir");

    println!("0%");

    for (i, file) in files.iter().enumerate() {
        let data = match reader::read_binary_file(file) {
            Ok(data) => data,
            Err(_e) => {
                continue;
            }
        };
        match flex::parse(data) {
            Err(_e) => {
                continue;
            }
            Ok(flex) => {
                let mut flex: Vec<Flex> = Vec::from(flex);
                match html::append(&mut flex) {
                    Err(_e) => {
                        continue;
                    }
                    Ok(table) => {
                        tables.push(table);
                    }
                }
            }
        };
        let progress: usize = ((i as f32 + 1.0) / files.len() as f32 * 100.0) as usize;
        println!("{}[2J", 27 as char);
        let mut symbols: String = "".to_string();

        for p in 0..100 {
            if p <= progress {
                symbols.push_str("+");
            } else {
                symbols.push_str(" ");
            }
        }
        println!("{}% [{}]", progress, symbols);
    }

    let report: String = html::generate(&tables).expect("Error: generate report");
    reader::write_report_file("report.html", report).expect("Error: save report");
}