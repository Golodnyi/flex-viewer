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

    for sensor in flex {
        if !sensor.enable {
            continue;
        }

        header.push(sensor.name.clone());
    }

    header.sort();
    header.dedup();
    header.reverse();

    let table = Table { header: header };

    Ok(table)
}

pub fn report(flex: &mut Vec<Flex>) -> Result<(), io::Error> {
    let data_table: Table = parse_table(flex)?;

    let html = format!("{}", html! {
        table {
            thead {
                tr {
                    @ for key in &data_table.header {
                        th: key
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