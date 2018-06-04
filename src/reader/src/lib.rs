use std::path::PathBuf;
use std::io;
use std::io::prelude::*;
use std::fs;
use std::fs::File;

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
