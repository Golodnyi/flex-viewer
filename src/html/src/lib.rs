#[macro_use]
extern crate horrorshow;
#[macro_use]
extern crate lazy_static;
extern crate chrono;
extern crate flex;
extern crate serde_json;

use chrono::prelude::*;
use flex::Flex;
use horrorshow::prelude::*;
use std::io;
use std::usize;

static JS_CODE: &'static str = include_str!("../../../template.js");
static ALLOWED_SENSORS_JSON: &'static str = include_str!("../../../allowedSensors.json");
static STYLES: &'static str = include_str!("../../../styles.css");

lazy_static! {
    static ref ALLOWED_SENSORS: Vec<String> = {
        let init = match serde_json::from_str(&ALLOWED_SENSORS_JSON) {
            Ok(data) => data,
            Err(e) => panic!("Error cannot parse allowed sensors: {:?}", e),
        };

        init
    };
}

pub struct Table {
    pub header: Vec<String>,
    pub body: String,
    pub date: String,
}

fn template() -> String {
    let template: String = format!(
        "{}",
        html! {
            html {
                head {
                    meta(charset="UTF-8");
                    title: "Report";
                    style: Raw(&STYLES);
                }
                body {
                    table { }
                    script: Raw(&JS_CODE)
                }
            }
        }
    );

    template
}

fn is_allowed_sensor(name: &String) -> Result<bool, io::Error> {
    let val = match ALLOWED_SENSORS.iter().position(|r| r == name) {
        Some(x) => x,
        None => usize::MAX,
    };

    if val == usize::MAX {
        return Ok(false);
    }

    Ok(true)
}

fn parse_table(flex: &mut Vec<Flex>) -> Result<Table, io::Error> {
    let mut header: Vec<String> = vec![];
    let mut i = 0;
    for sensor in flex.iter_mut() {
        if i >= 85 {
            break;
        }
        i += 1;

        if !sensor.enable || !is_allowed_sensor(&sensor.name)? {
            continue;
        }

        header.push(sensor.name.clone());
    }

    let timestamp = flex[2].value;
    let mut table = Table {
        header: header,
        body: "".to_owned(),
        date: NaiveDateTime::from_timestamp(timestamp as i64, 0)
            .format("%Y-%m-%d-%H-%M-%S")
            .to_string(),
    };

    table.body = format!(
        "{}",
        html! {
            table(id="table", class="table") {
                thead {
                    tr {
                        @ for key in &table.header {
                            th: key
                        }
                    }
                }
                tbody {
                    @ for i in 0..flex.len()/85 {
                        tr {
                            @ for j in i*85..(i+1)*85 {
                                @ if flex[j].enable && is_allowed_sensor(&flex[j].name).unwrap() {
                                    td(value=&flex[j].value)
                                }
                            }
                        }
                    }
                }
            }
        }
    );

    Ok(table)
}

pub fn generate(flex: &mut Vec<Flex>) -> Result<(String, String), io::Error> {
    let table: Table = parse_table(flex)?;

    let template: String = template();
    let template: String = template.replace("<table></table>", &table.body);

    Ok((table.date, template))
}
