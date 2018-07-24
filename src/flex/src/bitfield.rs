use std::io;

pub struct BitfieldSize {
    pub bits: u8,
    pub bytes: u8,
}
pub struct Bitfield {
    pub size: BitfieldSize,
    pub data: Vec<u8>,
}

pub fn get(data: &Vec<u8>) -> Result<Bitfield, io::Error> {
    if data.len() < 1 {
        let err = io::Error::new(io::ErrorKind::Other, "Not found bitfield size");
        return Err(err);
    }

    let mut bitfield = Bitfield {
        size: BitfieldSize {
            bits: data[0],
            bytes: data[0] / 8 + 1,
        },
        data: vec![],
    };

    if data[1..].len() < bitfield.size.bytes as usize {
        let err = io::Error::new(io::ErrorKind::Other, "Not found bitfield");
        return Err(err);
    }

    bitfield.data = data[1..=bitfield.size.bytes as usize].to_owned();

    Ok(bitfield)
}
