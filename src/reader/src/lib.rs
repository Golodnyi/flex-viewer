use std::path::PathBuf;
use std::io;
use std::io::prelude::*;
use std::fs;
use std::fs::File;
use std::io::BufWriter;
use std::fs::OpenOptions;

pub fn read_dir(dir: PathBuf) -> Result<Vec<PathBuf>, io::Error> {
    let mut files: Vec<PathBuf> = vec![];

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            continue;
        }

        files.push(entry.path());
    }

    Ok(files)
}

pub fn read_binary_file(file: &PathBuf) -> Result<Vec<u8>, io::Error> {
    let mut data: Vec<u8> = vec![];
    let mut file = File::open(file)?;
    file.read_to_end(&mut data)?;
    Ok(data)
}

pub fn write_report_file(path: &str, data: String) -> Result<(), io::Error> {
    let path = PathBuf::from(path);

    match fs::remove_file(&path) {
        Err(_e) => {},
        _ => {}
    };

    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(path)?;

    let mut buf = BufWriter::new(file);
    buf.write(data.as_bytes()).unwrap();

    Ok(())
}
