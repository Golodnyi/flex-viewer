#[macro_use]
extern crate serde_derive;
extern crate serde;

mod bitfield;
mod flex;
mod package;

use std::io;
use bitfield::Bitfield;
use flex::Flex;

pub fn parse(data: Vec<u8>) -> Result<(), io::Error> {
    if data.len() == 0 {
        let err = io::Error::new(io::ErrorKind::Other, "Not found bitfield size");
        return Err(err);
    }

    let bitfield: Bitfield = bitfield::get(&data)?;
    let mut sensors: Vec<Flex> = flex::get(&bitfield)?;
    package::parse(&data, &bitfield, &mut sensors)?;

    Ok(())
}
