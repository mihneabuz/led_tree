use std::iter;

use colored::Colorize;

pub const TREE: &str = "\
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

#[derive(PartialEq)]
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

pub fn parse_groups() -> Vec<Vec<u8>> {
    let counts = TREE
        .lines()
        .map(|s| {
            s.chars()
                .fold(0, |acc, c| acc + (char_type(c) == Type::Led) as usize)
        })
        .collect::<Vec<_>>();

    let mut groups: Vec<Vec<u8>> = vec![Vec::new()];
    for count in counts {
        if count == 0 && groups.last().unwrap().len() > 0 {
            groups.push(Vec::new());
        } else {
            groups
                .last_mut()
                .unwrap()
                .extend(iter::repeat(0).take(count));
        }
    }

    if groups.last().unwrap().is_empty() {
        groups.pop();
    }

    groups
}

pub fn clear() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

pub fn show(groups: &Vec<Vec<u8>>) {
    clear();

    let mut color = groups.iter().flatten().chain(iter::repeat(&0));

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
                    Type::Led => {
                        match color.next() {
                            Some(0) => fmt.black(),
                            Some(1) => fmt.red(),
                            Some(2) => fmt.green(),
                            Some(3) => fmt.yellow(),
                            Some(4) => fmt.blue(),
                            Some(5) => fmt.magenta(),
                            Some(6) => fmt.cyan(),
                            Some(7) => fmt.white(),
                            _ => unreachable!()
                        }
                    }
                }
            );
        }
    }
}
