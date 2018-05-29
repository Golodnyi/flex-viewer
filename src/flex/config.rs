extern crate serde_json;

use Flex;
use std::io;

static FLEX_CONFIG: &'static str = include_str!("./../../bitfield.json");

#[derive(Serialize, Deserialize)]
struct Config {
    name: String,
    size: u8,
    character: u8,
}

pub fn get() -> Result<Vec<Flex>, io::Error> {
    let mut sensors: Vec<Flex> = vec![];
    let flex: Vec<Config> = serde_json::from_str(&FLEX_CONFIG)?;

    for f in flex {
        sensors.push(Flex { name: f.name, size: f.size, character: f.character, enable: false, value: 0 as f64 });
    }

    Ok(sensors)
}