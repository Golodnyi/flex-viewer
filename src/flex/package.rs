extern crate byteorder;

use bitfield::Bitfield;
use std::io;
use self::byteorder::{LittleEndian, ReadBytesExt};
use flex::Flex;

const SIGNED: u8 = 0;
const UNSIGNED: u8 = 1;
const ONE: u8 = 1;
const TWO: u8 = 2;
const FOUR: u8 = 4;

pub fn parse(data: Vec<u8>, bitfield: Bitfield, flex: Vec<Flex>) -> Result<(), io::Error> {
    let mut flex:Vec<Flex> = Vec::from(flex);
    let mut from: usize = (bitfield.size.bytes + 1) as usize;
    let mut to: usize;

    while data[from..].len() > 0 {
        for sensor in flex.iter_mut() {
            if !sensor.enable {
                continue;
            }

            if data[from..].len() < sensor.size as usize {
                let err = io::Error::new(io::ErrorKind::Other, "Ended data");
                return Err(err);
            }

            to = from + sensor.size as usize;
            let mut field = &data[from..to];

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
                     (data[from..].len() - sensor.size as usize),
                     sensor.name,
                     sensor.value);

            from += sensor.size as usize;
        }
    }

    Ok(())
}