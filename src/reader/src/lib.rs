use std::path::PathBuf;
use std::io;
use std::io::prelude::*;
use std::fs;
use std::fs::File;
use std::io::BufWriter;
use std::fs::OpenOptions;
use std::ffi::OsStr;

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

pub fn read_log_file(file: &PathBuf) -> Result<Vec<u8>, io::Error> {
    if check_extension(file, "log") {
        let err = io::Error::new(io::ErrorKind::Other, "Is not log file");
        return Err(err);
    }

    let mut data: Vec<u8> = vec![];
    let mut file = File::open(file)?;
    file.read_to_end(&mut data)?;
    Ok(data)
}

pub fn remove_reports(dir: PathBuf) -> Result<(), io::Error> {
    let files = read_dir(dir)?;

    for file in &files {
        if check_extension(file, "html") {
            fs::remove_file(file)?;
        }
    }

    Ok(())
}

pub fn write_report_file(path: String, data: String) -> Result<(), io::Error> {
    let mut path = PathBuf::from(path);
    path.set_extension("html");

    if path.is_file() {
        fs::remove_file(&path)?;
    }

    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(path)?;

    let mut buf = BufWriter::new(file);
    buf.write(data.as_bytes()).unwrap();

    Ok(())
}

fn check_extension(file: &PathBuf, ext: &str) -> bool {
    let extension = match file.extension() {
        Some(ext) => ext,
        None => {
            OsStr::new("None")
        }
    };

    extension == ext
}