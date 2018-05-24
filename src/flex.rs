extern crate bitreader;

use std::io;
use self::bitreader::BitReader;

pub fn bitfield_parse(bytes: &Vec<u8>) -> Result<(), io::Error> {
    if bytes.len() == 0 {
        let err = io::Error::new(io::ErrorKind::Other, "Not found bitfield size");
        return Err(err);
    }

    let bitfield_size = bytes[0] / 8 + 1;
    println!("Bitfield size: {} bits ({} bytes)", bytes[0], bitfield_size);

    if bytes[1..].len() < bitfield_size as usize {
        let err = io::Error::new(io::ErrorKind::Other, "Not found bitfield");
        return Err(err);
    }

    let mut bitfield = BitReader::new(&bytes[1..=(bitfield_size as usize)]);
    for _i in 0..bytes[0] {
        print!("{}", bitfield.read_u8(1).unwrap());
    }

    println!();
    Ok(())
}
