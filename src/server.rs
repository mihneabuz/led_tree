use std::io;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time;

use serde::Deserialize;
use serde_json::{from_reader, json};
use tiny_http::{Response, Server};

enum Group {
    Static(Vec<u8>),
    Animate(Vec<Vec<u8>>, usize),
}

impl Group {
    fn leds(&self) -> &Vec<u8> {
        match self {
            Group::Static(v) => v,
            Group::Animate(m, i) => &m[*i],
        }
    }
}

#[derive(Deserialize)]
struct StaticReqBody {
    SC: Vec<u8>,
}

#[derive(Deserialize)]
struct AnimateReqBody {
    MC: Vec<Vec<u8>>,
}

fn log_update(group: usize, leds: &Vec<u8>) {
    println!("update {} {:?}", group, leds);
}

fn animate_handler(groups: Arc<Mutex<Vec<Group>>>) {
    thread::spawn(move || loop {
        {
            let mut groups = groups.lock().unwrap();
            for (g, group) in groups.iter_mut().enumerate() {
                if let Group::Animate(m, i) = group {
                    *i = (*i + 1) % m.len();
                    log_update(g, &m[*i]);
                }
            }
        }

        thread::sleep(time::Duration::from_secs(1));
    });
}

// this is very spaghetti code, but it is lightweight xd
pub fn start(port: &str, groups: Vec<Vec<u8>>) -> io::Result<()> {
    for (i, group) in groups.iter().enumerate() {
        println!("update {} {:?}", i, group);
    }

    let ng = groups.len();
    let groups = Arc::new(Mutex::new(
        groups
            .into_iter()
            .map(|g| Group::Static(g))
            .collect::<Vec<_>>(),
    ));

    animate_handler(Arc::clone(&groups));

    let server = Server::http("0.0.0.0:".to_owned() + port).unwrap();

    for mut request in server.incoming_requests() {
        let mut groups = groups.lock().unwrap();

        match request.method() {
            tiny_http::Method::Get => {
                let path = request.url().split("/").skip(1).collect::<Vec<_>>();
                if path[0] == "api" && path.len() > 1 {
                    if path[1] == "group" && path.len() >= 4 && valid_group(path[2], ng) && path[3] == "leds" {
                        let g = parse_group(path[2]);
                        let res = json!({"NL": groups[g].leds().len(), "LC": groups[g].leds()})
                            .to_string();
                        request.respond(Response::from_string(res))?;
                    } else if path[1] == "groups" && path.len() == 2 {
                        let res = json!({ "NG": ng }).to_string();
                        request.respond(Response::from_string(res))?;
                    }
                } else {
                    request.respond(Response::from_string("bad request"))?;
                }
            }

            tiny_http::Method::Post => {
                let path = request.url().split("/").skip(1).collect::<Vec<_>>();

                if path[0] == "api" && path[1] == "group" && path.len() >= 4 && valid_group(path[2], ng) {
                    let g = parse_group(path[2]);

                    if path[3] == "static" {
                        if let Ok(body) = from_reader::<_, StaticReqBody>(request.as_reader()) {
                            if body.SC.len() != groups[g].leds().len() {
                                request.respond(Response::from_string("bad request"))?;
                            } else {
                                groups[g] = Group::Static(body.SC);
                                request.respond(Response::from_string("ok"))?;
                                log_update(g, &groups[g].leds());
                            }
                        }
                    } else if path[3] == "animate" {
                        if let Ok(body) = from_reader::<_, AnimateReqBody>(request.as_reader()) {
                            if body.MC.iter().any(|leds| leds.len() != groups[g].leds().len()) {
                                request.respond(Response::from_string("bad request"))?;
                            } else {
                                groups[g] = Group::Animate(body.MC, 0);
                                request.respond(Response::from_string("ok"))?;
                                log_update(g, &groups[g].leds());
                            }
                        }
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

fn valid_group(s: &str, ng: usize) -> bool {
    let group = s.parse::<usize>();
    group.is_ok() && group.unwrap() < ng
}

fn parse_group(s: &str) -> usize {
    s.parse::<usize>().unwrap()
}
