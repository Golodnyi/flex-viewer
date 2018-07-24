extern crate bitreader;
extern crate serde_json;

use self::bitreader::BitReader;
use bitfield::Bitfield;
use std::io;

static FLEX_CONFIG: &'static str = include_str!("../../../bitfield.json");

#[derive(Serialize, Deserialize)]
struct Config {
    name: String,
    size: u8,
    character: u8,
}

lazy_static! {
    static ref CONFIG: Vec<Config> = {
        let init = match serde_json::from_str(&FLEX_CONFIG) {
            Ok(data) => data,
            Err(e) => panic!("Error parse flex config: {:?}", e),
        };
        init
    };
}

pub struct Flex {
    pub name: String,
    pub size: u8,
    pub character: u8,
    pub enable: bool,
    pub value: f64,
}

pub fn get(bitfield: &Bitfield) -> Result<Vec<Flex>, io::Error> {
    let mut flex: Vec<Flex> = vec![];

    for conf in CONFIG.iter() {
        flex.push(Flex {
            name: conf.name.to_owned(),
            size: conf.size,
            character: conf.character,
            enable: false,
            value: 0 as f64,
        });
    }
    load_sensors(&mut flex, &bitfield);

    Ok(flex)
}

fn load_sensors(flex: &mut Vec<Flex>, bitfield: &Bitfield) {
    let mut bit_reader = BitReader::new(&bitfield.data);
    for i in 0..bitfield.size.bits {
        flex[i as usize].enable = match bit_reader.read_u8(1).unwrap() {
            1 => true,
            _ => false,
        }
    }
}
