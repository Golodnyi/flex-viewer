#[macro_use]
extern crate horrorshow;
extern crate flex;

use std::io::prelude::*;
use std::io;
use flex::Flex;
use std::fs::OpenOptions;
use std::io::BufWriter;

struct Table {
    header: Vec<String>,
}

fn parse_table(flex: &mut Vec<Flex>) -> Result<Table, io::Error> {
    let mut header: Vec<String> = vec![];
    let mut i = 0;
    for sensor in flex {
        if i >= 85 {
            break;
        }
        i += 1;
        if !sensor.enable {
            continue;
        }

        header.push(sensor.name.clone());
    }

    let table = Table { header: header };

    Ok(table)
}

pub fn report(flex: &mut Vec<Flex>) -> Result<(), io::Error> {
    let data_table: Table = parse_table(flex)?;

    let html = format!("{}", html! {
        table(style="border: 1px solid black;") {
            thead {
                tr {
                    @ for key in &data_table.header {
                        th(style="border: 1px solid black;"): key
                    }
                }
            }
            tbody {
                @ for i in 0..flex.len()/85 {
                    tr {
                        @ for j in i*85..(i+1)*85 {
                            @ if flex[j].enable {
                                td(style="border: 1px solid black;"): &flex[j].value
                            }
                        }
                    }
                }
            }
        }
    });

    let file = OpenOptions::new().write(true).create(true).append(true).open("report.html")?;
    let mut buf = BufWriter::new(file);
    buf.write(&html.as_bytes()).unwrap();
    Ok(())
}