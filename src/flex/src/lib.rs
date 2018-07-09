#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;
extern crate serde;

mod bitfield;
mod flex;
mod package;

use std::io;
use bitfield::Bitfield;
pub use flex::Flex;

pub fn parse(data: Vec<u8>) -> Result<(Vec<Flex>), io::Error> {
    if data.len() == 0 {
        let err = io::Error::new(io::ErrorKind::Other, "Not found bitfield size");
        return Err(err);
    }

    let bitfield: Bitfield = bitfield::get(&data)?;
    let mut flex_data: Vec<Flex> = vec![];
    let mut from: usize = (bitfield.size.bytes + 1) as usize;
    let mut to: usize;

    while data[from..].len() > 0 {
        let mut flex: Vec<Flex> = flex::get(&bitfield)?;

        for sensor in flex.iter_mut() {
            if !sensor.enable {
                continue;
            }

            if data[from..].len() < sensor.size as usize {
                let err = io::Error::new(io::ErrorKind::Other, "Ended data");
                return Err(err);
            }

            to = from + sensor.size as usize;
            package::parse(sensor, &data[from..to])?;

            from += sensor.size as usize;
        }

        flex_data.extend(flex);
    }

    Ok(flex_data)
}
