extern crate serde;
extern crate serde_json;
extern crate bitreader;

use std::io;
use self::bitreader::BitReader;

static FLEX_CONFIG: &'static str = include_str!("./../bitfield.json");

struct Flex {
    size: u8,
    character: u8,
    enable: bool,
    value: usize,
}

#[derive(Serialize, Deserialize)]
struct Config {
    size: u8,
    character: u8
}

struct BitfieldSize {
    bites: u8,
    bytes: u8,
}

fn get_sensors() -> Result<Vec<Flex>, io::Error> {
    let mut sensors: Vec<Flex> = vec![];
    let flex: Vec<Config> = serde_json::from_str(&FLEX_CONFIG)?;

    for f in flex {
        sensors.push(Flex { size: f.size, character: f.character, enable: false, value: 0 });
    }

    Ok(sensors)
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
        let mut sensors: Vec<Flex> = get_sensors()?;
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

            to = from + sensor.size as usize;
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
