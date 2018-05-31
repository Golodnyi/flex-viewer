#[macro_use]
extern crate horrorshow;
extern crate flex;

use std::io::prelude::*;
use std::io;
use flex::Flex;
use std::fs::OpenOptions;
use std::io::BufWriter;

pub struct Table {
    pub header: Vec<String>,
    pub body: String,
}

fn template() -> String {
    let template: String = format!("{}", html! {
        html {
            head {
                title: "Report";
                style: "table, th, td { border: 1px solid black; }";
            }
            body {
                h1 {
                    : "Report";
                }

                table { }
            }
        }
    });

    template
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

    let table = Table { header: header, body: "".to_owned() };

    Ok(table)
}

pub fn append(flex: &mut Vec<Flex>) -> Result<Table, io::Error> {
    let mut table: Table = parse_table(flex)?;

    table.body = format!("{}", html! {
        table {
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
                            @ if flex[j].enable {
                                td: &flex[j].value
                            }
                        }
                    }
                }
            }
        }
    });

    Ok(table)
}

pub fn generate(tables: &Vec<Table>) -> Result<(), io::Error> {
    let template: String = template();
    let mut body: String = "".to_owned();
    let file = OpenOptions::new().write(true).create(true).append(true).open("report.html")?;
    let mut buf = BufWriter::new(file);
    for table in tables {
        body.push_str(&table.body);
    }

    let template: String = template.replace("<table></table>", &body);
    buf.write(template.as_bytes()).unwrap();

    Ok(())
}