extern crate colored;
extern crate serde;
extern crate serde_json;

use std::{env, io};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && args[1] == "server" {
    } else {
    }

    Ok(())
}
