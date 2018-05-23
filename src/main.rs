extern crate bitreader;
extern crate scan_dir;
use bitreader::BitReader;
use scan_dir::ScanDir;
use std::fs::File;
use std::io::prelude::*;

fn main() {
  let path = std::env::args()
    .skip(1)
    .next()
    .expect("usage: flex-viewer PATH");

  println!("path: {}", path);

  ScanDir::files()
    .read(path, |files| {
      for (entry, name) in files {
        println!("read: {}", name);
        print_binary_data(read_binary_file(entry.path()));
      }
    })
    .unwrap();
}

fn read_binary_file(file: std::path::PathBuf) -> Vec<u8> {
  let mut data = vec![];
  let mut file = File::open(file).unwrap();
  let len = file.read_to_end(&mut data).unwrap();
  println!("read {} bytes", len);
  data
}

fn print_binary_data(bytes: Vec<u8>) {
  print!("Bitfield:");
  let mut bitfield = BitReader::new(&bytes[..11]);
  for _n in 0..84 {
    print!("{}", bitfield.read_u8(1).unwrap());
  }
  println!();
}
