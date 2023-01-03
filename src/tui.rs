use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::process;
use std::thread::sleep;
use std::time::Duration;

use crate::tree::{CursesUI, UI};

fn get_pid() -> io::Result<u32> {
    let cmd = process::Command::new("pidof").arg("christmas-tree").output()?;
    for s in String::from_utf8(cmd.stdout).unwrap().trim().split(" ") {
        if let Ok(pid) = s.parse::<u32>() {
            if pid != process::id() {
                return Ok(pid);
            }
        }
    }

    Err(io::Error::new(
        io::ErrorKind::Other,
        "Server process not found",
    ))
}

pub fn handle_update(msg: &str, groups: &mut Vec<Vec<u8>>) -> bool {
    if msg.starts_with("update") {
        let (i, g) = msg[7..].split_once(" ").unwrap();
        groups[i.parse::<usize>().unwrap()] = serde_json::from_str::<Vec<u8>>(g).unwrap();

        true
    } else {
        false
    }
}

pub fn start(ng: usize) -> io::Result<()> {
    let server_out = File::open(format!("/proc/{}/fd/1", get_pid()?))?;
    let mut reader = BufReader::new(server_out);

    let mut groups = vec![Vec::new(); ng];
    let mut buf = String::new();
    while let Ok(l) = reader.read_line(&mut buf) {
        if l == 0 {
            break;
        }

        handle_update(&buf, &mut groups);
        buf.clear();
    }

    let mut tree = CursesUI::new();
    tree.show(&groups);

    let mut update = false;
    while let Ok(l) = reader.read_line(&mut buf) {
        if l != 0 {
            if handle_update(&buf, &mut groups) {
                update = true;
            }

            buf.clear();
        } else {
            if update {
                tree.show(&groups);
                update = false;
            }

            sleep(Duration::from_millis(100));
        }
    }

    Ok(())
}
