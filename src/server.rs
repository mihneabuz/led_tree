use std::io;

use serde::Deserialize;
use serde_json::{json, from_reader};
use tiny_http::{Response, Server};

#[derive(Deserialize)]
struct ReqBody {
    SC: Vec<u8>
}

// this is very spaghetti code, but it is lightweight xd
pub fn start(port: &str, groups: &mut Vec<Vec<u8>>) -> io::Result<()> {
    let ng = groups.len();

    for (i, group) in groups.iter().enumerate() {
        println!("update {} {:?}", i, group);
    }

    let server = Server::http("0.0.0.0:".to_owned() + port).unwrap();

    for mut request in server.incoming_requests() {
        match request.method() {
            tiny_http::Method::Get => {
                let path = request.url().split("/").skip(1).collect::<Vec<_>>();

                if path[0] == "api" && (path[1] == "groups" || path[1] == "group") {
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
                let path = request.url().split("/").skip(1).collect::<Vec<_>>();

                if path[0] == "api" && path[1] == "group" {
                    if path.len() >= 4 && path[3] == "static" {
                        let group = path[2].parse::<usize>();
                        if group.is_err() || group.clone().unwrap() >= ng {
                            request.respond(Response::from_string("bad request"))?;
                        } else {
                            let g = group.unwrap();
                            if let Ok(body) = from_reader::<_, ReqBody>(request.as_reader()) {
                                if body.SC.len() != groups[g].len() {
                                    request.respond(Response::from_string("bad request"))?;
                                } else {
                                    for i in 0..groups[g].len() {
                                        groups[g][i] = body.SC[i];
                                    }
                                    request.respond(Response::from_string("ok"))?;
                                    println!("update {} {:?}", g, &groups[g]);
                                }
                            } else {
                                request.respond(Response::from_string("bad request"))?;
                            }
                        }
                    } else {
                        request.respond(Response::from_string("bad request"))?;
                    }
                } else {
                    request.respond(Response::from_string("bad request"))?;
                }
            }

            _ => {
                request.respond(Response::from_string("bad method"))?;
            }
        }
    }

    Ok(())
}
