use std::fs::File;
use std::io::{self, BufReader, BufRead, Seek};
use std::process;

use crate::tree;

use serde::Deserialize;

#[derive(Deserialize)]
struct ReqBody {
    LC: Vec<u8>,
    NL: usize,
}

pub fn start(ng: usize) -> io::Result<()> {
    return tui(1000, ng);

    Err(io::Error::new(
        io::ErrorKind::Other,
        "Server process not found",
    ))
}

fn fetch_group(g: usize) -> Vec<u8> {
    vec![]
}

fn tui(server_pid: u32, ng: usize) -> io::Result<()> {
    let server_out = File::open(format!("/proc/{}/fd/1", server_pid)).unwrap();
    let mut reader = BufReader::new(server_out);

    reader.seek(io::SeekFrom::End(0))?;

    let mut groups = (0..ng)
        .into_iter()
        .map(|g| fetch_group(g))
        .collect::<Vec<_>>();

    tree::show(&groups);

    let mut buf = String::new();
    while let Ok(l) = reader.read_line(&mut buf) {
        if l != 0 {
            if buf.starts_with("update") {
                let g = buf.trim().split_once(" ").unwrap().1.parse::<usize>().unwrap();
                groups[g] = fetch_group(g);

                tree::show(&groups);
            }

            buf.clear();
        }
    }

    Ok(())
}
