#[macro_use]
extern crate horrorshow;
extern crate flex;
extern crate serde_json;

use flex::Flex;
use horrorshow::prelude::*;
use std::fs::OpenOptions;
use std::io;
use std::io::BufWriter;
use std::io::prelude::*;
use std::usize;

static JS_CODE: &'static str = include_str!("../../../template.js");
static ALLOWED_SENSORS: &'static str = include_str!("../../../allowedSensors.json");

pub struct Table {
    pub header: Vec<String>,
    pub body: String,
}

fn template() -> String {
    let template: String = format!(
        "{}",
        html! {
            html {
                head {
                    meta(charset="UTF-8");
                    title: "Report";
                    style: "table, th, td { border: 1px solid black; }";
                }
                body {
                    h1 {
                        : "Report";
                    }

                    table { }
                    script: Raw(&JS_CODE)
                }
            }
        }
    );

    template
}

fn is_allowed_sensor(name: &String) -> Result<bool, io::Error> {
    let allowed_sensors: Vec<String> = serde_json::from_str(&ALLOWED_SENSORS)?;

    let val = match allowed_sensors.iter().position(|r| r == name) {
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
    for sensor in flex {
        if i >= 85 {
            break;
        }
        i += 1;

        if !sensor.enable || !is_allowed_sensor(&sensor.name)? {
            continue;
        }

        header.push(sensor.name.clone());
    }

    let table = Table {
        header: header,
        body: "".to_owned(),
    };

    Ok(table)
}

pub fn append(flex: &mut Vec<Flex>) -> Result<Table, io::Error> {
    let mut table: Table = parse_table(flex)?;

    table.body = format!(
        "{}",
        html! {
            table(id="table") {
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

pub fn generate(tables: &Vec<Table>) -> Result<(), io::Error> {
    let template: String = template();
    let mut body: String = "".to_owned();
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open("report.html")?;
    let mut buf = BufWriter::new(file);
    for table in tables {
        body.push_str(&table.body);
    }

    let template: String = template.replace("<table></table>", &body);
    buf.write(template.as_bytes()).unwrap();

    Ok(())
}
