extern crate scan_dir;
use scan_dir::ScanDir;
use std::fs::File;
use std::io::prelude::*;

fn main() {
  let path = std::env::args()
    .skip(1)
    .next()
    .expect("usage: flex-viewer PATH");

  println!("path: {}", path);

  let mut data: Vec<u8> = vec![];
  ScanDir::files()
    .read(path, |files| {
      for (entry, name) in files {
        println!("read: {}", name);
        data = read_binary_file(entry.path());
      }
    })
    .unwrap();

}

fn read_binary_file(file: std::path::PathBuf) -> Vec<u8> {
  let mut data: Vec<u8> = vec![];
  let mut file = File::open(file).expect("open file error");
  file.read_to_end(&mut data).expect("read error");
  data
}
