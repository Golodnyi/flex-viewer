#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate bitreader;
extern crate byteorder;

use std::io;
use self::bitreader::BitReader;
use self::byteorder::{LittleEndian, ReadBytesExt};

const SIGNED: u8 = 0;
const UNSIGNED: u8 = 1;
const ONE: u8 = 1;
const TWO: u8 = 2;
const FOUR: u8 = 4;
mod config;

pub struct Flex {
    name: String,
    size: u8,
    character: u8,
    enable: bool,
    value: f64,
}

struct BitfieldSize {
    bites: u8,
    bytes: u8,
}

pub fn parse(bytes: &Vec<u8>) -> Result<(), io::Error> {
    if bytes.len() == 0 {
        let err = io::Error::new(io::ErrorKind::Other, "Not found bitfield size");
        return Err(err);
    }

    let bitfield_size: BitfieldSize = BitfieldSize { bites: bytes[0], bytes: bytes[0] / 8 + 1 };

    println!("Bitfield size: {} bits ({} bytes)", bitfield_size.bites, bitfield_size.bytes);

    if bytes[1..].len() < bitfield_size.bytes as usize {
        let err = io::Error::new(io::ErrorKind::Other, "Not found bitfield");
        return Err(err);
    }

    let mut from: usize = (bitfield_size.bytes + 1) as usize;
    let mut to: usize;

    while bytes[from..].len() > 0 {
        let mut sensors: Vec<Flex> = config::get()?;
        let mut bitfield = BitReader::new(&bytes[1..=(bitfield_size.bytes as usize)]);
        for i in 0..bitfield_size.bites {
            sensors[i as usize].enable = match bitfield.read_u8(1).unwrap() {
                1 => true,
                _ => false
            }
        }

        for mut sensor in sensors {
            if !sensor.enable {
                continue;
            }

            if bytes[from..].len() < sensor.size as usize {
                let err = io::Error::new(io::ErrorKind::Other, "Ended data");
                return Err(err);
            }

            to = from + sensor.size as usize;
            let mut field = &bytes[from..to];

            sensor.value = match sensor.size {
                ONE => {
                    if sensor.character == SIGNED {
                        field.read_i8().unwrap() as f64
                    } else {
                        field.read_u8().unwrap() as f64
                    }
                }
                TWO => {
                    if sensor.character == SIGNED {
                        field.read_i16::<LittleEndian>().unwrap() as f64
                    } else {
                        field.read_u16::<LittleEndian>().unwrap() as f64
                    }
                }
                FOUR => {
                    if sensor.character == SIGNED {
                        field.read_i32::<LittleEndian>().unwrap() as f64
                    } else if sensor.character == UNSIGNED {
                        field.read_u32::<LittleEndian>().unwrap() as f64
                    } else {
                        field.read_f32::<LittleEndian>().unwrap() as f64
                    }
                }
                _ => {
                    0 as f64
                }
            };

            println!("read from: {}, to: {}, total: {}, {}: {}",
                     from, sensor.size as usize,
                     (bytes[from..].len() - sensor.size as usize),
                     sensor.name,
                     sensor.value);

            from += sensor.size as usize;
        }
    }
    Ok(())
}
