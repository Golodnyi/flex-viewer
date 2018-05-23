extern crate scan_dir;
use scan_dir::ScanDir;

fn main() {
  let path = std::env::args()
    .skip(1)
    .next().expect("usage: flex-viewer PATH");

  println!("path: {}", path);

  ScanDir::files()
    .read(path, |files| {
      for (entry, name) in files {
        println!("File {:?} has full path {:?}", name, entry.path());
      }
    })
    .unwrap()
}
