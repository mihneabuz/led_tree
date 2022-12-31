extern crate colored;
extern crate serde_json;
extern crate tiny_http;

use std::{env, io};

use colored::Colorize;
use serde_json::json;
use tiny_http::{Response, Server};

const TREE: &str = "\
|-----------------------------------------------|\n\
|                                               |\n\
|                                               |\n\
|                       O                       |\n\
|                      OOO                      |\n\
|                      ~O~                      |\n\
|                     ~~~~~                     |\n\
|                    O~~~~~~                    |\n\
|                   ~~OO~~OO~                   |\n\
|                  ~~~~~OO~~~~                  |\n\
|                 ~~~~~~~~~~~~~                 |\n\
|                OO~~~~~~~~~~~~~                |\n\
|               ~~~O~~~~~><~~~~~O               |\n\
|              ~~~~~O~~~~~~~~~~O~~              |\n\
|             ~~~~~~~OO~~~~~~~O~~~~             |\n\
|            ~~~~><~~~~OO~~~OO~~~~~~            |\n\
|           ~~~~~~~~~~~~~OOO~~~~~~~~~           |\n\
|          ~~~~~~~~~~~~~~~~~~~~~~~~~~~          |\n\
|         ~~~~~~~~~~~~~~~~~~~~~~~~~~OO~         |\n\
|        O~~~~><~~~~~~~~~><~~~~~~~OO~~~~        |\n\
|       ~~OO~~~~~~~~~~~~~~~~~~~~OO~~~~~~~       |\n\
|      ~~~~~OO~~~~~~~~~~~~~~~OOO~~~~~~~~~~      |\n\
|     ~~~~~~~~OO~~~~~~~~~~OOO~~~~~~~~~~~~~~     |\n\
|    ~~~><~~~~~~OOO~~~~OOO~~~~~~~~~~~~~~~~~~    |\n\
|   ~~~~~~~~~~~~~~~OOOO~~~~~~~~~~><~~~~~~~~~~   |\n\
|  ~~~~~~~~~~~~~~OO~~~OOO~~~~~~~~~~~~~~~~~~~~~  |\n\
|                     #####                     |\n\
|                     #####                     |\n\
|                     #####                     |\n\
|                                               |\n\
|-----------------------------------------------|\n\
";

enum Type {
    Frame,
    Tree,
    Decoration,
    Led,
}

fn char_type(c: char) -> Type {
    match c {
        '|' | '-' | ' ' => Type::Frame,
        '~' | '#' => Type::Tree,
        '>' | '<' => Type::Decoration,
        'O' => Type::Led,
        _ => unreachable!(),
    }
}

fn show_tree() {
    for c in TREE.chars() {
        if c == '\n' {
            println!();
        } else {
            let fmt = format!("{}", c);
            print!(
                "{}",
                match char_type(c) {
                    Type::Frame => fmt.bold(),
                    Type::Tree => fmt.green(),
                    Type::Decoration => fmt.red(),
                    Type::Led => fmt.blue(),
                }
            );
        }
    }
}

fn main() -> io::Result<()> {
    let ng = 3;
    let mut groups = vec![vec![0, 0, 0]; ng];

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && args[1] == "server" {
        let port = &args[2];
        let server = Server::http("0.0.0.0:".to_owned() + port).unwrap();

        for request in server.incoming_requests() {
            match request.method() {
                tiny_http::Method::Get => {
                    let path = request.url().split("/").skip(1).collect::<Vec<_>>();

                    if path[0] == "api" && path[1] == "groups" {
                        if path.len() >= 4 && path[3] == "leds" {
                            let group = path[2].parse::<usize>();
                            if group.is_err() || group.clone().unwrap() >= ng {
                                request.respond(Response::from_string("bad request"))?;
                            } else {
                                let g = group.unwrap();
                                let res = json!({"NL": groups[g].len(), "LC": groups[g]}).to_string();
                                request.respond(Response::from_string(res))?;
                            }
                        } else {
                            let res = json!({"NG": ng}).to_string();
                            request.respond(Response::from_string(res))?;
                        }
                    } else {
                        request.respond(Response::from_string("bad request"))?;
                    }
                }

                tiny_http::Method::Post => {
                    request.respond(Response::from_string("post"))?;
                }

                _ => {
                    request.respond(Response::from_string("bad method"))?;
                }
            }
        }
    } else {
        show_tree();
    }

    Ok(())
}
