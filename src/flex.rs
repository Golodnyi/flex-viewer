extern crate bitreader;

use std::io;
use self::bitreader::BitReader;

const SIGNED: u8 = 0;
const UNSIGNED: u8 = 1;
const FLOAT: u8 = 2;

struct Flex {
    size: u8,
    character: u8,
    enable: bool,
    value: usize,
}

struct BitfieldSize {
    bites: u8,
    bytes: u8,
}

fn get_sensors() -> Vec<Flex> {
    let mut sensors: Vec<Flex> = vec![];

    let size: Vec<u8> = vec![
        4, 2, 4, 1, 1, 1, 1, 1, 4, 4, 4, 4, 4, 2, 4, 4, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1, 1, 1,
        1, 4, 4, 2, 2, 4, 2, 2, 2, 2, 2, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 2, 4, 2, 1, 4, 2, 2, 2, 2, 2,
        1, 1, 1, 2, 4, 2, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2
    ];

    let character: Vec<u8> = vec![
        UNSIGNED, UNSIGNED, UNSIGNED, UNSIGNED, UNSIGNED, UNSIGNED, UNSIGNED, UNSIGNED, UNSIGNED,
        SIGNED, SIGNED, SIGNED, FLOAT, UNSIGNED, FLOAT, FLOAT, UNSIGNED, UNSIGNED, UNSIGNED,
        UNSIGNED, UNSIGNED, UNSIGNED, UNSIGNED, UNSIGNED, UNSIGNED, UNSIGNED, UNSIGNED, UNSIGNED,
        UNSIGNED, UNSIGNED, UNSIGNED, UNSIGNED, UNSIGNED, UNSIGNED, UNSIGNED, UNSIGNED, UNSIGNED,
        UNSIGNED, UNSIGNED, UNSIGNED, UNSIGNED, UNSIGNED, UNSIGNED, UNSIGNED, SIGNED, SIGNED,
        SIGNED, SIGNED, SIGNED, SIGNED, SIGNED, SIGNED, UNSIGNED, FLOAT, UNSIGNED, SIGNED, FLOAT,
        UNSIGNED, UNSIGNED, UNSIGNED, UNSIGNED, UNSIGNED, UNSIGNED, UNSIGNED, UNSIGNED, UNSIGNED,
        UNSIGNED, SIGNED, UNSIGNED, UNSIGNED, UNSIGNED, UNSIGNED, UNSIGNED, UNSIGNED, UNSIGNED,
        UNSIGNED, UNSIGNED, UNSIGNED, UNSIGNED, UNSIGNED, UNSIGNED, UNSIGNED, UNSIGNED, UNSIGNED,
        UNSIGNED,
    ];

    for i in 0..85 {
        sensors.push(Flex { size: size[i as usize], character: character[i as usize], enable: false, value: 0 });
    }

    sensors
}

pub fn parse(bytes: &Vec<u8>) -> Result<(), io::Error> {
    if bytes.len() == 0 {
        let err = io::Error::new(io::ErrorKind::Other, "Not found bitfield size");
        return Err(err);
    }

    let mut bitfield_size: BitfieldSize = BitfieldSize { bites: bytes[0], bytes: bytes[0] / 8 + 1 };

    println!("Bitfield size: {} bits ({} bytes)", bitfield_size.bites, bitfield_size.bytes);

    if bytes[1..].len() < bitfield_size.bytes as usize {
        let err = io::Error::new(io::ErrorKind::Other, "Not found bitfield");
        return Err(err);
    }

    let mut from: usize = (bitfield_size.bytes + 1) as usize;
    let mut to: usize = from;

    while bytes[from..].len() > 0 {
        let mut sensors: Vec<Flex> = get_sensors();
        let mut bitfield = BitReader::new(&bytes[1..=(bitfield_size.bytes as usize)]);
        for i in 0..bitfield_size.bites {
            sensors[i as usize].enable = match bitfield.read_u8(1).unwrap() {
                1 => true,
                _ => false
            }
        }

        for sensor in sensors {
            if !sensor.enable {
                continue;
            }

            if bytes[from..].len() < sensor.size as usize {
                let err = io::Error::new(io::ErrorKind::Other, "Ended data");
                return Err(err);
            }

            to = (from + sensor.size as usize);
            let field = &bytes[from..to];

            println!("read from: {}, to: {}, total: {}, value: {:?}",
                     from, sensor.size as usize,
                     (bytes[from..].len() - sensor.size as usize),
                     field);

            from += sensor.size as usize;
        }
    }
    Ok(())
}
