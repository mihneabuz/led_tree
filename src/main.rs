extern crate serde;
extern crate serde_json;
extern crate tiny_http;
extern crate colored;
extern crate sysinfo;

mod server;
mod tree;
mod tui;

use std::{env, io};

fn main() -> io::Result<()> {
    let mut groups = tree::parse_groups();

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && args[1] == "server" {
        let port = &args[2];
        server::start(port, &mut groups)?;
    } else {
        tui::start(groups.len())?;
    }

    Ok(())
}
