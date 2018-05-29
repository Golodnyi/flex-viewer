extern crate byteorder;

use std::io;
use self::byteorder::{LittleEndian, ReadBytesExt};
use flex::Flex;

const SIGNED: u8 = 0;
const UNSIGNED: u8 = 1;
const ONE: u8 = 1;
const TWO: u8 = 2;
const FOUR: u8 = 4;

pub fn parse(flex: &mut Flex, field: &[u8]) -> Result<(), io::Error> {
    let mut field = field;

    flex.value = match flex.size {
        ONE => {
            if flex.character == SIGNED {
                field.read_i8().unwrap() as f64
            } else {
                field.read_u8().unwrap() as f64
            }
        }
        TWO => {
            if flex.character == SIGNED {
                field.read_i16::<LittleEndian>().unwrap() as f64
            } else {
                field.read_u16::<LittleEndian>().unwrap() as f64
            }
        }
        FOUR => {
            if flex.character == SIGNED {
                field.read_i32::<LittleEndian>().unwrap() as f64
            } else if flex.character == UNSIGNED {
                field.read_u32::<LittleEndian>().unwrap() as f64
            } else {
                field.read_f32::<LittleEndian>().unwrap() as f64
            }
        }
        _ => {
            0 as f64
        }
    };

    Ok(())
}