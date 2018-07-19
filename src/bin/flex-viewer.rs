extern crate flex;
extern crate html;
extern crate reader;

use flex::Flex;
use std::env::current_dir;
use std::path::PathBuf;
use std::process;

fn main() {
    let mut path = PathBuf::new();
    let dir = current_dir().unwrap();
    reader::remove_reports(dir).expect("Error: remove reports");

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

    if files.len() == 0 {
        println!("Empty log files");
        process::exit(0);
    }

    progress_bar(0);

    for (i, file) in files.iter().enumerate() {
        let data = match reader::read_log_file(file) {
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
                match html::generate(&mut flex) {
                    Err(_e) => {
                        continue;
                    }
                    Ok((date, template)) => {
                        reader::write_report_file(date, template).expect("Error: save report");
                    }
                }
            }
        };
        let progress: usize = ((i as f32 + 1.0) / files.len() as f32 * 100.0) as usize;
        progress_bar(progress);
    }
}

fn progress_bar(progress: usize) {
    if cfg!(target_os = "linux") {
        println!("{}[2J", 27 as char);
    }

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
