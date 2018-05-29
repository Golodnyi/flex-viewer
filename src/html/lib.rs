#[macro_use]
extern crate horrorshow;
extern crate flex;

use horrorshow::helper::doctype;
use std::fs::File;
use std::io::prelude::*;
use std::io;
use flex::Flex;

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

    let html = format!("{}", html! {: doctype::HTML;
    html {
        head {
            title: "Report";
            style: "table, th, td { border: 1px solid black; }"
        }
        body {
            h1: "Report";
            table {
                thead {
                    tr {
                        @ for key in &data_table.header {
                            th: key
                        }
                    }
                }
            }
        }
    }});

    let mut report = File::create("report.html").expect("Error create file");
    report.write_all(html.as_bytes()).expect("Error write report");

    Ok(())
}